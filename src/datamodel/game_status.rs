use crate::datamodel::quests::Quest;
use crate::datamodel::turns::turn_history::TurnHistory;
use crate::datamodel::turns::Turn;

pub struct GameStatus<'a> {
    pub turn_history: TurnHistory<'a>,
    pub current_turn: Turn,
    pub failed_proposals_count: u32,
    pub current_quest: Option<Quest<'a>>,
}
