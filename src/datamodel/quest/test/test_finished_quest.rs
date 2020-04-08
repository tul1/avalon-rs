use crate::datamodel::character::{Merlin, Mordred, Percival};
use crate::datamodel::quest::{FinishedQuest, Quest, QuestObjective};
use crate::datamodel::test::test_utils::create_3_players;
use crate::datamodel::Player;

#[test]
fn test_finished_quest_creation() {
    let players = create_3_players();
    let _finished_quest: FinishedQuest = FinishedQuest {
        won: true,
        quest: Quest {
            objective: QuestObjective {
                num_players: 2,
                required_successes: 2,
            },
            proposer: &players[0],
            members: vec![&players[1], &players[2]],
        },
    };
}
