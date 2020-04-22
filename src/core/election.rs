use std::collections::HashMap;
use std::hash::Hash;

pub struct Election<T> {
    pub electors_votes: HashMap<String, Option<T>>,
}

impl<T> Election<T> {
    pub fn new(electors: &[String]) -> Election<T> {
        if electors.len() == 0 {
            panic!("No electors in this election!");
        }
        let electors_votes: HashMap<String, Option<T>> = (*electors).iter()
                                                          .map(|v| (v.clone(), None))
                                                          .collect();
        Election { electors_votes, }
    }

    pub fn vote(&mut self, elector: &String, vote: T) {
        if let Some(elector) = self.electors_votes.get_mut(elector) {
            elector.get_or_insert(vote);
        } else {
            panic!("Elector doesn't exist!");
        }
    }

    pub fn count_votes(self) -> Result<Scrutiny<T>, Election<T>> 
    where T: 
        Eq + Hash + Clone,
    {
        let elector_not_having_voted = self.electors_votes.values()
                                                          .filter(|v| !v.is_some())
                                                          .count();
        if elector_not_having_voted > 0 {
            Err(self)
        } else {
            let mut votes_counter = HashMap::<Option<T>, usize>::new();
            for vote in self.electors_votes.values() {
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
