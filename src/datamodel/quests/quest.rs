use crate::datamodel::quests::quest_objective::QuestObjective;
use crate::datamodel::Player;

pub struct Quest<'a> {
    pub objective: QuestObjective,
    pub proposer: &'a Player,
    pub members: Vec<&'a Player>,
}
