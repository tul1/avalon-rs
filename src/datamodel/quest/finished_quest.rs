use crate::datamodel::quest::Quest;

pub struct FinishedQuest<'a> {
    pub won: bool,
    pub quest: Quest<'a>,
}
