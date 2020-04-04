use crate::datamodel::character::*;
use crate::datamodel::quest::*;
use crate::datamodel::*;

#[test]
fn test_game() {
    let players = vec![
        Player {
            name: "jimi".to_string(),
            character: Box::new(Merlin {}),
        },
        Player {
            name: "volan".to_string(),
            character: Box::new(Percival {}),
        },
        Player {
            name: "pato".to_string(),
            character: Box::new(Mordred {}),
        },
    ];

    let board = Board {
        quest_objectives: vec![
            QuestObjective {
                num_players: 2,
                required_successes: 2,
            },
            QuestObjective {
                num_players: 3,
                required_successes: 3,
            },
            QuestObjective {
                num_players: 4,
                required_successes: 3,
            },
        ],
    };

    let m = Game {
        num_players: 5,
        players,
        board,
    };
}
