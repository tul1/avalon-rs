use crate::datamodel::characters::*;
use crate::datamodel::*;
use std::rc::Rc;

#[test]
fn test_get_name_is_correct() {
    let name = String::from("jimi");
    let player = Player {
        name,
        character: Rc::new(Merlin {}),
    };
    assert_eq!("jimi", player.get_name());
}
