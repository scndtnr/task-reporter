mod clickup;

pub(crate) use clickup::ClickupRepositoryImpl;

use crate::domain::{
    model::clickup::{ClickupTask, ClickupTimeEntry},
    repository::Repositories,
};
use crate::infra::web::BasicClient;

#[derive(Debug, Clone)]
pub(crate) struct RepositoryImpls {
    clickup_task_repository: ClickupRepositoryImpl<ClickupTask>,
    clickup_time_entry_repository: ClickupRepositoryImpl<ClickupTimeEntry>,
}

impl Repositories for RepositoryImpls {
    type ClickupTaskRepo = ClickupRepositoryImpl<ClickupTask>;
    type ClickupTimeEntryRepo = ClickupRepositoryImpl<ClickupTimeEntry>;

    fn clickup_task_repository(&self) -> &Self::ClickupTaskRepo {
        &self.clickup_task_repository
    }
    fn clickup_time_entry_repository(&self) -> &Self::ClickupTimeEntryRepo {
        &self.clickup_time_entry_repository
    }
}

impl RepositoryImpls {
    pub(crate) fn new(client: BasicClient) -> Self {
        // 各リポジトリのインスタンスを生成する
        let clickup_task_repository = ClickupRepositoryImpl::new(client.clone());
        let clickup_time_entry_repository = ClickupRepositoryImpl::new(client);
        Self {
            clickup_task_repository,
            clickup_time_entry_repository,
        }
    }
}
