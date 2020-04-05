use crate::datamodel::quests::FinishedQuest;

pub struct FinishedTurn<'a> {
    pub quest: FinishedQuest<'a>,
}
