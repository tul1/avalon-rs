use crate::datamodel::quests::Quest;

pub struct FinishedQuest<'a> {
    pub won: bool,
    pub quest: Quest<'a>,
}
