use crate::datamodel::character::character::{Character, CharacterName, Team};

#[derive(PartialEq, Debug)]
pub struct Merlin {}

impl Merlin {
    pub fn new() -> Self {
        Merlin {}
    }
}

impl Character for Merlin {
    fn get_team(&self) -> Team {
        Team::Good
    }

    fn get_name(&self) -> CharacterName {
        CharacterName::Merlin
    }

    fn is_seen_by(&self, other: &CharacterName) -> bool {
        match *other {
            CharacterName::Percival {} => true,
            _ => false,
        }
    }

    fn can_see(&self, other: &CharacterName) -> bool {
        match *other {
            CharacterName::MinionOfMordred
            | CharacterName::Assassin
            | CharacterName::Morgana
            | CharacterName::Oberon => true,
            _ => false,
        }
    }
}
