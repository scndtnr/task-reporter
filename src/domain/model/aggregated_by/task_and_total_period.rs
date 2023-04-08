use std::{collections::HashMap, ops::Add};

use chrono::{DateTime, Duration, FixedOffset};

use crate::domain::model::{AsVec, DateRange, TaskRecord, TaskRecords};
use derive_new::new;

#[derive(new, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct TaskAndTotalPeriodRecord {
    updated_at: DateTime<FixedOffset>,
    charge_name: String,
    task_id: String,
    task_name: String,
    task_url: String,
    task_status: String,
    total_duration: Duration,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct TaskAndTotalPeriodRecords {
    date_range: DateRange,
    records: Vec<TaskAndTotalPeriodRecord>,
}

impl TaskAndTotalPeriodRecords {
    pub(crate) fn new(date_range: DateRange, task_records: TaskRecords) -> Self {
        let mut map = HashMap::<String, Vec<TaskRecord>>::new();

        for task_record in task_records.into_inner() {
            let key = task_record.task_id.clone();
            map.entry(key).or_default().push(task_record);
        }

        let mut aggregated_records = Vec::new();

        for (task_id, records) in map {
            let latest_record = records
                .iter()
                .max_by_key(|record| record.updated_at)
                .unwrap();
            let total_duration = records
                .iter()
                .map(|record| record.duration)
                .reduce(|total, duration| total.add(duration))
                .unwrap();
            aggregated_records.push(TaskAndTotalPeriodRecord::new(
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
