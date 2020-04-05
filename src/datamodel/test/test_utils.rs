use crate::datamodel::characters::{Merlin, Mordred, Percival};
use crate::datamodel::Player;
use std::rc::Rc;

pub fn create_3_players() -> Vec<Rc<Player>> {
    vec![
        Rc::new(Player {
            name: "jimi".to_string(),
            character: Rc::new(Merlin {}),
        }),
        Rc::new(Player {
            name: "volan".to_string(),
            character: Rc::new(Percival {}),
        }),
        Rc::new(Player {
            name: "pato".to_string(),
            character: Rc::new(Mordred {}),
        }),
    ]
}
