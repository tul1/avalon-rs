pub trait Character {
    fn get_team(&self) -> Team;
    fn get_name(&self) -> CharacterName;
    fn is_seen_by(&self, other: &CharacterName) -> bool;
    fn can_see(&self, other: &CharacterName) -> bool;
}

#[derive(PartialEq, Debug)]
pub enum CharacterName {
    LoyalServantOfArthur,
    MinionOfMordred,
    Merlin,
    Percival,
    Morgana,
    Mordred,
    Assassin,
    Oberon,
}

#[derive(PartialEq, Debug)]
pub enum Team {
    Good,
    Evil,
}
