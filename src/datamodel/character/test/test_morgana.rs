use crate::datamodel::character::*;

#[test]
fn test_morgana_constructor_is_correct() {
    let m = Morgana::new();
    assert_eq!(Morgana {}, m);
}

#[test]
fn test_morgana_get_team_is_correct() {
    let m = Morgana {};
    assert_eq!(Team::Evil, m.get_team());
}

#[test]
fn test_morgana_get_name_is_correct() {
    let m = Morgana {};
    assert_eq!(CharacterName::Morgana, m.get_name());
}

#[test]
fn test_morgana_is_seen_by_minion_of_mordred() {
    let m1 = Morgana {};
    let m2 = MinionOfMordred {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_morgana_is_seen_by_mordred() {
    let m1 = Morgana {};
    let m2 = Mordred {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_morgana_is_seen_by_assassin() {
    let m1 = Morgana {};
    let m2 = Assassin {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_morgana_is_not_seen_by_oberon() {
    let m1 = Morgana {};
    let m2 = Oberon {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_morgana_is_seen_by_merlin() {
    let m1 = Morgana {};
    let m2 = Merlin {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_morgana_is_seen_by_percival() {
    let m1 = Morgana {};
    let m2 = Percival {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_morgana_is_not_seen_by_loyal_servant_of_arthur() {
    let m1 = Morgana {};
    let m2 = LoyalServantOfArthur {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_morgana_can_see_minion_of_mordred() {
    let m1 = Morgana {};
    let m2 = MinionOfMordred {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_morgana_can_see_mordred() {
    let m1 = Morgana {};
    let m2 = Mordred {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_morgana_can_see_assassin() {
    let m1 = Morgana {};
    let m2 = Assassin {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_morgana_cannot_see_oberon() {
    let m1 = Morgana {};
    let m2 = Oberon {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_morgana_cannot_see_merlin() {
    let m1 = Morgana {};
    let m2 = Merlin {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_morgana_cannot_see_percival() {
    let m1 = Morgana {};
    let m2 = Percival {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_morgana_cannot_see_loyal_servant_of_arthur() {
    let m1 = Morgana {};
    let m2 = LoyalServantOfArthur {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}
