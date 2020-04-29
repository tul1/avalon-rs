use std::hash::Hash;

use crate::core::election::Election;
use crate::datamodel::quests::winner_rule::WinnerRule;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub enum Vote {
    Success,
    Failed,
}

pub struct QuestNew {
    pub election: Election<Vote>,
    pub winner_rule: WinnerRule<Vote>,
}

impl QuestNew {
    pub fn new(quest_member: &[String], winner_rule: WinnerRule<Vote>) -> QuestNew {
        assert!(
            quest_member.len() >= winner_rule.required_votes,
            "Winner's rule cannot be bigger than quest member number"
        );
        let election = Election::<Vote>::new(quest_member);
        QuestNew {
            election,
            winner_rule,
        }
    }

    pub fn vote(&mut self, quest_member: &str, vote: Vote) {
        self.election.vote(quest_member, vote);
    }

    pub fn finish_quest(self) -> Result<QuestResult, QuestNew> {
        let scrutiny = match self.election.count_votes() {
            Ok(s) => s,
            Err(e) => {
                return Err(QuestNew {
                    election: e,
                    winner_rule: self.winner_rule,
                })
            }
        };
        let scrutiny = scrutiny.result();
        let candidate = self.winner_rule.candidate;
        let election_result = scrutiny.get(&Some(candidate));
        let quest_result = match (candidate, election_result) {
            (_, Some(&votes_count)) if votes_count >= self.winner_rule.required_votes => candidate,
            (Vote::Success, _) => Vote::Failed,
            (Vote::Failed, _) => Vote::Success,
        };
        Ok(QuestResult { quest_result })
    }
}

pub struct QuestResult {
    quest_result: Vote,
}

impl QuestResult {
    pub fn result(&self) -> &Vote {
        &self.quest_result
    }
}
