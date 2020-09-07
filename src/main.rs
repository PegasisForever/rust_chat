mod structs;

use std::{env, io::Error};

use log::info;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Sender, Receiver};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};
use futures_util::core_reexport::result::Result::Ok;
use serde_json::{Value};
use std::collections::HashSet;
use std::iter::FromIterator;
use crate::structs::{UserChange, ChangeType, OnlineUsersBoardCast, NameReq, MsgReq, MsgBoardCast};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Listening");

    let (ws_tx, _) = broadcast::channel(32);
    let (user_tx, user_rx) = mpsc::channel(32);

    tokio::select! {
        _ = user_manager(ws_tx.clone(), user_rx) => (),
        _ = async move {
            while let Ok((stream, _)) = listener.accept().await {
                let ws_tx = ws_tx.clone();
                let ws_rx = ws_tx.subscribe();
                tokio::spawn(process_connection(stream, ws_tx, ws_rx, user_tx.clone()));
            }
        } => (),
         _ = signal::ctrl_c() => (),
    }

    info!("stopped");
    Ok(())
}

async fn user_manager(ws_tx: Sender<String>, mut user_rx: mpsc::Receiver<UserChange>) {
    let mut users = HashSet::new();
    while let Some(change) = user_rx.recv().await {
        match change.change_type {
            ChangeType::Join => users.insert(change.name),
            ChangeType::Leave => users.remove(&change.name),
        };

        let bc = OnlineUsersBoardCast {
            typ: "users".to_string(),
            users: Vec::from_iter(users.clone().into_iter()),
        };
        ws_tx.send(serde_json::to_string(&bc).unwrap()).unwrap();
    }
}

async fn process_connection(stream: TcpStream, ws_tx: Sender<String>, mut ws_rx: Receiver<String>, mut user_tx: mpsc::Sender<UserChange>) {
    info!("New WebSocket connection");
    let (mut write, mut read) =
        tokio_tungstenite::accept_async(stream).await.unwrap().split();

    let receive_task = async move {
        let mut name: Option<String> = None;
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(json_text)) => {
                    if let Ok(json) = serde_json::from_str::<Value>(&json_text) {
                        if json["typ"] == "name" && name == None {
                            if let Ok(req) = serde_json::from_value::<NameReq>(json.clone()) {
                                name = Some(req.name);
                                info!("name set to {:?}", name.as_ref().unwrap());
                                let user_change = UserChange {
                                    change_type: ChangeType::Join,
                                    name: String::from(name.as_ref().unwrap()),
                                };
                                user_tx.send(user_change).await.unwrap();
                            }
                        } else if json["typ"] == "msg" {
                            if let Ok(req) = serde_json::from_value::<MsgReq>(json.clone()) {
                                let bc = MsgBoardCast {
                                    typ: "msg".to_string(),
                                    time: req.time,
                                    name: String::from(name.as_ref().unwrap()),
                                    text: req.text,
                                };
                                ws_tx.send(serde_json::to_string(&bc).unwrap()).unwrap();
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

        let user_change = UserChange {
            change_type: ChangeType::Leave,
            name: String::from(name.as_ref().unwrap()),
        };
        user_tx.send(user_change).await.unwrap();
    };

    let send_task = async move {
        while let Ok(msg) = ws_rx.recv().await {
            write.send(Message::Text(msg)).await.unwrap();
        }
    };

    tokio::select! {
        _ = receive_task => (),
        _ = send_task => (),
    }


    info!("WebSocket closed");
}
