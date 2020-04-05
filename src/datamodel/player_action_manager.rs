use crate::datamodel::characters::CharacterName;
use crate::datamodel::turn_manager::TurnManager;
use crate::datamodel::turns::turn::TurnRole;
use crate::datamodel::turns::Turn;
use crate::datamodel::Player;
use std::collections::HashMap;
use std::rc::Rc;

pub struct PlayerActionManager {
    player: Rc<Player>,
    turn_manager: Rc<TurnManager>,
}

impl PlayerActionManager {
    pub fn new(player: Rc<Player>, turn_manager: Rc<TurnManager>) -> PlayerActionManager {
        PlayerActionManager {
            player,
            turn_manager,
        }
    }

    pub fn get_turn_roles(&self) -> Vec<TurnRole> {
        unimplemented!()
    }

    pub fn see_knowable_roles(&self) -> HashMap<String, CharacterName> {
        unimplemented!()
    }

    pub fn see_current_turn(&self) -> &Turn {
        unimplemented!()
    }
}
