use crate::datamodel::quests::{FinishedQuest, Quest, QuestObjective};
use crate::datamodel::turns::Turn;

use crate::datamodel::characters::{Merlin, Mordred, Percival};
use crate::datamodel::test::test_utils::create_3_players;
use crate::datamodel::turns::finished_turn::FinishedTurn;
use crate::datamodel::turns::turn::TurnRole;
use crate::datamodel::turns::turn::TurnRole::{ProposalVoter, QuestProposer};
use crate::datamodel::turns::turn_history::TurnHistory;
use crate::datamodel::Player;
use multimap::MultiMap;
use std::rc::Rc;

#[test]
fn test_turn_construction() {
    let players = create_3_players();

    let mut mut_map: MultiMap<String, TurnRole> = MultiMap::new();

    mut_map.insert(players[0].name.clone(), ProposalVoter);
    mut_map.insert(players[1].name.clone(), ProposalVoter);
    mut_map.insert(players[2].name.clone(), ProposalVoter);

    mut_map.insert(players[2].name.clone(), QuestProposer);

    let map = mut_map;

    let _turn = Turn {
        current_turn_roles: map,
        current_quest_objective: QuestObjective {
            num_players: 3,
            required_successes: 3,
        },
    };
}

#[test]
fn test_turn_history_construction() {
    let players = create_3_players();
    let _turn_history = TurnHistory {
        finished_turns: vec![FinishedTurn {
            quest: FinishedQuest {
                won: false,
                quest: Quest {
                    objective: QuestObjective {
                        num_players: 2,
                        required_successes: 2,
                    },
                    proposer: &players[0],
                    members: vec![&players[1], &players[2]],
                },
            },
        }],
    };
}
