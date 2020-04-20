use std::collections::HashMap;
use std::hash::Hash;

pub struct Election<T> {
    pub voters: HashMap<String, Option<T>>,
}

impl<T> Election<T> {
    pub fn new(voters: &[String]) -> Election<T> {
        let voters: HashMap<String, Option<T>> = (*voters).iter()
                                                          .map(|v| (v.clone(), None))
                                                          .collect();
        Election { voters, }
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
            Ok(Scrutiny { result: votes_counter, })
        }
    }
}

pub struct Scrutiny<T> {
    result: HashMap::<Option<T>, usize>,
}

impl<T> Scrutiny<T> {
    pub fn result(&self) -> &HashMap::<Option<T>, usize> {
        &self.result
    }
}
