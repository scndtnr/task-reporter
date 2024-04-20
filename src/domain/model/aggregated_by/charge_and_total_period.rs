use std::collections::HashMap;

use chrono::{DateTime, FixedOffset};

use crate::domain::model::{AsVec, DateRange, TaskDuration, TaskRecord, TaskRecords};
use derive_new::new;

#[derive(new, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct ChargeAndTotalPeriodRecord {
    updated_at: DateTime<FixedOffset>,
    total_duration: TaskDuration,
    charge_name: String,
}

impl std::fmt::Display for ChargeAndTotalPeriodRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}",
            self.updated_at.format("%Y/%m/%d %H:%M:%S"),
            self.total_duration,
            self.charge_name,
        )
    }
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

impl std::fmt::Display for ChargeAndTotalPeriodRecords {
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
        records.sort_by_key(|record| (record.charge_name.clone(), record.total_duration.clone()));

        let mut tsv = records.iter().map(|record| record.to_string()).fold(
            vec![vec![
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
        let grand_total = format!(
            "総合計\t{}",
            records
                .into_iter()
                .map(|record| record.total_duration)
                .fold(TaskDuration::new(), |accum, duration| accum.add(duration))
        );
        tsv.push(grand_total);
        write!(f, "\n{}\n[\n{}\n]", title, tsv.join("\n"))
    }
}
