use crate::datamodel::character::Character;

pub struct Player {
    pub name: String,
    pub character: Box<dyn Character>,
}

impl Player {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
