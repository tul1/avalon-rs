use crate::datamodel::character::*;
use crate::datamodel::*;

#[test]
fn test_get_name_is_correct() {
    let name = String::from("jimi");
    let player = Player{name, character: Box::new(Merlin{})};
    assert_eq!("jimi", player.get_name());
}
