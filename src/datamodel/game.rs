use crate::datamodel::board::Board;
use crate::datamodel::player::Player;

pub struct Game {
    pub num_players: u32,
    pub players: Vec<Player>,
    pub board: Board,
}
