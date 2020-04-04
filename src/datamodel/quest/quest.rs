use crate::datamodel::quest::quest_objective::QuestObjective;
use crate::datamodel::quest::quest_proposer::QuestProposer;
use crate::datamodel::quest::quest_member::QuestMember;

pub struct Quest{
    objective: QuestObjective,
    proposer: QuestProposer,
    members: Vec<QuestMember>,
}
