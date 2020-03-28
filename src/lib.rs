pub trait Character {
    fn who_am_i(&self) -> String;
    fn am_i_evil(&self) -> bool;
}

pub struct EvilCharacter {
    name: String,
    good: bool,
}

impl EvilCharacter {
    pub fn new(name: String) -> Self {
        EvilCharacter { name, good: false }
    }
}

impl Character for EvilCharacter {
    fn who_am_i(&self) -> String {
        self.name.clone()
    }
    fn am_i_evil(&self) -> bool {
        self.good
    }
}
