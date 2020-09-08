use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct NameReq {
    pub typ: String,
    pub name: String,
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgReq {
    pub typ: String,
    pub time: i64,
    pub text: String,
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChessReq {
    pub typ: String,
    pub time: i64,
    pub chess: Vec<Option<bool>>,
    pub id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgBoardCast {
    pub typ: String,
    pub time: i64,
    pub name: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChessBoardCast {
    pub typ: String,
    pub time: i64,
    pub chess: Vec<Option<bool>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnlineUsersBoardCast {
    pub typ: String,
    pub users: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkBoardCast {
    pub typ: String,
    pub available: bool,
}
