use crate::datamodel::quest::quest_objective::QuestObjective;
use std::collections::HashMap;

pub enum TurnRole {
    QuestProposer,
    QuestMember,
    ProposalVoter,
}

pub struct Turn {
    pub current_turn_roles: HashMap<String, TurnRole>,
    pub current_quest_objective: QuestObjective,
}
