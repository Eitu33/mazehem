use crate::cell::Cell;
use crate::input::SerKey;
use crate::player::Player;
use serde_derive::{Deserialize, Serialize};
use rsa::RsaPrivateKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Connection,
    PrivateKey(RsaPrivateKey),
    Handshake(Vec<u8>),
    Cells(Vec<Cell>),
    Players(Vec<Player>),
    Key(SerKey),
}
