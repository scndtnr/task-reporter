extern crate anyhow;
extern crate async_trait;

use crate::domain::model::clickup::{ClickupTasks, ClickupTimeEntries};
use crate::domain::model::DateRange;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ClickupTaskRepository {
    async fn find_tasks_by_date_range(&self, date_range: &DateRange) -> Result<ClickupTasks>;
    async fn tasks_pagination(&self, date_range: &DateRange) -> Result<ClickupTasks>;
}
#[async_trait]
pub trait ClickupTimeEntryRepository {
    async fn find_time_entries_by_date_range(
        &self,
        date_range: &DateRange,
    ) -> Result<ClickupTimeEntries>;
}
