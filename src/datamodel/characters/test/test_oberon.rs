use crate::datamodel::characters::*;

#[test]
fn test_oberon_constructor_is_correct() {
    let m = Oberon::new();
    assert_eq!(Oberon {}, m);
}

#[test]
fn test_oberon_get_team_is_correct() {
    let m = Oberon {};
    assert_eq!(Team::Evil, m.get_team());
}

#[test]
fn test_oberon_get_name_is_correct() {
    let m = Oberon {};
    assert_eq!(CharacterName::Oberon, m.get_name());
}

#[test]
fn test_oberon_is_not_seen_by_minion_of_mordred() {
    let m1 = Oberon {};
    let m2 = MinionOfMordred {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_oberon_is_not_seen_by_morgana() {
    let m1 = Oberon {};
    let m2 = Morgana {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_oberon_is_not_seen_by_mordred() {
    let m1 = Oberon {};
    let m2 = Mordred {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_oberon_is_not_seen_by_assassin() {
    let m1 = Oberon {};
    let m2 = Assassin {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_oberon_is_seen_by_merlin() {
    let m1 = Oberon {};
    let m2 = Merlin {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_oberon_is_not_seen_by_percival() {
    let m1 = Oberon {};
    let m2 = Percival {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_oberon_is_not_seen_by_loyal_servant_of_arthur() {
    let m1 = Oberon {};
    let m2 = LoyalServantOfArthur {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_oberon_cannot_see_minion_of_mordred() {
    let m1 = Oberon {};
    let m2 = MinionOfMordred {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_oberon_cannot_see_morgana() {
    let m1 = Oberon {};
    let m2 = Morgana {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_oberon_cannot_see_mordred() {
    let m1 = Oberon {};
    let m2 = Mordred {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_oberon_cannot_see_assassin() {
    let m1 = Oberon {};
    let m2 = Assassin {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_oberon_cannot_see_merlin() {
    let m1 = Oberon {};
    let m2 = Merlin {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_oberon_cannot_see_percival() {
    let m1 = Oberon {};
    let m2 = Percival {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_oberon_cannot_see_loyal_servant_of_arthur() {
    let m1 = Oberon {};
    let m2 = LoyalServantOfArthur {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}
