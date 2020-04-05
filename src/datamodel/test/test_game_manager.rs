use crate::datamodel::characters::{Merlin, Mordred, Percival};
use crate::datamodel::game_manager::GameManager;
use crate::datamodel::game_status::GameStatus;
use crate::datamodel::player_action_manager::PlayerActionManager;
use crate::datamodel::quests::QuestObjective;
use crate::datamodel::test::test_utils::create_3_players;
use crate::datamodel::turn_manager::TurnManager;
use crate::datamodel::turns::turn::TurnRole;
use crate::datamodel::turns::turn::TurnRole::{ProposalVoter, QuestProposer};
use crate::datamodel::turns::turn_history::TurnHistory;
use crate::datamodel::turns::Turn;
use crate::datamodel::{Board, Game, Player};
use multimap::MultiMap;
use std::collections::HashMap;
use std::rc::Rc;

#[test]
pub fn test_game_manage_construction() {
    let players = create_3_players();

    let mut mut_initial_turn_roles: MultiMap<String, TurnRole> = MultiMap::new();

    for p in &players {
        mut_initial_turn_roles.insert((*p.name).to_string(), ProposalVoter);
    }

    mut_initial_turn_roles.insert(players[0].name.to_string(), QuestProposer);

    let initial_turn_roles = mut_initial_turn_roles;

    let turn_manager = Rc::new(TurnManager::new(HashMap::new(), HashMap::new()));

    let action_managers = players
        .iter()
        .map(|p| {
            (
                (p.name).to_string(),
                PlayerActionManager::new(Rc::clone(p), Rc::clone(&turn_manager)),
            )
        })
        .collect();

    let _game_manager = GameManager {
        game: Game {
            num_players: 3,
            players,
            board: Board {
                quest_objectives: vec![
                    QuestObjective {
                        num_players: 1,
                        required_successes: 1,
                    },
                    QuestObjective {
                        num_players: 2,
                        required_successes: 2,
                    },
                    QuestObjective {
                        num_players: 3,
                        required_successes: 2,
                    },
                ],
            },
        },
        status: GameStatus {
            turn_history: TurnHistory {
                finished_turns: vec![],
            },
            current_turn: Turn {
                current_turn_roles: initial_turn_roles,
                current_quest_objective: QuestObjective {
                    num_players: 1,
                    required_successes: 1,
                },
            },
            failed_proposals_count: 0,
            current_quest: None,
        },
        turn_manager,
        action_managers,
    };
}
