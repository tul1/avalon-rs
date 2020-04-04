use crate::datamodel::quest::quest_member::QuestMember;
use crate::datamodel::quest::quest_objective::QuestObjective;
use crate::datamodel::quest::quest_proposer::QuestProposer;

pub struct Quest {
    objective: QuestObjective,
    proposer: QuestProposer,
    members: Vec<QuestMember>,
}
