use crate::datamodel::turns::finished_turn::FinishedTurn;

pub struct TurnHistory<'a> {
    pub finished_turns: Vec<FinishedTurn<'a>>,
}
