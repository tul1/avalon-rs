use std::collections::HashMap;
use std::hash::Hash;

use crate::core::election::*;

#[derive(Hash, Eq, Clone, Copy, PartialEq, Debug)]
pub enum FakePresidentialElection {
    Donald,
    Alberto,
    Emmanuel,
    CoronaBoris,
}

#[test]
#[should_panic(expected = "No electors in this election!")]
fn test_election_no_election_without_electors() {
    let electors = [];
    let election = Election::<FakePresidentialElection>::new(&electors);
}

#[test]
#[should_panic(expected = "Elector doesn't exist!")]
fn test_election_elector_must_be_resgistered() {
    let electors = [String::from("jimi")];
    let mut election = Election::<FakePresidentialElection>::new(&electors);
    let fake_elector = String::from("jimbo");
    election.vote(&fake_elector, FakePresidentialElection::Emmanuel);
}

#[test]
fn test_election_elector_cannot_change_vote() {
    let electors = [String::from("jimi")];
    let mut election = Election::<FakePresidentialElection>::new(&electors);
    election.vote(&electors[0], FakePresidentialElection::CoronaBoris);
    election.vote(&electors[0], FakePresidentialElection::Emmanuel);
    assert_eq!(election.electors_votes[&electors[0]], Some(FakePresidentialElection::CoronaBoris));
}

#[test]
fn test_election_no_vote_counting_allow_before_everyone_have_voted() {
    let electors = [String::from("jimi"), String::from("pato"), String::from("volan")];
    let mut election = Election::<FakePresidentialElection>::new(&electors);
    assert!(election.count_votes().is_err());
}

#[test]
fn test_election_ownership_remains_in_current_scope() {
    let electors = [String::from("jimi"), String::from("pato"), String::from("volan")];
    let votes = [FakePresidentialElection::Donald,
                 FakePresidentialElection::Alberto,
                 FakePresidentialElection::CoronaBoris];
    let mut election = Election::<FakePresidentialElection>::new(&electors);
    for (index, voter) in electors.iter().enumerate() {
        election.vote(&voter, votes[index]);
    }
    let election_result = election.count_votes().ok().unwrap();
    let espected_election_result = HashMap::<Option<FakePresidentialElection>, usize>::new();
    assert_eq!(*election_result.result(), espected_election_result);
    assert_eq!(electors[0], String::from("jimi"));
    assert_eq!(electors[1], String::from("pato"));
    assert_eq!(electors[2], String::from("volan"));
    assert_eq!(votes[0], FakePresidentialElection::Donald);
    assert_eq!(votes[1], FakePresidentialElection::Alberto);
    assert_eq!(votes[2], FakePresidentialElection::CoronaBoris);
}
