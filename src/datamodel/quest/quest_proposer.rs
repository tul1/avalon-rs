use crate::datamodel::quest::quest_proposer_role_action_manager::QuestProposerRoleActionManager;

pub struct QuestProposer {
    action_manager: QuestProposerRoleActionManager,
}

impl QuestProposer {
    pub fn get_action_manager(&self) -> &QuestProposerRoleActionManager {
        &self.action_manager
    }
}
