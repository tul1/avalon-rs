use crate::datamodel::quests::quest_new::*;
use crate::datamodel::quests::winner_rule::WinnerRule;

#[test]
#[should_panic(expected = "No electors in this election!")]
fn test_quest_no_quest_without_quest_members() {
    let quest_members = [];
    let winner_rule = WinnerRule {
        candidate: Vote::Success,
        required_votes: quest_members.len(),
    };
    let mut quest = QuestNew::new(&quest_members, winner_rule);
}

#[test]
#[should_panic(expected = "Elector doesn't exist!")]
fn test_quest_quest_member_must_be_resgistered() {
    let quest_members = [String::from("jimi")];
    let winner_rule = WinnerRule {
        candidate: Vote::Success,
        required_votes: quest_members.len(),
    };
    let mut quest = QuestNew::new(&quest_members, winner_rule);
    let fake_quest_members = String::from("jimbo");
    quest.vote(&fake_quest_members, Vote::Success);
}

#[test]
fn test_quest_not_finished_before_everyone_have_voted() {
    let quest_members = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let winner_rule = WinnerRule {
        candidate: Vote::Success,
        required_votes: quest_members.len(),
    };
    let mut quest = QuestNew::new(&quest_members, winner_rule);
    quest.vote(&quest_members[0], Vote::Success);
    let quest_result = quest.finish_quest();
    assert!(quest_result.is_err());

    let mut quest = quest_result.err().unwrap();
    quest.vote(&quest_members[1], Vote::Success);
    quest.vote(&quest_members[2], Vote::Success);
    let quest_result = quest.finish_quest();
    assert!(quest_result.is_ok());
}

#[test]
#[should_panic(expected = "Winner's rule cannot be bigger than quest member number")]
fn test_quest_panics_on_winner_rule_bigger_than_quest_member_num() {
    let quest_members = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let winner_rule = WinnerRule {
        candidate: Vote::Success,
        required_votes: quest_members.len() + 1,
    };
    let _quest = QuestNew::new(&quest_members, winner_rule);
}

#[test]
fn test_quest_successes_on_full_crew_voting_success_with_full_success_rule() {
    let quest_members = [
        String::from("jimi"),
        String::from("pato"),
        String::from("volan"),
    ];
    let votes = [Vote::Success, Vote::Success, Vote::Success];
    let winner_rule = WinnerRule {
        candidate: Vote::Success,
        required_votes: quest_members.len(),
    };
    let mut quest = QuestNew::new(&quest_members, winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest().ok().unwrap();
    assert_eq!(*quest_result.result(), Vote::Success);
}

#[test]
fn test_quest_fails_on_full_crew_voting_failed_with_full_success_rule() {
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
fn test_quest_fails_with_full_success_rule() {
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
fn test_quest_fails_with_success_rule() {
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
fn test_quest_successes_with_success_rule() {
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
