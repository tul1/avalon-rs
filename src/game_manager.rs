// use std::collections::HashMap;

use crate::datamodel::game::Game;

pub struct GameManager {
    game: game::Game,
    // status: game::GameStatus,
    // turn_manage: turn_manager::TurnManager,
    // action_manager: HashMap<player::Player,player_action_manager::PlayerActionManager >
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            game: game::Game{}
        }
    }
    
    // pub fn next_turn(&self) -> GameStatus {
    //     //unimplemented
    //     GameStatus {}
    // }
    
    // pub fn create_quest_proposal(&self, Vec<quest_manager::QuestManager>) {
    //     //unimplemented
    // }

    // pub fn count_proposal_votes(&self) {
    //     //unimplemented
    // }

    // pub fn count_quest_votes(&self) {
    //     //unimplemented
    // }

}
