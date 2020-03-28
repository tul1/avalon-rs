use avalon_rs::*;

#[test]
fn i_am_groot() {
    let groot = EvilCharacter::new(String::from("pepe"));
    assert_eq!(groot.who_am_i(), String::from("pepe"));
}
