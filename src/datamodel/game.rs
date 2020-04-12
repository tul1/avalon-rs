use crate::datamodel::board::Board;
use crate::datamodel::player::Player;
use std::rc::Rc;

pub struct Game {
    pub num_players: u32,
    pub players: Vec<Rc<Player>>,
    pub board: Board,
}
