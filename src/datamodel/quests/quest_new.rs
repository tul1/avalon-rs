use std::hash::Hash;

use crate::core::election::Election;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Copy)]
pub enum Vote {
    Success,
    Failed,
}

pub struct WinnerRule {
    cantidate: Vote,
    electoral_cutoff: usize,
}

pub struct QuestNew<'a> {
    election: Election<Vote>,
    winner_rule: &'a WinnerRule,
}

impl<'a> QuestNew<'a> {
    pub fn new(quest_member: &[String], winner_rule: &'a WinnerRule) -> QuestNew<'a> {
        assert!(quest_member.len() >= winner_rule.electoral_cutoff, "Winner's rule cannot be bigger than quest member number");
        let election = Election::<Vote>::new(quest_member);
        QuestNew { election, winner_rule, }
    }

    pub fn vote(&mut self, quest_member: &String, vote: Vote) {
        self.election.vote(quest_member, vote);
    }

    pub fn finish_quest(self) -> Result<QuestResult, QuestNew<'a>> {
        let scrutiny = match self.election.count_votes() {
            Ok(s) => s,
            Err(_) => return Err(self),
        };
        let scrutiny = scrutiny.result();
        let quest_result = match (self.winner_rule.cantidate , scrutiny.get(&Some(self.winner_rule.cantidate))) { 
            (_ , Some(&votes_num)) if votes_num >= self.winner_rule.electoral_cutoff => self.winner_rule.cantidate,
            (Vote::Success , _) => Vote::Failed,
            (Vote::Failed , _) => Vote::Success,
        };
        Ok(QuestResult{ quest_result, })
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