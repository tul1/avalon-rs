pub mod election;

pub use election::Election;

//Unit tests
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
