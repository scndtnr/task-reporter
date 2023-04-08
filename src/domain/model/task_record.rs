use chrono::{DateTime, Duration, FixedOffset, NaiveDate};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct TaskRecord {
    pub(crate) task_id: String,
    pub(crate) task_name: String,
    pub(crate) task_url: String,
    pub(crate) task_status: String,
    pub(crate) charge_name: String,
    pub(crate) duration: Duration,
    pub(crate) target_date: NaiveDate,
    pub(crate) updated_at: DateTime<FixedOffset>,
}
