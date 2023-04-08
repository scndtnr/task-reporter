use crate::domain::{
    model::{clickup::ClickupTasks, DateRange},
    repository::{ClickupTaskRepository, ClickupTimeEntryRepository, Repositories},
};
use anyhow::Result;
use derive_new::new;

#[derive(new, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct AggregateDurationUseCase<R: Repositories> {
    task_repo: R::ClickupTaskRepo,
    time_entry_repo: R::ClickupTimeEntryRepo,
}

impl<R: Repositories> AggregateDurationUseCase<R> {
    #[tracing::instrument(skip_all)]
    pub(crate) async fn by_task_and_total_period<T: Into<String>>(
        &self,
        start_date: Option<T>,
        end_date: Option<T>,
    ) -> Result<ClickupTasks> {
        let date_range = DateRange::new(start_date, end_date);
        let mut tasks = self.task_repo.find_tasks_by_date_range(&date_range).await?;
        let time_entries = self
            .time_entry_repo
            .find_time_entries_by_date_range(&date_range)
            .await?;

        tracing::debug!("{:#?}", tasks);
        tracing::debug!("{:#?}", time_entries);

        Ok(tasks.merge_tasks(&mut time_entries.to_tasks()))
    }
}
