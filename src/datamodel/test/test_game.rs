use crate::datamodel::characters::*;
use crate::datamodel::quests::*;
use crate::datamodel::test::test_utils::create_3_players;
use crate::datamodel::*;
use std::rc::Rc;

#[test]
fn test_game() {
    let players = create_3_players();
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

    let _game = Game {
        num_players: 5,
        players,
        board,
    };
}
