use serde::{Serialize, Deserialize};
use uuid::Uuid;

// ws -> process_connection
#[derive(Serialize, Deserialize, Debug)]
pub struct NameReq {
    pub typ: String,
    pub name: String,
    pub id: Uuid,
}

// ws -> process_connection
#[derive(Serialize, Deserialize, Debug)]
pub struct MsgReq {
    pub typ: String,
    pub time: i32,
    pub text: String,
    pub id: Uuid,
}

// process_connection -> ws
#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentStateRes {
    pub messages: Vec<MsgBoardCast>,
}

// process_connection -> ws_board_cast -> process_connection
#[derive(Serialize, Deserialize, Debug)]
pub struct MsgBoardCast {
    pub typ: String,
    pub time: i32,
    pub name: String,
    pub text: String,
}

// user_manager -> ws_board_cast -> process_connection
#[derive(Serialize, Deserialize, Debug)]
pub struct OnlineUsersBoardCast {
    pub typ: String,
    pub users: Vec<String>,
}
