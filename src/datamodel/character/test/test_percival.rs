use crate::datamodel::character::*;

#[test]
fn test_percival_constructor_is_correct() {
    let m = Percival::new();
    assert_eq!(Percival {}, m);
}

#[test]
fn test_percival_get_team_is_correct() {
    let m = Percival {};
    assert_eq!(Team::Good, m.get_team());
}

#[test]
fn test_percival_get_name_is_correct() {
    let m = Percival {};
    assert_eq!(CharacterName::Percival, m.get_name());
}

#[test]
fn test_percival_is_not_seen_by_minion_of_mordred() {
    let m1 = Percival {};
    let m2 = MinionOfMordred {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_percival_is_not_seen_by_morgana() {
    let m1 = Percival {};
    let m2 = Morgana {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_percival_is_not_seen_by_mordred() {
    let m1 = Percival {};
    let m2 = Mordred {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_percival_is_not_seen_by_assassin() {
    let m1 = Percival {};
    let m2 = Assassin {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_percival_is_not_seen_by_oberon() {
    let m1 = Percival {};
    let m2 = Oberon {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_percival_is_not_seen_by_merlin() {
    let m1 = Percival {};
    let m2 = Merlin {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_percival_is_not_seen_by_loyal_servant_of_arthur() {
    let m1 = Percival {};
    let m2 = LoyalServantOfArthur {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_percival_cannot_see_minion_of_mordred() {
    let m1 = Percival {};
    let m2 = MinionOfMordred {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_percival_can_see_morgana() {
    let m1 = Percival {};
    let m2 = Morgana {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_percival_cannot_see_mordred() {
    let m1 = Percival {};
    let m2 = Mordred {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_percival_cannot_see_assassin() {
    let m1 = Percival {};
    let m2 = Assassin {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_percival_cannot_see_oberon() {
    let m1 = Percival {};
    let m2 = Oberon {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_percival_can_see_merlin() {
    let m1 = Percival {};
    let m2 = Merlin {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_percival_cannot_see_loyal_servant_of_arthur() {
    let m1 = Percival {};
    let m2 = LoyalServantOfArthur {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}
