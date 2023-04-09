use std::collections::HashMap;

use crate::domain::model::{AsVec, DateRange, TaskDuration, TaskRecord, TaskRecords};
use chrono::{DateTime, FixedOffset, NaiveDate};
use derive_new::new;

#[derive(new, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ChargeAndDailyRecord {
    target_date: NaiveDate,
    updated_at: DateTime<FixedOffset>,
    charge_name: String,
    total_duration: TaskDuration,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ChargeAndDailyRecords {
    date_range: DateRange,
    records: Vec<ChargeAndDailyRecord>,
}

impl ChargeAndDailyRecords {
    pub(crate) fn new(date_range: DateRange, task_records: TaskRecords) -> Self {
        let mut map = HashMap::<(String, chrono::NaiveDate), Vec<TaskRecord>>::new();

        for task_record in task_records.into_inner() {
            let key = (task_record.charge_name.clone(), task_record.target_date);
            map.entry(key).or_default().push(task_record);
        }

        let mut aggregated_records = Vec::new();

        for ((charge_name, target_date), records) in map {
            let latest_record = records
                .iter()
                .max_by_key(|record| record.updated_at)
                .unwrap();
            let total_duration = records
                .iter()
                .map(|record| record.duration.clone())
                .reduce(|total, duration| total.add(duration))
                .unwrap();
            aggregated_records.push(ChargeAndDailyRecord::new(
                target_date,
                latest_record.updated_at,
                latest_record.charge_name.clone(),
                total_duration,
            ))
        }
        Self {
            date_range,
            records: aggregated_records,
        }
    }
}
