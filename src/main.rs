mod structs;

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


type UsersMap = Arc<Mutex<HashMap<String, Sender<String>>>>;
type MessageList = Arc<Mutex<Vec<MsgBoardCast>>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Listening");

    let users_map: UsersMap = Arc::new(Mutex::new(HashMap::new()));
    let message_list: MessageList = Arc::new(Mutex::new(Vec::new()));

    tokio::select! {
        _ = async move {
            while let Ok((stream, _)) = listener.accept().await {
                tokio::spawn(process_connection(stream, users_map.clone(),message_list.clone()));
            }
        } => (),
         _ = signal::ctrl_c() => (),
    }

    info!("stopped");
    Ok(())
}

fn user_joined(name: &Option<String>, users_map: &UsersMap, ws_tx: &Sender<String>) {
    users_map.lock().unwrap().insert(String::from(name.as_ref().unwrap()), ws_tx.clone());
    send_user_list_bc(users_map);
}

fn user_left(name: &Option<String>, users_map: &UsersMap) {
    users_map.lock().unwrap().remove(name.as_ref().unwrap());
    send_user_list_bc(users_map);
}

fn send_user_list_bc(users_map: &UsersMap) {
    let mut users_map = users_map.lock().unwrap();
    let user_list = users_map.iter()
        .map(|(name, _)| {
            String::from(name)
        })
        .collect::<Vec<String>>();
    let bc = OnlineUsersBoardCast {
        typ: "users".to_string(),
        users: user_list,
    };
    let text = serde_json::to_string(&bc).unwrap();
    for (_, tx) in users_map.iter_mut() {
        tx.try_send(text.clone()).unwrap();
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

                                let res = json!({
                                    "typ": "state",
                                    "messages": &*message_list.lock().unwrap()
                                });
                                ws_tx.send(res.to_string()).await.unwrap();

                                user_joined(&name, &users_map, &ws_tx);
                            }
                        } else if json["typ"] == "msg" {
                            if let Ok(req) = serde_json::from_value::<MsgReq>(json.clone()) {
                                let bc = MsgBoardCast {
                                    typ: "msg".to_string(),
                                    time: req.time,
                                    name: String::from(name.as_ref().unwrap()),
                                    text: req.text,
                                };

                                let text = serde_json::to_string(&bc).unwrap();
                                message_list.lock().unwrap().push(bc);

                                let mut users_map = users_map.lock().unwrap();
                                for (_, tx) in users_map.iter_mut() {
                                    tx.try_send(text.clone()).unwrap();
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
