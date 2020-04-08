use crate::datamodel::character::{Merlin, Mordred, Percival};
use crate::datamodel::Player;

pub fn create_3_players() -> Vec<Player> {
    let jimi = Player {
        name: "jimi".to_string(),
        character: Box::new(Merlin {}),
    };
    let pato = Player {
        name: "pato".to_string(),
        character: Box::new(Mordred {}),
    };
    let volan = Player {
        name: "volan".to_string(),
        character: Box::new(Percival {}),
    };
    vec![jimi, pato, volan]
}
