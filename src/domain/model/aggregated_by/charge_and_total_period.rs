use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};

use crate::domain::model::{AsVec, DateRange, TaskDuration, TaskRecord, TaskRecords};
use derive_new::new;

#[derive(new, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ChargeAndTotalPeriodRecord {
    updated_at: DateTime<FixedOffset>,
    charge_name: String,
    total_duration: TaskDuration,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ChargeAndTotalPeriodRecords {
    date_range: DateRange,
    records: Vec<ChargeAndTotalPeriodRecord>,
}

impl ChargeAndTotalPeriodRecords {
    pub(crate) fn new(date_range: DateRange, task_records: TaskRecords) -> Self {
        let mut map = HashMap::<String, Vec<TaskRecord>>::new();

        for task_record in task_records.into_inner() {
            let key = task_record.charge_name.clone();
            map.entry(key).or_default().push(task_record);
        }

        let mut aggregated_records = Vec::new();

        for (charge_name, records) in map {
            let latest_record = records
                .iter()
                .max_by_key(|record| record.updated_at)
                .unwrap();
            let total_duration = records
                .iter()
                .map(|record| record.duration.clone())
                .reduce(|total, duration| total.add(duration))
                .unwrap();
            aggregated_records.push(ChargeAndTotalPeriodRecord::new(
                latest_record.updated_at,
                charge_name,
                total_duration,
            ))
        }
        Self {
            date_range,
            records: aggregated_records,
        }
    }
}
