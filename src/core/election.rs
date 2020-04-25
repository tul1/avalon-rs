use std::collections::HashMap;
use std::hash::Hash;

///     `Election` object setups and manages a generic votation process. Election's participants,
///     AKA `electors`, are able to vote for available `candidates`.

///     `Election` follows these guidelines:
///     * `Candidate` must be defined within an `Enum` type and there is no candidates' limit.
///     * `Elector` must be a `String` and must be registered when creating `Election` inside
///     an array of strings. `Election` won't own any `Elector`.
///     * `Vote` must be a `candidate` type variable and it will be consume by `Election` once it's used
///     to keep the secrecy of the votation. `Election` will own votes.
///     * Result of the `Election` is a `HashMap` containing `Some(Candidate)` as key and the
///     number of `electors` that have voted for him as key.
///
///     # Panics
///    
///     Election will panic with the following cases:
///     * Election is initialice with no electors (empty array), thrown error: "No electors in this election!".
///     * Voting with an elector not registered during Election's initialization, thrown error: "Elector doesn't exist!".
///    
///     # Examples
///   
///     ```
///     extern crate avalon_rs;
///
///     use std::hash::Hash;
///     use avalon_rs::core::election::Election;
///
///     #[derive(Hash, Eq, Clone, Copy, PartialEq, Debug)]
///     pub enum Candidates {
///         Candidate1,
///         Candidate2,
///         Candidate3,
///         Candidate4,
///     }
///     let electors = [String::from("elector1"),
///                     String::from("elector2"),
///                     String::from("elector3")];
///     let votes = [Candidates::Candidate1,
///                  Candidates::Candidate2,
///                  Candidates::Candidate3];
///
///     let mut election = Election::<Candidates>::new(&electors);
///     for (index, voter) in electors.iter().enumerate() {
///        election.vote(&voter, votes[index]);
///     }
///     let election_result = election.count_votes().ok().unwrap();
///     println!("{:?}", election_result.result());
///     ```
pub struct Election<T> {
    pub electors_votes: HashMap<String, Option<T>>,
}

impl<T> Election<T> {
    pub fn new(electors: &[String]) -> Election<T> {
        assert!(electors.len() > 0, "No electors in this election!");
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
