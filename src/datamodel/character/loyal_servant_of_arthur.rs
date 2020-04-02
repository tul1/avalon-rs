use crate::datamodel::character::character::{Character, CharacterName, Team};

#[derive(PartialEq, Debug)]
pub struct LoyalServantOfArthur {}

impl LoyalServantOfArthur {
    pub fn new() -> Self {
        LoyalServantOfArthur {}
    }
}

impl Character for LoyalServantOfArthur {
    fn get_team(&self) -> Team {
        Team::Good
    }

    fn get_name(&self) -> CharacterName {
        CharacterName::LoyalServantOfArthur
    }

    fn is_seen_by(&self, _other: &CharacterName) -> bool {
        false
    }

    fn can_see(&self, _other: &CharacterName) -> bool {
        false
    }
}
