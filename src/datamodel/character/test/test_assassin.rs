use crate::datamodel::character::*;

//test ASSASSIN
//test constructor
#[test]
fn test_assassin_constructor_is_correct() {
    let m = Assassin::new();
    assert_eq!(Assassin {}, m);
}

//test get_team
#[test]
fn test_assassin_get_team_is_correct() {
    let m = Assassin {};
    assert_eq!(Team::Evil, m.get_team());
}

//test get_name
#[test]
fn test_assassin_get_name_is_correct() {
    let m = Assassin {};
    assert_eq!(CharacterName::Assassin, m.get_name());
}

//test is_seen_by
#[test]
fn test_assassin_is_seen_by_minion_of_mordred() {
    let m1 = Assassin {};
    let m2 = MinionOfMordred {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_assassin_is_seen_by_morgana() {
    let m1 = Assassin {};
    let m2 = Morgana {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_assassin_is_seen_by_mordred() {
    let m1 = Assassin {};
    let m2 = Mordred {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_assassin_is_not_seen_by_oberon() {
    let m1 = Assassin {};
    let m2 = Oberon {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_assassin_is_seen_by_merlin() {
    let m1 = Assassin {};
    let m2 = Merlin {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_assassin_is_not_seen_by_percival() {
    let m1 = Assassin {};
    let m2 = Percival {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_assassin_is_not_seen_by_loyal_servant_of_arthur() {
    let m1 = Assassin {};
    let m2 = LoyalServantOfArthur {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_assassin_can_see_minion_of_mordred() {
    let m1 = Assassin {};
    let m2 = MinionOfMordred {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_assassin_can_see_morgana() {
    let m1 = Assassin {};
    let m2 = Morgana {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_assassin_can_see_mordred() {
    let m1 = Assassin {};
    let m2 = Mordred {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_assassin_cannot_see_oberon() {
    let m1 = Assassin {};
    let m2 = Oberon {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_assassin_cannot_see_merlin() {
    let m1 = Assassin {};
    let m2 = Merlin {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_assassin_cannot_see_percival() {
    let m1 = Assassin {};
    let m2 = Percival {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_assassin_cannot_see_loyal_servant_of_arthur() {
    let m1 = Assassin {};
    let m2 = LoyalServantOfArthur {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}
