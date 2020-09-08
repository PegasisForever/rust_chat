mod structs;
mod tools;

use std::{env, io::Error};

use log::info;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use futures_util::core_reexport::result::Result::Ok;
use serde_json::{Value};
use std::collections::HashMap;
use crate::structs::{OnlineUsersBoardCast, NameReq, MsgReq, MsgBoardCast, ChessReq, ChessBoardCast};
use tokio::signal;
use std::sync::{Mutex, Arc};
use serde_json::json;
use crate::tools::ensure_file_exists;
use tokio::fs;
use uuid::Uuid;


type UsersMap = Arc<Mutex<HashMap<String, Sender<String>>>>;
type MessageList = Arc<Mutex<Vec<MsgBoardCast>>>;
type ChessData = Arc<Mutex<Vec<Option<bool>>>>;

const MESSAGE_PATH: &str = "chat_data/messages.json";
const CHESS_PATH: &str = "chat_data/chess.json";

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Listening");

    let users_map: UsersMap = Arc::new(Mutex::new(HashMap::new()));

    let message_list: MessageList = {
        ensure_file_exists(MESSAGE_PATH, "[]").await.unwrap();
        let text = fs::read_to_string(MESSAGE_PATH).await.unwrap();
        let list: Vec<MsgBoardCast> = serde_json::from_str(&text).unwrap();

        Arc::new(Mutex::new(list))
    };
    let message_list2 = message_list.clone();

    let chess_data: ChessData = {
        ensure_file_exists(CHESS_PATH, &*json!(vec![Option::<bool>::None;15*15]).to_string()).await.unwrap();
        let text = fs::read_to_string(CHESS_PATH).await.unwrap();
        let list: Vec<Option<bool>> = serde_json::from_str(&text).unwrap();

        Arc::new(Mutex::new(list))
    };
    let chess_data2 = chess_data.clone();

    tokio::select! {
        _ = async move {
            while let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(process_connection(stream, users_map.clone(), message_list.clone(), chess_data.clone()));
            }
        } => (),
         _ = signal::ctrl_c() => (),
    }

    {
        let text = json!(&*message_list2.lock().unwrap()).to_string();
        fs::write(MESSAGE_PATH, text).await.unwrap();

        let text = json!(&*chess_data2.lock().unwrap()).to_string();
        fs::write(CHESS_PATH, text).await.unwrap();
    }


    info!("stopped");
    Ok(())
}

fn user_joined(name: &Option<String>, users_map: &UsersMap, ws_tx: &Sender<String>) {
    let name_ref = name.as_ref().unwrap();
    users_map.lock().unwrap().insert(String::from(name_ref), ws_tx.clone());
    send_user_list_bc(users_map, name_ref);
}

fn user_left(name: &Option<String>, users_map: &UsersMap) {
    let name_ref = name.as_ref().unwrap();
    users_map.lock().unwrap().remove(name_ref);
    send_user_list_bc(users_map, name_ref);
}

fn get_user_list_json(users_map: &UsersMap) -> Vec<String> {
    let users_map = users_map.lock().unwrap();
    users_map.iter()
        .map(|(name, _)| {
            String::from(name)
        })
        .collect::<Vec<String>>()
}

fn send_user_list_bc(users_map: &UsersMap, excluded_user: &str) {
    let mut user_list = get_user_list_json(users_map);
    user_list.sort_unstable();
    let bc = OnlineUsersBoardCast {
        typ: "users".to_string(),
        users: user_list,
    };

    let text = serde_json::to_string(&bc).unwrap();
    let mut users_map = users_map.lock().unwrap();
    for (name, tx) in users_map.iter_mut() {
        if name != excluded_user {
            tx.try_send(text.clone()).unwrap();
        }
    }
}

fn reply_and_board_cast(mut json: Value, users_map: &UsersMap, reply_user: &str, reply_id: &Uuid) {
    let others_text = json.to_string();
    json["id"] = serde_json::to_value(reply_id).unwrap();
    let res_text = json.to_string();

    let mut users_map = users_map.lock().unwrap();
    for (name, tx) in users_map.iter_mut() {
        if name != reply_user {
            tx.try_send(others_text.clone()).unwrap();
        } else {
            tx.try_send(res_text.clone()).unwrap();
        }
    }
}

async fn process_connection(stream: TcpStream, users_map: UsersMap, message_list: MessageList, chess_data: ChessData) {
    let ws = tokio_tungstenite::accept_async(stream).await;
    if ws.is_err() {
        return;
    }

    info!("New WebSocket connection");
    let (mut write, mut read) =
        ws.unwrap().split();
    let (mut ws_tx, mut ws_rx) = mpsc::channel::<String>(32);

    let receive_task = async move {
        let mut name: Option<String> = None;
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(json_text)) => {
                    if let Ok(json) = serde_json::from_str::<Value>(&json_text) {
                        if json["typ"] == "name" && name.is_none() {
                            if let Ok(req) = serde_json::from_value::<NameReq>(json.clone()) {
                                name = Some(req.name);

                                user_joined(&name, &users_map, &ws_tx);

                                let res = json!({
                                    "typ": "state",
                                    "messages": &*message_list.lock().unwrap(),
                                    "chess": &*chess_data.lock().unwrap(),
                                    "users": get_user_list_json(&users_map),
                                    "id": req.id,
                                });
                                ws_tx.send(res.to_string()).await.unwrap();
                            }
                        } else if json["typ"] == "msg" && name.is_some() {
                            if let Ok(req) = serde_json::from_value::<MsgReq>(json.clone()) {
                                let bc = MsgBoardCast {
                                    typ: "msg".to_string(),
                                    time: req.time,
                                    name: String::from(name.as_ref().unwrap()),
                                    text: req.text,
                                };

                                let json = serde_json::to_value(&bc).unwrap();
                                reply_and_board_cast(json, &users_map, name.as_ref().unwrap(), &req.id);
                                let mut message_list = message_list.lock().unwrap();
                                message_list.push(bc);
                                message_list.sort_unstable_by_key(|item| item.time)
                            }
                        } else if json["typ"] == "chess" && name.is_some() {
                            if let Ok(req) = serde_json::from_value::<ChessReq>(json.clone()) {
                                let bc = ChessBoardCast {
                                    typ: "chess".to_string(),
                                    time: req.time,
                                    chess: req.chess,
                                };
                                let json = serde_json::to_value(&bc).unwrap();
                                reply_and_board_cast(json, &users_map, name.as_ref().unwrap(), &req.id);
                                *(chess_data.lock().unwrap()) = bc.chess
                            }
                        }
                    }
                }
                Ok(Message::Close(None)) => { break; }
                _ => {}
            }
        }

        user_left(&name, &users_map);
    };

    let send_task = async move {
        while let Some(msg) = ws_rx.recv().await {
            write.send(Message::Text(msg)).await.unwrap();
        }
    };

    tokio::select! {
        _ = receive_task => (),
        _ = send_task => (),
    }


    info!("WebSocket closed");
}
