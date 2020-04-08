use crate::datamodel::quest::FinishedQuest;

pub struct FinishedTurn<'a> {
    pub quest: FinishedQuest<'a>,
}
