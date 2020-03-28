use crate::utils::{Quest, Vote};

pub trait Character<T> {
    fn who_am_i(&self) -> &T;
    fn am_i_evil(&self) -> bool;
    fn am_i_good(&self) -> bool;
    fn am_i_alive(&self) -> bool;
    fn am_i_dead(&self) -> bool;
    fn die(&mut self);
    fn i_vote_for_mission_to(&mut self, vote: Vote);
    fn did_i_vote_to_go(&self) -> bool;
    fn did_i_vote_to_stay(&self) -> bool;
    fn in_quest_i_will(&mut self, quest: Quest);
}
