extern crate chrono;

use super::ClickupTask;
use crate::domain::model::{AsVec, TaskRecords};

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
}
