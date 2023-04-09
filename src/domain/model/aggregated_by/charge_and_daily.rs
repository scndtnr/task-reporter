use std::collections::HashMap;

use crate::domain::model::{AsVec, DateRange, TaskDuration, TaskRecord, TaskRecords};
use chrono::{DateTime, FixedOffset, NaiveDate};
use derive_new::new;

#[derive(new, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ChargeAndDailyRecord {
    target_date: NaiveDate,
    updated_at: DateTime<FixedOffset>,
    total_duration: TaskDuration,
    charge_name: String,
}

impl std::fmt::Display for ChargeAndDailyRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}",
            self.target_date,
            self.updated_at.format("%Y/%m/%dT%H:%M:%S"),
            self.total_duration,
            self.charge_name,
        )
    }
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
                total_duration,
                charge_name,
            ))
        }
        Self {
            date_range,
            records: aggregated_records,
        }
    }
}

impl std::fmt::Display for ChargeAndDailyRecords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = if self.date_range.is_same_date() {
            format!("集計対象日付：{}", self.date_range.start_date_str())
        } else {
            format!(
                "集計対象期間：{} ～ {}",
                self.date_range.start_date_str(),
                self.date_range.end_target_date_str()
            )
        };

        // 並び替える
        let mut records = self.records.clone();
        records.sort_by_key(|record| {
            (
                record.target_date,
                record.charge_name.clone(),
                record.total_duration.clone(),
            )
        });

        let tsv = records.iter().map(|record| record.to_string()).fold(
            vec![vec![
                "target_date".to_string(),
                "updated_at".to_string(),
                "total_duration".to_string(),
                "charge_name".to_string(),
            ]
            .join("\t")],
            |mut records, record| {
                records.push(record);
                records
            },
        );
        write!(f, "\n{}\n[\n{}\n]", title, tsv.join("\n"))
    }
}
