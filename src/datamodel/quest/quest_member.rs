use crate::datamodel::quest::quest_member_role_action_manager::QuestMemberRoleActionManager;

pub struct QuestMember {
    action_manager: QuestMemberRoleActionManager,
}

impl QuestMember {
    pub fn get_action_manager(&self) -> &QuestMemberRoleActionManager {
        &self.action_manager
    }
}
