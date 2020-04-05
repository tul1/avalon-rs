use crate::datamodel::characters::character::{Character, CharacterName, Team};

#[derive(PartialEq, Debug, Default)]
pub struct MinionOfMordred {}

impl MinionOfMordred {
    pub fn new() -> Self {
        MinionOfMordred {}
    }
}

impl Character for MinionOfMordred {
    fn get_team(&self) -> Team {
        Team::Evil
    }

    fn get_name(&self) -> CharacterName {
        CharacterName::MinionOfMordred
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
            CharacterName::MinionOfMordred
            | CharacterName::Assassin
            | CharacterName::Mordred
            | CharacterName::Morgana => true,
            _ => false,
        }
    }
}
