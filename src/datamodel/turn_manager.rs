use crate::datamodel::turns::proposal_voter_role_action_manager::ProposalVoterRoleActionManager;
use crate::datamodel::turns::{QuestMemberRoleActionManager, QuestProposerRoleActionManager};
use std::collections::HashMap;

pub struct TurnManager {
    proposal_votes: HashMap<String, bool>,
    quest_votes: HashMap<String, bool>,
}

impl TurnManager {
    pub fn new(
        proposal_votes: HashMap<String, bool>,
        quest_votes: HashMap<String, bool>,
    ) -> TurnManager {
        TurnManager {
            proposal_votes,
            quest_votes,
        }
    }

    pub fn receive_quest_proposal(
        &self,
        members: Vec<String>,
        proposer: &QuestProposerRoleActionManager,
    ) {
        unimplemented!()
    }

    pub fn receive_proposal_vote(&self, voter: &ProposalVoterRoleActionManager, vote: bool) {
        unimplemented!()
    }

    pub fn receive_quest_vote(&self, voter: &QuestMemberRoleActionManager, success: bool) {
        unimplemented!()
    }
}
