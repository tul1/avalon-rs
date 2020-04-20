use crate::core::election::*;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum Vote2 {
    Success,
    Failed,
}

#[test]
fn test_election_nominal_scenario() {
    let voters = [String::from("jimi"), String::from("pato"), String::from("volan")];
    let mut election = Election::<Vote2>::new(&voters);
    println!("{:?}", election.voters);
    election.vote(&voters[0], Vote2::Success);
    println!("{:?}", election.voters);
    election.vote(&voters[0], Vote2::Failed);
    println!("{:?}", election.voters);
    election.vote(&voters[1], Vote2::Failed);
    election.vote(&voters[2], Vote2::Failed);
    println!("{:?}", election.voters);

    let election_result = election.count_votes().ok().unwrap();
    println!("{:?}", *election_result.result());
}