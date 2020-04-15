use crate::datamodel::turn::turn_new::*;

#[test]
fn test_turn_new_going_into_quest_and_success() {
    let objective = Rc::new(QuestObjective {
        num_players: 3,
        required_successes: 3,
    });
    let mut all_players = vec!["Jimi", "Volan", "Pato", "pepe", "juancito"];
    let mut proposed_players = vec!["Pato", "pepe", "juancito"];

    let mut turn = Turn::new("Jimi", all_players, objective);

    turn.propose_players(proposed_players);
    
    turn.add_vote("Jimi", true);
    turn.add_vote("Volan", false);
    let turn = turn.go_into_quest();
    assert!(turn.is_err());
    let mut turn = turn.err().unwrap();
    turn.add_vote("Pato", true);
    turn.add_vote("pepe", true);
    turn.add_vote("juancito", true);

    let turn = turn.go_into_quest();
    assert!(turn.is_ok());
    let mut turn = turn.ok().unwrap();

    turn.add_quest_vote("Pato", true);
    turn.add_quest_vote("pepe", true);
    let turn = turn.finish_turn();
    assert!(turn.is_err());
    let mut turn = turn.err().unwrap();
    turn.add_quest_vote("juancito", true);

    let turn = turn.finish_turn();
    assert!(turn.is_ok());
    let mut turn = turn.ok().unwrap();
    
    assert_eq!(turn.result(), true);
}

#[test]
fn test_turn_new_go_into_quest_and_success() {
    let objective = Rc::new(QuestObjective {
        num_players: 3,
        required_successes: 3,
    });
    let mut all_players = vec!["Jimi", "Volan", "Pato", "pepe", "juancito"];
    let mut proposed_players = vec!["Pato", "pepe", "juancito"];

    let mut turn = Turn::new("Jimi", all_players, objective);

    turn.propose_players(proposed_players);
    
    turn.add_vote("Jimi", true);
    turn.add_vote("Volan", false);
    turn.add_vote("Pato", true);
    turn.add_vote("pepe", true);
    turn.add_vote("juancito", true);

    let turn = turn.go_into_quest();
    assert!(turn.is_err());
    let mut turn = turn.err().unwrap();

    turn.add_quest_vote("Pato", true);
    turn.add_quest_vote("pepe", false);
    let turn = turn.finish_turn();
    assert!(turn.is_err());
    let mut turn = turn.err().unwrap();
    turn.add_quest_vote("juancito", true);

    let turn = turn.finish_turn();
    let mut turn = turn.err().unwrap();
    
    assert_eq!(turn.ok().unwrap().result(), true);
}


#[test]
fn test_turn_new_not_going_into_quest() {
    let objective = Rc::new(QuestObjective {
        num_players: 3,
        required_successes: 3,
    });
    let mut all_players = vec!["Jimi", "Volan", "Pato", "pepe", "juancito"];
    let mut proposed_players = vec!["Pato", "pepe", "juancito"];

    let mut turn = Turn::new("Jimi", all_players, objective);

    turn.propose_players(proposed_players);
    
    turn.add_vote("Jimi", true);
    turn.add_vote("Volan", false);
    turn.add_vote("Pato", true);
    turn.add_vote("pepe", true);
    turn.add_vote("juancito", true);

    let turn = turn.go_into_quest();
    assert!(turn.is_err());
    let mut turn = turn.err().unwrap();
}