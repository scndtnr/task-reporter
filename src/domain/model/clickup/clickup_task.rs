use crate::domain::model::{DateRange, Jst, TaskDuration, TaskRecord};

use chrono::{DateTime, FixedOffset};

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub struct ClickupTask {
    pub(crate) task_id: String,
    pub(crate) task_name: String,
    pub(crate) task_url: String,
    pub(crate) task_status: String,
    pub(crate) parent_list_name: String,
    pub(crate) duration: TaskDuration,
    pub(crate) updated_at: DateTime<FixedOffset>,
}

impl ClickupTask {
    pub fn new(
        task_id: impl Into<String>,
        task_name: impl Into<String>,
        task_url: impl Into<String>,
        task_status: impl Into<String>,
        list_name: impl Into<String>,
        duration: Option<&str>,
        updated_at: &str,
    ) -> Self {
        Self {
            task_id: task_id.into(),
            task_name: task_name.into(),
            task_url: task_url.into(),
            task_status: task_status.into(),
            parent_list_name: list_name.into(),
            duration: TaskDuration::from(duration),
            updated_at: Jst::timestamp_millis(updated_at.parse::<i64>().unwrap()),
        }
    }
}

impl PartialEq for ClickupTask {
    fn eq(&self, other: &Self) -> bool {
        self.task_id == other.task_id
    }
}

impl From<ClickupTask> for TaskRecord {
    fn from(clickup_task: ClickupTask) -> Self {
        Self {
            task_id: clickup_task.task_id,
            task_name: clickup_task.task_name,
            task_url: clickup_task.task_url,
            task_status: clickup_task.task_status,
            charge_name: clickup_task.parent_list_name,
            duration: clickup_task.duration,
            target_date: DateRange::convert_datetime_to_date(clickup_task.updated_at),
            updated_at: clickup_task.updated_at,
        }
    }
}
