use std::hash::Hash;

use crate::core::election::Election;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub enum Vote {
    Success,
    Failed,
}

#[derive(Clone)]
pub struct WinnerRule {
    pub candidate: Vote,
    pub electoral_cutoff: usize,
}

pub struct QuestNew {
    pub election: Election<Vote>,
    pub winner_rule: WinnerRule,
}

impl QuestNew {
    pub fn new(quest_member: &[String], winner_rule: &WinnerRule) -> QuestNew {
        assert!(
            quest_member.len() >= winner_rule.electoral_cutoff,
            "Winner's rule cannot be bigger than quest member number"
        );
        let election = Election::<Vote>::new(quest_member);
        QuestNew {
            election,
            winner_rule: winner_rule.clone(),
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
        let quest_result = match (
            self.winner_rule.candidate,
            scrutiny.get(&Some(self.winner_rule.candidate)),
        ) {
            (_, Some(&votes_num)) if votes_num >= self.winner_rule.electoral_cutoff => {
                self.winner_rule.candidate
            }
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
