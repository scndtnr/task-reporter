use crate::domain::model::{DateRange, Jst, TaskDuration, TaskRecord};

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
            task_id: task_id.into(),
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

impl From<ClickupTimeEntry> for TaskRecord {
    fn from(clickup_time_entry: ClickupTimeEntry) -> Self {
        Self {
            task_id: clickup_time_entry.task_id,
            task_name: clickup_time_entry.task_name,
            task_url: clickup_time_entry.task_url,
            task_status: clickup_time_entry.task_status,
            charge_name: clickup_time_entry.parent_list_name,
            duration: TaskDuration::from(clickup_time_entry.duration),
            // 開始時点の日時で対象日付を判定する
            target_date: DateRange::convert_datetime_to_date(clickup_time_entry.start),
            // 更新日時は終了時点のものを採用する
            updated_at: clickup_time_entry.end,
        }
    }
}
