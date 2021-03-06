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
    let _election = Election::<FakePresidentialElection>::new(&electors);
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
    assert_eq!(
        election.electors_votes[&electors[0]],
        Some(FakePresidentialElection::CoronaBoris)
    );
}

#[test]
fn test_election_no_vote_counting_allowed_before_everyone_have_voted() {
    let electors = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let mut election = Election::<FakePresidentialElection>::new(&electors);
    election.vote(&electors[0], FakePresidentialElection::Emmanuel);
    let election_vote_count = election.count_votes();
    assert!(election_vote_count.is_err());

    let mut election = election_vote_count.err().unwrap();
    election.vote(&electors[1], FakePresidentialElection::Donald);
    election.vote(&electors[2], FakePresidentialElection::Alberto);
    assert!(election.count_votes().is_ok());
}

#[test]
fn test_election_nominal_case() {
    let electors = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let votes = [
        FakePresidentialElection::Donald,
        FakePresidentialElection::Alberto,
        FakePresidentialElection::CoronaBoris,
    ];
    let expected_election = votes.iter().clone().map(|v| (Some(*v), 1)).collect();

    let mut election = Election::<FakePresidentialElection>::new(&electors);
    for (index, voter) in electors.iter().enumerate() {
        election.vote(&voter, votes[index]);
    }
    let election_result = election.count_votes().ok().unwrap();
    assert_eq!(*election_result.result(), expected_election);
}
