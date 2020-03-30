use crate::datamodel::character::character::{Character, CharacterName, Team};

#[derive(PartialEq, Debug)]
pub struct Oberon {}

impl Oberon {
    pub fn new() -> Self {
        Oberon {}
    }
}

impl Character for Oberon {
    fn get_team(&self) -> Team {
        Team::Evil
    }

    fn get_name(&self) -> CharacterName {
        CharacterName::Oberon
    }

    fn is_seen_by(&self, other: &CharacterName) -> bool {
        match *other {
            CharacterName::Merlin => true,
            _ => false,
        }
    }

    fn can_see(&self, _other: &CharacterName) -> bool {
        false
    }
}
