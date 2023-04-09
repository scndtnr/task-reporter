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

impl std::fmt::Display for TaskAndDailyRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}\t{}",
            self.target_date,
            self.updated_at.format("%Y/%m/%dT%H:%M:%S"),
            self.total_duration,
            self.charge_name,
            self.task_status,
            self.task_name,
        )
    }
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

impl std::fmt::Display for TaskAndDailyRecords {
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
                record.task_status.clone(),
                record.total_duration.clone(),
            )
        });

        let tsv = records.iter().map(|record| record.to_string()).fold(
            vec![vec![
                "target_date".to_string(),
                "updated_at".to_string(),
                "total_duration".to_string(),
                "charge_name".to_string(),
                "task_status".to_string(),
                "task_name".to_string(),
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
