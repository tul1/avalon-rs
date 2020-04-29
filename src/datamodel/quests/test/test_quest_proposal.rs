use crate::datamodel::quests::quest_proposal::*;
use crate::datamodel::quests::winner_rule::WinnerRule;

#[test]
#[should_panic(expected = "No electors in this election!")]
fn test_quest_propasal_no_quest_propasal_without_players() {
    let players = [];
    let mut quest_proposal = QuestProposal::new(&players);
}

#[test]
#[should_panic(expected = "Elector doesn't exist!")]
fn test_quest_propasal_voter_must_be_resgistered() {
    let players = [String::from("jimi")];
    let mut quest_proposal = QuestProposal::new(&players);
    let fake_player = String::from("jimbo");
    quest_proposal.vote(&fake_player, ProposalVote::InFavor);
}

#[test]
fn test_quest_propasal_not_finished_before_everyone_have_voted() {
    let players = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let mut quest_proposal = QuestProposal::new(&players);
    quest_proposal.vote(&players[0], ProposalVote::InFavor);
    let quest_proposal_result = quest_proposal.finish_quest_proposal();
    assert!(quest_proposal_result.is_err());

    let mut quest_proposal = quest_proposal_result.err().unwrap();
    quest_proposal.vote(&players[1], ProposalVote::InFavor);
    quest_proposal.vote(&players[2], ProposalVote::InFavor);
    let quest_proposal_result = quest_proposal.finish_quest_proposal();
    assert!(quest_proposal_result.is_ok());
}

#[test]
fn test_quest_propasal_successes_on_full_crew_voting_in_favor() {
    let players = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
        String::from("juancito"),
    ];
    let mut quest_proposal = QuestProposal::new(&players);
    for player in &players {
        quest_proposal.vote(&player, ProposalVote::InFavor);
    }
    let quest_proposal_result = quest_proposal.finish_quest_proposal().ok().unwrap();
    assert_eq!(*quest_proposal_result.result(), ProposalVote::InFavor);
}

#[test]
fn test_quest_propasal_fails_on_full_crew_voting_against() {
    let players = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
        String::from("juancito"),
    ];
    let mut quest_proposal = QuestProposal::new(&players);
    for player in &players {
        quest_proposal.vote(&player, ProposalVote::Against);
    }
    let quest_proposal_result = quest_proposal.finish_quest_proposal().ok().unwrap();
    assert_eq!(*quest_proposal_result.result(), ProposalVote::Against);
}

#[test]
fn test_quest_propasal_fails_without_simple_majority_in_favor_on_even_players_number() {
    let players = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
        String::from("juancito"),
    ];
    let votes = [
        ProposalVote::Against,
        ProposalVote::Against,
        ProposalVote::InFavor,
        ProposalVote::InFavor,
    ];
    let mut quest_proposal = QuestProposal::new(&players);
    for (index, player) in players.iter().enumerate() {
        quest_proposal.vote(&player, votes[index]);
    }
    let quest_proposal_result = quest_proposal.finish_quest_proposal().ok().unwrap();
    assert_eq!(*quest_proposal_result.result(), ProposalVote::Against);
}

#[test]
fn test_quest_propasal_fails_without_simple_majority_in_favor_on_odd_players_number() {
    let players = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let votes = [
        ProposalVote::Against,
        ProposalVote::Against,
        ProposalVote::InFavor,
    ];
    let mut quest_proposal = QuestProposal::new(&players);
    for (index, player) in players.iter().enumerate() {
        quest_proposal.vote(&player, votes[index]);
    }
    let quest_proposal_result = quest_proposal.finish_quest_proposal().ok().unwrap();
    assert_eq!(*quest_proposal_result.result(), ProposalVote::Against);
}

#[test]
fn test_quest_propasal_successes_without_simple_majority_against_on_odd_players_number() {
    let players = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let votes = [
        ProposalVote::Against,
        ProposalVote::InFavor,
        ProposalVote::InFavor,
    ];
    let mut quest_proposal = QuestProposal::new(&players);
    for (index, player) in players.iter().enumerate() {
        quest_proposal.vote(&player, votes[index]);
    }
    let quest_proposal_result = quest_proposal.finish_quest_proposal().ok().unwrap();
    assert_eq!(*quest_proposal_result.result(), ProposalVote::InFavor);
}
