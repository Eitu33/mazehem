use crate::cell::Cell;
use crate::input::SerKey;
use crate::player::Player;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Data {
    Cell(Cell),
    Players(Vec<Player>),
    Key(SerKey),
}
