#[derive(Clone)]
pub struct WinnerRule<T> {
    pub candidate: T,
    pub required_votes: usize,
}
