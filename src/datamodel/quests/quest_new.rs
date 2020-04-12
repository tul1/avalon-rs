use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum Vote {
    Success,
    Failed,
}

pub struct QuestObjective {
    pub num_players: u32,
    pub required_successes: u32,
}

pub struct QuestNew {
    members: HashMap<&'static str, bool>,
    objective: Rc<QuestObjective>,
    success_votes: u32,
}

impl QuestNew {
    pub fn new(members: Vec<&'static str>, objective: Rc<QuestObjective>) -> QuestNew {
        let members: HashMap<&'static str, bool> =
            members.into_iter().map(|m| (m, false)).collect();
        QuestNew {
            objective,
            members,
            success_votes: 0,
        }
    }

    pub fn add_vote(&mut self, player: &str, vote: Vote) {
        if self.members[player] == true {
            return;
        }
        if vote == Vote::Success {
            self.success_votes += 1;
        }
        *self.members.get_mut(player).unwrap() = true;
    }

    pub fn finish_quest(self) -> Result<FinishedQuestNew, QuestNew> {
        let unvoted: Vec<bool> = self.members.iter().map(|m| *m.1).filter(|v| !*v).collect();
        if unvoted.len() > 0 {
            Err(self)
        } else {
            if self.success_votes == self.objective.required_successes {
                Ok(FinishedQuestNew { success: true })
            } else {
                Ok(FinishedQuestNew { success: false })
            }
        }
    }
}

pub struct FinishedQuestNew {
    success: bool,
}

impl FinishedQuestNew {
    pub fn result(&self) -> bool {
        self.success
    }
}
