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
    quest_result: Option<Vote>,
}

impl<'a> QuestNew<'a> {
    pub fn new(quest_member: &Vec<String>, winner_rule: &'a (Vote, usize)) -> QuestNew<'a> {
        let election = Election::<Vote>::new(quest_member);
        QuestNew {
            election,
            winner_rule,
            quest_result: None,
        }
    }

    pub fn vote(&mut self, quest_member: &String, vote: Vote) {
        self.election.vote(quest_member, vote);
    }

    pub fn finish_quest(self) -> Option<Vote> {
        if let Some(mut result) = self.quest_result {
            let scrutiny = self.election.count_votes().ok().unwrap().result().clone();
            result = match self.winner_rule.0 {
                Vote::Success if scrutiny.get(&Some(self.winner_rule.0)) <= Some(&self.winner_rule.1) => Vote::Success,
                Vote::Success if scrutiny.get(&Some(self.winner_rule.0)) > Some(&self.winner_rule.1) => Vote::Failed,
                Vote::Failed if scrutiny.get(&Some(self.winner_rule.0)) <= Some(&self.winner_rule.1) => Vote::Failed,
                Vote::Failed if scrutiny.get(&Some(self.winner_rule.0)) > Some(&self.winner_rule.1) => Vote::Success,
                _ => panic!("There is not other case available for vote"),
            }
        }
        self.quest_result
    }
}
