use crate::datamodel::character::character::{Character, CharacterName, Team};

#[derive(PartialEq, Debug)]
pub struct Mordred {}

impl Mordred {
    pub fn new() -> Self {
        Mordred {}
    }
}

impl Character for Mordred {
    fn get_team(&self) -> Team {
        Team::Evil
    }

    fn get_name(&self) -> CharacterName {
        CharacterName::Mordred
    }

    fn is_seen_by(&self, other: &CharacterName) -> bool {
        match *other {
            CharacterName::Assassin | CharacterName::MinionOfMordred | CharacterName::Morgana => {
                true
            }
            _ => false,
        }
    }

    fn can_see(&self, other: &CharacterName) -> bool {
        match *other {
            CharacterName::MinionOfMordred | CharacterName::Assassin | CharacterName::Morgana => {
                true
            }
            _ => false,
        }
    }
}
