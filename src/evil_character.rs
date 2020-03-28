use crate::character::Character;
use crate::utils::{Quest, Vote};

#[derive(PartialEq, Debug)]
pub enum EvilCharacterName {
    Mordred,
    MinionOfMordred1,
    MinionOfMordred2,
    MinionOfMordred3,
    Assasin,
    Oberon,
    Morgana,
}

pub struct EvilCharacter {
    name: EvilCharacterName,
    alive: bool,
    vote: Option<Vote>,
    quest: Option<Quest>,
}

impl EvilCharacter {
    pub fn new(name: EvilCharacterName) -> Self {
        EvilCharacter {
            name,
            alive: true,
            vote: None,
            quest: None,
        }
    }
}

impl Character<EvilCharacterName> for EvilCharacter {
    fn who_am_i(&self) -> &EvilCharacterName {
        &self.name
    }

    fn am_i_evil(&self) -> bool {
        true
    }

    fn am_i_good(&self) -> bool {
        false
    }

    fn am_i_alive(&self) -> bool {
        self.alive
    }

    fn am_i_dead(&self) -> bool {
        !self.alive
    }

    fn die(&mut self) {
        self.alive = false;
    }

    fn i_vote_for_mission_to(&mut self, vote: Vote) {
        self.vote = Some(vote);
    }

    fn did_i_vote_to_go(&self) -> bool {
        self.vote == Some(Vote::Approve)
    }

    fn did_i_vote_to_stay(&self) -> bool {
        self.vote == Some(Vote::Reject)
    }

    fn in_quest_i_will(&mut self, quest: Quest) {
        self.quest = Some(quest);
    }
}
