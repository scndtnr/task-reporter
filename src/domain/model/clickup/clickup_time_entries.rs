extern crate chrono;

use crate::domain::model::{AsVec, DateRange, TaskDuration, TaskRecords};
use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;

use super::{ClickupTask, ClickupTasks, ClickupTimeEntry};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClickupTimeEntries(Vec<ClickupTimeEntry>);

impl AsVec for ClickupTimeEntries {
    type Item = ClickupTimeEntry;
    fn into_inner(self) -> Vec<Self::Item> {
        self.0
    }
    fn as_vec(&self) -> &Vec<Self::Item> {
        &self.0
    }
    fn as_mut_vec(&mut self) -> &mut Vec<Self::Item> {
        &mut self.0
    }
}

impl From<ClickupTimeEntries> for TaskRecords {
    fn from(clickup_time_entries: ClickupTimeEntries) -> Self {
        Self::new(
            clickup_time_entries
                .into_inner()
                .into_iter()
                .map(|entry| entry.into())
                .collect(),
        )
    }
}

impl ClickupTimeEntries {
    pub fn new(time_entries: Vec<ClickupTimeEntry>) -> Self {
        Self(time_entries)
    }

    /// 記録開始時点の日付を対象として絞り込む
    pub fn filter_entries_by_start_with_date_range(&self, dr: &DateRange) -> Self {
        let rule = |te: &ClickupTimeEntry| dr.start() <= te.start && te.start <= dr.end();
        Self(
            self.0
                .clone()
                .into_iter()
                .filter(rule)
                .collect::<Vec<ClickupTimeEntry>>(),
        )
    }

    /// タスクIDをキーとして集約しタスクに変換する
    pub fn to_tasks(&self) -> ClickupTasks {
        let mut aggregated_tasks = Vec::new();
        for (_, group) in self.group_by_task_id() {
            let (task_id, task_name, task_url, task_status, parent_list_name, updated_at) =
                self.extract_task_attibute_at_end(&group);
            let duration = self.sum_duration(&group);
            let task = ClickupTask {
                task_id,
                task_name,
                task_url,
                task_status,
                parent_list_name,
                duration,
                updated_at,
            };
            aggregated_tasks.push(task);
        }
        ClickupTasks::new(aggregated_tasks)
    }

    fn group_by_task_id(&self) -> HashMap<String, Vec<ClickupTimeEntry>> {
        let mut groups = HashMap::new();
        for te in self.0.clone().into_iter() {
            groups
                .entry(te.task_id.to_string())
                .or_insert_with(Vec::new)
                .push(te);
        }
        groups
    }
    fn extract_task_attibute_at_end(
        &self,
        te_group: &[ClickupTimeEntry],
    ) -> (
        String,
        String,
        String,
        String,
        String,
        DateTime<FixedOffset>,
    ) {
        let te = te_group.iter().max_by_key(|te| te.end).unwrap();
        (
            te.task_id.clone(),
            te.task_name.clone(),
            te.task_url.clone(),
            te.task_status.clone(),
            te.parent_list_name.clone(),
            te.end,
        )
    }
    fn sum_duration(&self, te_group: &[ClickupTimeEntry]) -> TaskDuration {
        te_group
            .iter()
            .map(|te| TaskDuration::new(te.duration))
            .reduce(|accum, duration| accum.add(duration))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::model::AsVec;

    use super::*;

    fn gen_time_entries() -> ClickupTimeEntries {
        let tes = vec![
            ClickupTimeEntry::new(
                "A-A",
                "1000",          // 1s
                "1343696400000", // 2012-07-31 10:00:00
                "1343696401000", // 2012-07-31 10:00:01
                "A",
                "task_A",
                "task_url_A",
                "Running",
                "list_name",
            ),
            ClickupTimeEntry::new(
                "A-B",
                "20000",         // 20s
                "1343714400000", // 2012-07-31 15:00:00
                "1343714420000", // 2012-07-31 15:00:20
                "A",
                "task_A",
                "task_url_A",
                "Running",
                "list_name",
            ),
            ClickupTimeEntry::new(
                "B-C",
                "180000",        // 180s = 3min
                "1343718000000", // 2012-07-31 16:00:00
                "1343718180000", // 2012-07-31 16:03:00
                "B",
                "task_B",
                "task_url_B",
                "Running",
                "list_name",
            ),
            ClickupTimeEntry::new(
                "B-D",
                "1800000",       // 1800s = 30min
                "1343764200000", // 2012-08-01 04:50:00
                "1343766000000", // 2012-08-01 05:20:00
                "B",
                "task_B",
                "task_url_B",
                "Running",
                "list_name",
            ),
            ClickupTimeEntry::new(
                "B-E",
                "2400000",       // 2400s = 40min
                "1343808000000", // 2012-08-01 17:00:00
                "1343810400000", // 2012-08-01 17:40:00
                "B",
                "task_B",
                "task_url_B",
                "Running",
                "list_name",
            ),
            ClickupTimeEntry::new(
                "C-F",
                "3600000",       // 3600s = 60min
                "1343725200000", // 2012-07-31 18:00:00
                "1343728800000", // 2012-07-31 19:00:00
                "C",
                "task_C",
                "task_url_C",
                "Running",
                "list_name",
            ),
        ];
        ClickupTimeEntries(tes)
    }

    #[test]
    fn check_filter_entries_by_start_with_date_range() {
        let tes = gen_time_entries();
        let dr = DateRange::new(Some("2012/07/31"), None);
        let mut filterd_tes = tes.filter_entries_by_start_with_date_range(&dr);

        // 順番を固定する
        filterd_tes.0.sort();

        // テスト
        // B-Dパターン（2012/07/31扱い）が含まれていて、
        // B-Eパターン（2012/08/01扱い）が弾かれていることを確認する
        let start_a_a = "2012-07-31 10:00:00 +09:00";
        let end_a_a = "2012-07-31 10:00:01 +09:00";
        assert_eq!(filterd_tes.as_vec().get(0).unwrap().id.as_str(), "A-A");
        assert_eq!(
            filterd_tes.as_vec().get(0).unwrap().start.to_string(),
            start_a_a
        );
        assert_eq!(
            filterd_tes.as_vec().get(0).unwrap().end.to_string(),
            end_a_a
        );

        let start_a_b = "2012-07-31 15:00:00 +09:00";
        let end_a_b = "2012-07-31 15:00:20 +09:00";
        assert_eq!(filterd_tes.as_vec().get(1).unwrap().id.as_str(), "A-B");
        assert_eq!(
            filterd_tes.as_vec().get(1).unwrap().start.to_string(),
            start_a_b
        );
        assert_eq!(
            filterd_tes.as_vec().get(1).unwrap().end.to_string(),
            end_a_b
        );

        let start_b_c = "2012-07-31 16:00:00 +09:00";
        let end_b_c = "2012-07-31 16:03:00 +09:00";
        assert_eq!(filterd_tes.as_vec().get(2).unwrap().id.as_str(), "B-C");
        assert_eq!(
            filterd_tes.as_vec().get(2).unwrap().start.to_string(),
            start_b_c
        );
        assert_eq!(
            filterd_tes.as_vec().get(2).unwrap().end.to_string(),
            end_b_c
        );

        let start_b_d = "2012-08-01 04:50:00 +09:00";
        let end_b_d = "2012-08-01 05:20:00 +09:00";
        assert_eq!(filterd_tes.as_vec().get(3).unwrap().id.as_str(), "B-D");
        assert_eq!(
            filterd_tes.as_vec().get(3).unwrap().start.to_string(),
            start_b_d
        );
        assert_eq!(
            filterd_tes.as_vec().get(3).unwrap().end.to_string(),
            end_b_d
        );

        let start_c_f = "2012-07-31 18:00:00 +09:00";
        let end_c_f = "2012-07-31 19:00:00 +09:00";
        assert_eq!(filterd_tes.as_vec().get(4).unwrap().id.as_str(), "C-F");
        assert_eq!(
            filterd_tes.as_vec().get(4).unwrap().start.to_string(),
            start_c_f
        );
        assert_eq!(
            filterd_tes.as_vec().get(4).unwrap().end.to_string(),
            end_c_f
        );

        assert_eq!(filterd_tes.as_vec().get(5), None);
    }

    #[test]
    fn check_aggregate_by_task_id() {
        let tes = gen_time_entries();
        let mut tasks = tes.to_tasks();

        // 順番を固定する
        // 同一タスクIDは存在せず、
        // 更新日時がendの最大値であること、
        // durationが合計値であることを確認する。
        tasks.as_mut_vec().sort_by_key(|t| t.updated_at);

        // テスト
        let updated_at_a = "2012-07-31 15:00:20 +09:00";
        assert_eq!(tasks.as_vec()[0].task_id.as_str(), "A");
        assert_eq!(tasks.as_vec()[0].updated_at.to_string(), updated_at_a);
        assert_eq!(tasks.as_vec()[0].duration.as_duration().num_seconds(), 21);

        let updated_at_c = "2012-07-31 19:00:00 +09:00";
        assert_eq!(tasks.as_vec()[1].task_id.as_str(), "C");
        assert_eq!(tasks.as_vec()[1].updated_at.to_string(), updated_at_c);
        assert_eq!(tasks.as_vec()[1].duration.as_duration().num_seconds(), 3600);

        let updated_at_b = "2012-08-01 17:40:00 +09:00";
        assert_eq!(tasks.as_vec()[2].task_id.as_str(), "B");
        assert_eq!(tasks.as_vec()[2].updated_at.to_string(), updated_at_b);
        assert_eq!(tasks.as_vec()[2].duration.as_duration().num_seconds(), 4380);

        assert_eq!(tasks.as_vec().get(3), None);
    }
}
