use avalon_rs::character::*;
use avalon_rs::evil_character::*;
use avalon_rs::utils::*;

fn hello_i_am_mordred() -> EvilCharacter {
    EvilCharacter::new(EvilCharacterName::Mordred)
}

#[test]
fn i_am_mordred() {
    assert_eq!(*hello_i_am_mordred().who_am_i(), EvilCharacterName::Mordred);
}

#[test]
fn i_am_not_mordred() {
    assert_ne!(*hello_i_am_mordred().who_am_i(), EvilCharacterName::Morgana);
}

#[test]
fn is_mordred_evil() {
    assert_eq!(hello_i_am_mordred().am_i_evil(), true);
}

#[test]
fn is_mordred_good() {
    assert_eq!(hello_i_am_mordred().am_i_good(), false);
}

#[test]
fn has_mordred_borned_alive() {
    assert_eq!(hello_i_am_mordred().am_i_alive(), true);
}

#[test]
fn has_mordred_borned_dead() {
    assert_eq!(hello_i_am_mordred().am_i_dead(), false);
}

#[test]
fn is_mordred_really_dead() {
    let mut player = hello_i_am_mordred();
    player.die();
    assert_eq!(player.am_i_dead(), true);
}

#[test]
fn is_mordred_a_zombie() {
    let mut player = hello_i_am_mordred();
    player.die();
    assert_eq!(player.am_i_alive(), false);
}

#[test]
fn is_mordred_against_when_he_rejects() {
    let mut player = hello_i_am_mordred();
    player.i_vote_for_mission_to(Vote::Reject);
    assert_eq!(player.did_i_vote_to_stay(), true);
}

#[test]
fn is_mordred_for_when_he_rejects() {
    let mut player = hello_i_am_mordred();
    player.i_vote_for_mission_to(Vote::Reject);
    assert_eq!(player.did_i_vote_to_go(), false);
}

#[test]
fn is_mordred_against_when_he_approves() {
    let mut player = hello_i_am_mordred();
    player.i_vote_for_mission_to(Vote::Approve);
    assert_eq!(player.did_i_vote_to_stay(), false);
}

#[test]
fn is_mordred_for_when_he_approves() {
    let mut player = hello_i_am_mordred();
    player.i_vote_for_mission_to(Vote::Approve);
    assert_eq!(player.did_i_vote_to_go(), false);
}
