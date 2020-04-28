use crate::datamodel::quests::quest_proposal::*;
use crate::datamodel::quests::winner_rule::WinnerRule;

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
    let votes = [ProposalVote::InFavor, ProposalVote::InFavor, ProposalVote::InFavor, ProposalVote::InFavor];
    let mut quest_proposal = QuestNew::new(&quest_members, winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest().ok().unwrap();
    assert_eq!(*quest_result.result(), Vote::Success);
}

#[test]
fn test_quest_propasal_fails_on_full_crew_voting_failed() {
    let quest_members = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let votes = [Vote::Failed, Vote::Failed, Vote::Failed];
    let winner_rule = WinnerRule {
        candidate: Vote::Success,
        required_votes: quest_members.len(),
    };
    let mut quest = QuestNew::new(&quest_members, winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest().ok().unwrap();
    assert_eq!(*quest_result.result(), Vote::Failed);
}

#[test]
fn test_quest_propasal_fails_without_simple_majority_against() {
    let quest_members = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let votes = [Vote::Success, Vote::Success, Vote::Failed];
    let winner_rule = WinnerRule {
        candidate: Vote::Success,
        required_votes: quest_members.len(),
    };
    let mut quest = QuestNew::new(&quest_members, winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest().ok().unwrap();
    assert_eq!(*quest_result.result(), Vote::Failed);
}

#[test]
fn test_quest_propasal_successes_with_success_rule() {
    let quest_members = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let votes = [Vote::Success, Vote::Failed, Vote::Failed];
    let winner_rule = WinnerRule {
        candidate: Vote::Success,
        required_votes: quest_members.len() - 1,
    };
    let mut quest = QuestNew::new(&quest_members, winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest().ok().unwrap();
    assert_eq!(*quest_result.result(), Vote::Failed);
}

#[test]
fn test_quest_propasal_successes_with_success_rule() {
    let quest_members = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let votes = [Vote::Success, Vote::Success, Vote::Failed];
    let winner_rule = WinnerRule {
        candidate: Vote::Success,
        required_votes: quest_members.len() - 1,
    };
    let mut quest = QuestNew::new(&quest_members, winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest().ok().unwrap();
    assert_eq!(*quest_result.result(), Vote::Success);
}

#[test]
fn test_quest_successes_on_full_crew_voting_success_with_failed_rule() {
    let quest_members = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let votes = [Vote::Success, Vote::Success, Vote::Success];
    let winner_rule = (Vote::Failed, 1);
    let winner_rule = WinnerRule {
        candidate: Vote::Failed,
        required_votes: 1,
    };
    let mut quest = QuestNew::new(&quest_members, winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest().ok().unwrap();
    assert_eq!(*quest_result.result(), Vote::Success);
}

#[test]
fn test_quest_fails_with_failed_rule() {
    let quest_members = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let votes = [Vote::Success, Vote::Failed, Vote::Success];
    let winner_rule = WinnerRule {
        candidate: Vote::Failed,
        required_votes: 1,
    };
    let mut quest = QuestNew::new(&quest_members, winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest().ok().unwrap();
    assert_eq!(*quest_result.result(), Vote::Failed);
}

#[test]
fn test_quest_fails_on_full_crew_voting_failed_with_failed_rule() {
    let quest_members = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let votes = [Vote::Failed, Vote::Failed, Vote::Failed];
    let winner_rule = WinnerRule {
        candidate: Vote::Failed,
        required_votes: 1,
    };
    let mut quest = QuestNew::new(&quest_members, winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest().ok().unwrap();
    assert_eq!(*quest_result.result(), Vote::Failed);
}
