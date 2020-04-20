use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Vote2 {
    Success,
    Failed,
}

pub struct Election<T> {
    pub voters: HashMap<String, Option<T>>,
    pub winner_threshold: u32,
}

impl<T> Election<T> {
    pub fn new(voters: &[String], winner_threshold: u32) -> Election<T> {
        let voters: HashMap<String, Option<T>> = (*voters).iter()
                                                          .map(|v| (v.clone(), None))
                                                          .collect();
        Election { voters, winner_threshold, }
    }

    pub fn vote(&mut self, voter: &String, vote: T) {
        if let Some(voter) = self.voters.get_mut(voter) {
            voter.get_or_insert(vote);
        }
    }

    pub fn count_votes(self) -> Result<Scrutiny<T>, Election<T>> 
    where T: 
        Eq + Hash + Clone,
    {
        let unvoted = self.voters.values()
                                 .filter(|v| !v.is_some())
                                 .count();
        if unvoted > 0 {
            Err(self)
        } else {
            let mut votes_counter = HashMap::<Option<T>, usize>::new();
            for vote in self.voters.values() {
                if let Some(val) = votes_counter.get_mut(&vote) {
                    *val += 1;
                } else {
                     votes_counter.insert(vote.clone(), 1);
                }
            }
            let winner = votes_counter.into_iter()
                                      .max_by_key(|(_, v)| *v);
            Ok(Scrutiny { winner: winner.unwrap().0 })
        }
    }
}

pub struct Scrutiny<T> {
    winner: Option<T>,
}

impl<T> Scrutiny<T> {
    pub fn result(&self) -> &Option<T> {
        &self.winner
    }
}
