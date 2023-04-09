use std::collections::HashMap;

use chrono::{DateTime, FixedOffset, NaiveDate};

use crate::domain::model::{AsVec, DateRange, TaskDuration, TaskRecord, TaskRecords};
use derive_new::new;

#[derive(new, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct TaskAndDailyRecord {
    target_date: NaiveDate,
    updated_at: DateTime<FixedOffset>,
    charge_name: String,
    task_id: String,
    task_name: String,
    task_url: String,
    task_status: String,
    total_duration: TaskDuration,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct TaskAndDailyRecords {
    date_range: DateRange,
    records: Vec<TaskAndDailyRecord>,
}

impl TaskAndDailyRecords {
    pub(crate) fn new(date_range: DateRange, task_records: TaskRecords) -> Self {
        let mut map = HashMap::<(String, chrono::NaiveDate), Vec<TaskRecord>>::new();

        for task_record in task_records.into_inner() {
            let key = (task_record.task_id.clone(), task_record.target_date);
            map.entry(key).or_default().push(task_record);
        }

        let mut aggregated_records = Vec::new();

        for ((task_id, target_date), records) in map {
            let latest_record = records
                .iter()
                .max_by_key(|record| record.updated_at)
                .unwrap();
            let total_duration = records
                .iter()
                .map(|record| record.duration.clone())
                .reduce(|total, duration| total.add(duration))
                .unwrap();
            aggregated_records.push(TaskAndDailyRecord::new(
                target_date,
                latest_record.updated_at,
                latest_record.charge_name.clone(),
                task_id,
                latest_record.task_name.clone(),
                latest_record.task_url.clone(),
                latest_record.task_status.clone(),
                total_duration,
            ))
        }
        Self {
            date_range,
            records: aggregated_records,
        }
    }
}