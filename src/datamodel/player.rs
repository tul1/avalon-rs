use crate::datamodel::characters::Character;
use std::rc::Rc;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub character: Rc<dyn Character>,
}

impl Player {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
