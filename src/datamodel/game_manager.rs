use crate::datamodel::game_status::GameStatus;
use crate::datamodel::player_action_manager::PlayerActionManager;
use crate::datamodel::turn_manager::TurnManager;
use crate::datamodel::Game;
use std::collections::HashMap;
use std::rc::Rc;

pub struct GameManager<'a> {
    pub game: Game,
    pub status: GameStatus<'a>,
    pub turn_manager: Rc<TurnManager>,
    pub action_managers: HashMap<String, PlayerActionManager>,
}

impl GameManager<'_> {
    pub fn next_turn(&self) {
        unimplemented!()
    }

    pub fn create_quest_proposal(&self) {
        unimplemented!()
    }

    pub fn count_proposal_votes(&self) {
        unimplemented!()
    }

    pub fn count_quest_votes(&self) {
        unimplemented!()
    }
}
