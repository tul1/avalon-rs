use crate::datamodel::characters::{Merlin, Mordred, Percival};
use crate::datamodel::game_status::GameStatus;
use crate::datamodel::quests::{Quest, QuestObjective};
use crate::datamodel::turns::turn_history::TurnHistory;
use crate::datamodel::turns::Turn;
use crate::datamodel::Player;
use std::rc::Rc;

#[test]
fn test_game_status_construction() {
    let _game_status = GameStatus {
        turn_history: TurnHistory {
            finished_turns: vec![],
        },
        current_turn: Turn {
            current_turn_roles: Default::default(),
            current_quest_objective: QuestObjective {
                num_players: 2,
                required_successes: 2,
            },
        },
        failed_proposals_count: 0,
        current_quest: Option::Some(Quest {
            objective: QuestObjective {
                num_players: 2,
                required_successes: 2,
            },
            proposer: &Player {
                name: "proposer".to_string(),
                character: Rc::new(Merlin {}),
            },
            members: vec![
                &Player {
                    name: "member1".to_string(),
                    character: Rc::new(Percival {}),
                },
                &Player {
                    name: "member2".to_string(),
                    character: Rc::new(Mordred {}),
                },
            ],
        }),
    };
}
