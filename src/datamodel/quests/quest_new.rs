use std::hash::Hash;

use crate::core::election::Election;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub enum Vote {
    Success,
    Failed,
}

pub struct QuestNew<'a> {
    election: Election<Vote>,
    winner_rule: &'a (Vote, usize),
}

impl<'a> QuestNew<'a> {
    pub fn new(quest_member: &[String], winner_rule: &'a (Vote, usize)) -> QuestNew<'a> {
        let election = Election::<Vote>::new(quest_member);
        QuestNew {
            election,
            winner_rule,
        }
    }

    pub fn vote(&mut self, quest_member: &String, vote: Vote) {
        self.election.vote(quest_member, vote);
    }

    pub fn finish_quest(self) -> QuestResult {
        let scrutiny = self.election.count_votes().ok().unwrap();
        match (self.winner_rule.0 , scrutiny.result().get(&Some(self.winner_rule.0))) {
            (_ , Some(&votes_num)) if votes_num >= self.winner_rule.1 => QuestResult{ quest_result: self.winner_rule.0, },
            (Vote::Success , _) => QuestResult{ quest_result: Vote::Success, },
            (Vote::Failed , _) => QuestResult{ quest_result: Vote::Success, },
        }
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