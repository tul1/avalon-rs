use crate::datamodel::quests::quest_new::*;
use std::rc::Rc;

#[test]
fn test_quest_state_sequence_nominal_fails() {
    let mut players = vec!["Jimi", "Volan", "Pato"];
    let objective = Rc::new(QuestObjective {
        num_players: 3,
        required_successes: 3,
    });
    let mut quest = QuestNew::new(players, Rc::clone(&objective));

    quest.add_vote("Volan", Vote::Success);
    quest.add_vote("Pato", Vote::Success);
    quest.add_vote("Pato", Vote::Failed);
    let quest = quest.finish_quest();
    assert!(quest.is_err());
    let mut quest = quest.err().unwrap();

    quest.add_vote("Jimi", Vote::Failed);

    let quest = quest.finish_quest();
    assert!(quest.is_ok());
    assert_eq!(quest.ok().unwrap().result(), false);
    assert_eq!(objective.num_players, 3);
}

#[test]
fn test_quest_state_sequence_nominal_successes() {
    let mut players = vec!["Jimi", "Volan", "Pato"];
    let objective = Rc::new(QuestObjective {
        num_players: 3,
        required_successes: 3,
    });
    let mut quest = QuestNew::new(players, Rc::clone(&objective));

    quest.add_vote("Volan", Vote::Success);
    quest.add_vote("Pato", Vote::Success);
    quest.add_vote("Pato", Vote::Success);
    let quest = quest.finish_quest();
    assert!(quest.is_err());
    let mut quest = quest.err().unwrap();

    quest.add_vote("Jimi", Vote::Success);

    let quest = quest.finish_quest();
    assert!(quest.is_ok());
    assert_eq!(quest.ok().unwrap().result(), true);
    assert_eq!(objective.num_players, 3);
}
