use crate::datamodel::character::character::{Character, CharacterName, Team};

#[derive(PartialEq, Debug)]
pub struct Percival {}

impl Percival {
    pub fn new() -> Self {
        Percival {}
    }
}

impl Character for Percival {
    fn get_team(&self) -> Team {
        Team::Good
    }

    fn get_name(&self) -> CharacterName {
        CharacterName::Percival
    }

    fn is_seen_by(&self, _other: &CharacterName) -> bool {
        false
    }

    fn can_see(&self, other: &CharacterName) -> bool {
        match *other {
            CharacterName::Merlin | CharacterName::Morgana => true,
            _ => false,
        }
    }
}
