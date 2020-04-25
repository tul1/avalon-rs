use crate::datamodel::quests::quest_new::*;

#[test]
fn test_quest_state_sequence_nominal_successes() {
    let quest_members = [String::from("jimi"), String::from("pato"), String::from("volan")];
    let votes = [Vote::Success, Vote::Success, Vote::Failed];
    let winner_rule = (Vote::Failed, 2);
    let mut quest = QuestNew::new(&quest_members, &winner_rule);
    for (index, voter) in quest_members.iter().enumerate() {
        quest.vote(&voter, votes[index]);
    }
    let quest_result = quest.finish_quest();
    assert_eq!(*quest_result.result(), Vote::Success);

}
