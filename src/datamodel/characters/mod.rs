pub mod assassin;
pub mod character;
pub mod loyal_servant_of_arthur;
pub mod merlin;
pub mod minion_of_mordred;
pub mod mordred;
pub mod morgana;
pub mod oberon;
pub mod percival;

//TODO I suspect these uses can be cut and pasted elsewhere
pub use {
    assassin::*, character::*, loyal_servant_of_arthur::*, merlin::*, minion_of_mordred::*,
    mordred::*, morgana::*, oberon::*, percival::*,
};

//Unit tests
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
