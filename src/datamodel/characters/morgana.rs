use crate::datamodel::characters::character::{Character, CharacterName, Team};

#[derive(PartialEq, Debug, Default)]
pub struct Morgana {}

impl Morgana {
    pub fn new() -> Self {
        Morgana {}
    }
}

impl Character for Morgana {
    fn get_team(&self) -> Team {
        Team::Evil
    }

    fn get_name(&self) -> CharacterName {
        CharacterName::Morgana
    }

    fn is_seen_by(&self, other: &CharacterName) -> bool {
        match *other {
            CharacterName::Percival
            | CharacterName::Merlin
            | CharacterName::Assassin
            | CharacterName::MinionOfMordred
            | CharacterName::Mordred => true,
            _ => false,
        }
    }

    fn can_see(&self, other: &CharacterName) -> bool {
        match *other {
            CharacterName::MinionOfMordred | CharacterName::Assassin | CharacterName::Mordred => {
                true
            }
            _ => false,
        }
    }
}
