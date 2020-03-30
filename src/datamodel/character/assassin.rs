use crate::datamodel::character::character::{Character, CharacterName, Team};

#[derive(PartialEq, Debug)]
pub struct Assassin {}

impl Assassin {
    pub fn new() -> Self {
        Assassin {}
    }
}

impl Character for Assassin {
    fn get_team(&self) -> Team {
        Team::Evil
    }

    fn get_name(&self) -> CharacterName {
        CharacterName::Assassin
    }

    fn is_seen_by(&self, other: &CharacterName) -> bool {
        match *other {
            CharacterName::Merlin
            | CharacterName::Assassin
            | CharacterName::MinionOfMordred
            | CharacterName::Mordred
            | CharacterName::Morgana => true,
            _ => false,
        }
    }

    fn can_see(&self, other: &CharacterName) -> bool {
        match *other {
            CharacterName::MinionOfMordred | CharacterName::Mordred | CharacterName::Morgana => {
                true
            }
            _ => false,
        }
    }
}
