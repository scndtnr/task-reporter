extern crate chrono;

use super::values::ClickupDuration;
use super::ClickupTask;
use crate::domain::model::{task_records::TaskRecords, AsVec};
use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClickupTasks(Vec<ClickupTask>);

impl AsVec for ClickupTasks {
    type Item = ClickupTask;
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

impl From<ClickupTasks> for TaskRecords {
    fn from(clickup_tasks: ClickupTasks) -> Self {
        Self::new(
            clickup_tasks
                .into_inner()
                .into_iter()
                .map(|task| task.into())
                .collect(),
        )
    }
}

impl ClickupTasks {
    pub fn new(tasks: Vec<ClickupTask>) -> Self {
        Self(tasks)
    }

    pub fn merge_tasks(&mut self, other: &mut ClickupTasks) -> Self {
        // 返り値用の受け皿を用意する
        let mut merged_tasks = Vec::new();

        // タスクのリストを連結する
        self.0.append(&mut other.0);

        // タスクID毎のリストを作成し、ループ処理をする
        for (_, tasks) in self.group_by_task_id() {
            // 要素数が1であれば重複はないので、そのまま返り値に入れる
            if tasks.len() == 1 {
                merged_tasks.push(tasks[0].clone());
                continue;
            }

            // 要素数が2以上の場合は、最新の値や合計値を返す
            let (task_id, task_name, task_url, task_status, parent_list_name, updated_at) =
                self.extract_task_attibute_at_end(&tasks);
            let duration = self.sum_duration(&tasks);
            merged_tasks.push(ClickupTask {
                task_id,
                task_name,
                task_url,
                task_status,
                parent_list_name,
                updated_at,
                duration,
            })
        }
        ClickupTasks(merged_tasks)
    }

    fn group_by_task_id(&self) -> HashMap<String, Vec<ClickupTask>> {
        let mut task_dict = HashMap::new();
        for task in self.0.clone().into_iter() {
            task_dict
                .entry(task.task_id.to_string())
                .or_insert_with(Vec::new)
                .push(task);
        }
        task_dict
    }

    fn extract_task_attibute_at_end(
        &self,
        tasks: &[ClickupTask],
    ) -> (
        String,
        String,
        String,
        String,
        String,
        DateTime<FixedOffset>,
    ) {
        let t = tasks.iter().max_by_key(|t| t.updated_at).unwrap();
        (
            t.task_id.clone(),
            t.task_name.clone(),
            t.task_url.clone(),
            t.task_status.clone(),
            t.parent_list_name.clone(),
            t.updated_at,
        )
    }

    fn sum_duration(&self, tasks: &[ClickupTask]) -> ClickupDuration {
        tasks
            .iter()
            .map(|task| task.duration.clone())
            .fold(ClickupDuration::new(None), |accum, duration| {
                accum.add(duration)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::super::{ClickupTimeEntries, ClickupTimeEntry};
    use super::*;
    use crate::domain::model::DateRange;

    fn gen_tasks() -> ClickupTasks {
        let ts = vec![
            ClickupTask::new(
                "A",
                "task_A",
                "task_url_A",
                "Running",
                "list_name",
                Some("3600000"), // 3600s = 60min
                "1343718000000", // 2012-07-31 16:00:00
            ),
            ClickupTask::new(
                "C",
                "task_C",
                "task_url_C",
                "Complete",
                "list_name",
                None,
                "1343732400000", // 2012-07-31 20:00:00
            ),
        ];
        ClickupTasks(ts)
    }

    fn gen_time_entries() -> ClickupTimeEntries {
        let time_entries = vec![
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
        ClickupTimeEntries::new(time_entries)
    }

    #[test]
    fn merge_tasks_and_time_entries() {
        let mut tasks = gen_tasks();
        let time_entries = gen_time_entries();
        let dr = DateRange::new(Some("2012/07/31"), None);

        let mut other_tasks = time_entries
            .filter_entries_by_start_with_date_range(&dr)
            .to_tasks();

        let mut merged_tasks = tasks.merge_tasks(&mut other_tasks);
        merged_tasks.as_mut_vec().sort();

        let update_at_a = "2012-07-31 16:00:00 +09:00";
        assert_eq!(merged_tasks.as_vec()[0].task_id.as_str(), "A");
        assert_eq!(merged_tasks.as_vec()[0].task_status, "Running");
        assert_eq!(merged_tasks.as_vec()[0].updated_at.to_string(), update_at_a);
        assert_eq!(
            merged_tasks.as_vec()[0]
                .duration
                .as_duration()
                .num_seconds(),
            3621
        );

        let update_at_b = "2012-08-01 05:20:00 +09:00";
        assert_eq!(merged_tasks.as_vec()[1].task_id.as_str(), "B");
        assert_eq!(merged_tasks.as_vec()[1].task_status, "Running");
        assert_eq!(merged_tasks.as_vec()[1].updated_at.to_string(), update_at_b);
        assert_eq!(
            merged_tasks.as_vec()[1]
                .duration
                .as_duration()
                .num_seconds(),
            1980
        );

        let update_at_c = "2012-07-31 20:00:00 +09:00";
        assert_eq!(merged_tasks.as_vec()[2].task_id.as_str(), "C");
        assert_eq!(merged_tasks.as_vec()[2].task_status, "Complete");
        assert_eq!(merged_tasks.as_vec()[2].updated_at.to_string(), update_at_c);
        assert_eq!(
            merged_tasks.as_vec()[2]
                .duration
                .as_duration()
                .num_seconds(),
            3600
        );

        // for t in merged_tasks.0 {
        //     println!("{}", t.task_id.id);
        //     println!("{}", t.task_status.0);
        //     println!("{}", t.updated_at);
        //     println!("{}", t.duration.unwrap().num_seconds());
        // }
    }
}
