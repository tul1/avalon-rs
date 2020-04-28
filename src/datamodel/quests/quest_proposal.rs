use std::hash::Hash;

use crate::core::election::Election;
use crate::datamodel::quests::winner_rule::WinnerRule;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub enum ProposalVote {
    InFavor,
    Against,
}

pub struct QuestProposal {
    pub election: Election<ProposalVote>,
    pub winner_rule: WinnerRule<ProposalVote>,
}

impl QuestProposal {
    pub fn new(players: &[String]) -> QuestProposal {
        let winner_rule = WinnerRule {
            candidate: ProposalVote::InFavor,
            required_votes: players.len() / 2,
        };
        let election = Election::<ProposalVote>::new(players);
        QuestProposal {
            election,
            winner_rule,
        }
    }

    pub fn vote(&mut self, player: &str, vote: ProposalVote) {
        self.election.vote(player, vote);
    }

    pub fn finish_quest_proposal(self) -> Result<QuestProposalResult, QuestProposal> {
        let scrutiny = match self.election.count_votes() {
            Ok(s) => s,
            Err(e) => {
                return Err(QuestProposal {
                    election: e,
                    winner_rule: self.winner_rule,
                })
            }
        };
        let scrutiny = scrutiny.result();
        let candidate = self.winner_rule.candidate;
        let election_result = scrutiny.get(&Some(candidate));
        let quest_proposal_result = match (candidate, election_result) {
            (_, Some(&votes_count)) if votes_count >= self.winner_rule.required_votes => candidate,
            (ProposalVote::InFavor, _) => ProposalVote::Against,
            (ProposalVote::Against, _) => ProposalVote::InFavor,
        };
        Ok(QuestProposalResult {
            quest_proposal_result,
        })
    }
}

pub struct QuestProposalResult {
    quest_proposal_result: ProposalVote,
}

impl QuestProposalResult {
    pub fn result(&self) -> &ProposalVote {
        &self.quest_proposal_result
    }
}
