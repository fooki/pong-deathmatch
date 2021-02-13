use crate::pong_state::PongState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerOrder {
    P1,
    P2,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMsg {

    // These are communicated FROM the server TO the client.

    Hi,

    Ping,
    Timeout,
    Disconnect,
    Connect,

    Start(PlayerOrder),
    State(PongState),
    Abort,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMsg {

    // These are communicated FROM the client TO the server.

    Hi,

    Pong,
    Timeout,
    Connect,
    Disconnect,

    MoveUp,
    MoveDown,
}
