use super::{task_record, AsVec, TaskRecord};

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct TaskRecords(Vec<TaskRecord>);

impl AsVec for TaskRecords {
    type Item = TaskRecord;
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

impl TaskRecords {
    pub(crate) fn new(task_records: Vec<TaskRecord>) -> Self {
        Self(task_records)
    }

    pub(crate) fn concat(&self, other: &Self) -> Self {
        let mut records = self.0.clone();
        records.extend_from_slice(&other.0);
        Self::new(records)
    }
}
