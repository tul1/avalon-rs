use crate::datamodel::turn::finished_turn::FinishedTurn;

pub struct TurnHistory<'a> {
    pub finished_turns: Vec<FinishedTurn<'a>>,
}
