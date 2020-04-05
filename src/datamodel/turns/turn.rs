use crate::datamodel::quests::quest_objective::QuestObjective;
use multimap::MultiMap;

pub enum TurnRole {
    QuestProposer,
    QuestMember,
    ProposalVoter,
}

pub struct Turn {
    pub current_turn_roles: MultiMap<String, TurnRole>,
    pub current_quest_objective: QuestObjective,
}
