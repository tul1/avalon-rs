use crate::datamodel::quests::quest_proposal::*;

#[test]
fn test_quest_proposal_positive_result() {
    let mut all_players = vec!["Jimi", "Volan", "Pato", "pepe", "juancito"];

    let mut quest_proposal = QuestProposal::new(all_players);

    quest_proposal.add_vote("Jimi", true);
    quest_proposal.add_vote("Volan", true);
    let quest_proposal = quest_proposal.finish_quest_proposal();
    assert!(quest_proposal.is_err());
    let mut quest_proposal = quest_proposal.err().unwrap();
    quest_proposal.add_vote("Pato", true);    
    quest_proposal.add_vote("pepe", true);
    quest_proposal.add_vote("juancito", false);

    let quest_proposal = quest_proposal.finish_quest_proposal();
    assert_eq!(quest_proposal.ok().unwrap().result(), true);

}

#[test]
fn test_quest_proposal_negative_result() {
    let mut all_players = vec!["Jimi", "Volan", "Pato", "pepe", "juancito"];

    let mut quest_proposal = QuestProposal::new(all_players);

    quest_proposal.add_vote("Jimi", false);
    quest_proposal.add_vote("Volan", false);
    let quest_proposal = quest_proposal.finish_quest_proposal();
    assert!(quest_proposal.is_err());
    let mut quest_proposal = quest_proposal.err().unwrap();
    quest_proposal.add_vote("Pato", true);    
    quest_proposal.add_vote("pepe", true);
    quest_proposal.add_vote("juancito", false);

    let quest_proposal = quest_proposal.finish_quest_proposal();
    assert_eq!(quest_proposal.ok().unwrap().result(), false);

}

#[test]
fn test_quest_proposal_positive_result_on_tie() {
    let mut all_players = vec!["Jimi", "Volan", "Pato", "pepe"];

    let mut quest_proposal = QuestProposal::new(all_players);

    quest_proposal.add_vote("Jimi", false);
    quest_proposal.add_vote("Volan", false);
    let quest_proposal = quest_proposal.finish_quest_proposal();
    assert!(quest_proposal.is_err());
    let mut quest_proposal = quest_proposal.err().unwrap();
    quest_proposal.add_vote("Pato", true);    
    quest_proposal.add_vote("pepe", true);

    let quest_proposal = quest_proposal.finish_quest_proposal();
    assert_eq!(quest_proposal.ok().unwrap().result(), false);

}
