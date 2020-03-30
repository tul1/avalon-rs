use crate::datamodel::character::*;

#[test]
fn test_minion_of_mordred_constructor_is_correct() {
    let m = MinionOfMordred::new();
    assert_eq!(MinionOfMordred {}, m);
}

#[test]
fn test_minion_of_mordred_get_team_is_correct() {
    let m = MinionOfMordred {};
    assert_eq!(Team::Evil, m.get_team());
}

#[test]
fn test_minion_of_mordred_get_name_is_correct() {
    let m = MinionOfMordred {};
    assert_eq!(CharacterName::MinionOfMordred, m.get_name());
}

#[test]
fn test_minion_of_mordred_is_seen_by_minion_of_mordred() {
    let m1 = MinionOfMordred {};
    let m2 = MinionOfMordred {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_is_seen_by_morgana() {
    let m1 = MinionOfMordred {};
    let m2 = Morgana {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_is_seen_by_mordred() {
    let m1 = MinionOfMordred {};
    let m2 = Mordred {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_is_seen_by_assassin() {
    let m1 = MinionOfMordred {};
    let m2 = Assassin {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_is_not_seen_by_oberon() {
    let m1 = MinionOfMordred {};
    let m2 = Oberon {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_is_seen_by_merlin() {
    let m1 = MinionOfMordred {};
    let m2 = Merlin {};
    assert_eq!(true, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_is_not_seen_by_percival() {
    let m1 = MinionOfMordred {};
    let m2 = Percival {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_is_not_seen_by_loyal_servant_of_arthur() {
    let m1 = MinionOfMordred {};
    let m2 = LoyalServantOfArthur {};
    assert_eq!(false, m1.is_seen_by(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_can_see_minion_of_mordred() {
    let m1 = MinionOfMordred {};
    let m2 = MinionOfMordred {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_can_see_morgana() {
    let m1 = MinionOfMordred {};
    let m2 = Morgana {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_can_see_mordred() {
    let m1 = MinionOfMordred {};
    let m2 = Mordred {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_can_see_assassin() {
    let m1 = MinionOfMordred {};
    let m2 = Assassin {};
    assert_eq!(true, m1.can_see(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_can_see_oberon() {
    let m1 = MinionOfMordred {};
    let m2 = Oberon {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_cannot_see_merlin() {
    let m1 = MinionOfMordred {};
    let m2 = Merlin {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_cannot_see_percival() {
    let m1 = MinionOfMordred {};
    let m2 = Percival {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}

#[test]
fn test_minion_of_mordred_cannot_see_loyal_servant_of_arthur() {
    let m1 = MinionOfMordred {};
    let m2 = LoyalServantOfArthur {};
    assert_eq!(false, m1.can_see(&m2.get_name()));
}
