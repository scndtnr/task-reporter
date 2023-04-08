use crate::domain::model::Jst;

use chrono::{DateTime, Duration, FixedOffset};

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub struct ClickupTimeEntry {
    pub id: String,
    pub duration: Duration,
    pub start: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub task_id: String,
    pub task_name: String,
    pub task_url: String,
    pub task_status: String,
    pub parent_list_name: String,
}

impl ClickupTimeEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        duration: &str,
        start: &str,
        end: &str,
        task_id: impl Into<String>,
        task_name: impl Into<String>,
        task_url: impl Into<String>,
        task_status: impl Into<String>,
        list_name: impl Into<String>,
    ) -> Self {
        ClickupTimeEntry {
            id: id.into(),
            duration: Duration::milliseconds(duration.parse::<i64>().unwrap()),
            start: Jst::timestamp_millis(start.parse::<i64>().unwrap()),
            end: Jst::timestamp_millis(end.parse::<i64>().unwrap()),
            task_id: task_id.into().into(),
            task_name: task_name.into(),
            task_url: task_url.into(),
            task_status: task_status.into(),
            parent_list_name: list_name.into(),
        }
    }
}

impl PartialEq for ClickupTimeEntry {
    fn eq(&self, other: &Self) -> bool {
        self.task_id == other.task_id
    }
}
