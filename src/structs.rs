use serde::{Serialize, Deserialize};


// ws -> process_connection
#[derive(Serialize, Deserialize, Debug)]
pub struct NameReq {
    pub typ: String,
    pub name: String,
}

// ws -> process_connection
#[derive(Serialize, Deserialize, Debug)]
pub struct MsgReq {
    pub typ: String,
    pub time: i32,
    pub text: String,
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

#[derive(Debug, Clone)]
pub enum ChangeType {
    Join,
    Leave,
}

// process_connection -> user_mpsc -> user_manager
#[derive(Debug, Clone)]
pub struct UserChange {
    pub change_type: ChangeType,
    pub name: String,
}
