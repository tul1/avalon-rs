use std::collections::HashMap;


pub struct QuestProposal {
    players: HashMap<&'static str, bool>,
    positive_votes: u32, 
}

impl QuestProposal {
    pub fn new(players: Vec<&'static str>) -> QuestProposal {
        let players: HashMap<&'static str, bool> = players.into_iter()
                                                          .map(|m| (m, false))
                                                          .collect();
        QuestProposal { players, positive_votes: 0,}
    }

    pub fn add_vote(&mut self, player: &str, vote: bool) {
        if !self.players[player] {       
            if vote {
                self.positive_votes += 1;
            }
            *self.players.get_mut(player).unwrap() = true;
        }
    }

    pub fn finish_quest_proposal(self) -> Result<FinishQuestProposal, QuestProposal> {
        let unvoted: Vec<bool> = self.players.iter()
                                             .map(|m| *m.1)
                                             .filter(|v| !*v)
                                             .collect();
        if unvoted.len() > 0 {
            Err(self)
        } else {
            if self.positive_votes > (self.players.len() as u32 / 2) {
                Ok(FinishQuestProposal { success: true })
            } else {
                Ok(FinishQuestProposal { success: false })
            }
        }
    }

}

pub struct FinishQuestProposal {
    success: bool,
}

impl FinishQuestProposal{
    pub fn result(&self) -> bool {
        self.success
    }
}
