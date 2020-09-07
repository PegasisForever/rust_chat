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
use crate::structs::{OnlineUsersBoardCast, NameReq, MsgReq, MsgBoardCast, CurrentStateRes};
use tokio::signal;
use std::sync::{Mutex, Arc};
use serde_json::json;
use crate::tools::ensure_file_exists;
use tokio::fs;


type UsersMap = Arc<Mutex<HashMap<String, Sender<String>>>>;
type MessageList = Arc<Mutex<Vec<MsgBoardCast>>>;

const MESSAGE_PATH: &str = "chat_data/messages.json";

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
        let json: Value = serde_json::from_str(&text).unwrap();
        let list = json.as_array().unwrap().iter()
            .map(|msg_bc_json| {
                serde_json::from_value::<MsgBoardCast>(msg_bc_json.clone()).unwrap()
            })
            .collect();
        Arc::new(Mutex::new(list))
    };
    let message_list2 = message_list.clone();

    tokio::select! {
        _ = async move {
            while let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(process_connection(stream, users_map.clone(), message_list.clone()));
            }
        } => (),
         _ = signal::ctrl_c() => (),
    }

    {
        let text = json!(&*message_list2.lock().unwrap()).to_string();
        fs::write(MESSAGE_PATH, text).await.unwrap();
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
    let mut users_map = users_map.lock().unwrap();
    users_map.iter()
        .map(|(name, _)| {
            String::from(name)
        })
        .collect::<Vec<String>>()
}

fn send_user_list_bc(users_map: &UsersMap, excluded_user: &str) {
    let user_list = get_user_list_json(users_map);
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

async fn process_connection(stream: TcpStream, users_map: UsersMap, message_list: MessageList) {
    info!("New WebSocket connection");
    let (mut write, mut read) =
        tokio_tungstenite::accept_async(stream).await.unwrap().split();
    let (mut ws_tx, mut ws_rx) = mpsc::channel::<String>(32);

    let receive_task = async move {
        let mut name: Option<String> = None;
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(json_text)) => {
                    if let Ok(json) = serde_json::from_str::<Value>(&json_text) {
                        if json["typ"] == "name" && name == None {
                            if let Ok(req) = serde_json::from_value::<NameReq>(json.clone()) {
                                name = Some(req.name);

                                user_joined(&name, &users_map, &ws_tx);

                                let res = json!({
                                    "typ": "state",
                                    "messages": &*message_list.lock().unwrap(),
                                    "users": get_user_list_json(&users_map),
                                    "id": req.id,
                                });
                                ws_tx.send(res.to_string()).await.unwrap();
                            }
                        } else if json["typ"] == "msg" {
                            if let Ok(req) = serde_json::from_value::<MsgReq>(json.clone()) {
                                let bc = MsgBoardCast {
                                    typ: "msg".to_string(),
                                    time: req.time,
                                    name: String::from(name.as_ref().unwrap()),
                                    text: req.text,
                                };

                                let mut value = serde_json::to_value(&bc).unwrap();
                                let others_text = value.to_string();
                                value["id"] = serde_json::to_value(&req.id).unwrap();
                                let res_test = value.to_string();

                                message_list.lock().unwrap().push(bc);

                                let mut users_map = users_map.lock().unwrap();
                                let my_name = name.as_ref().unwrap();
                                for (name, tx) in users_map.iter_mut() {
                                    if name != my_name {
                                        tx.try_send(others_text.clone()).unwrap();
                                    } else {
                                        tx.try_send(res_test.clone()).unwrap();
                                    }
                                }
                            }
                        } else if json["type"] == "chess" {
                            unimplemented!();
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
