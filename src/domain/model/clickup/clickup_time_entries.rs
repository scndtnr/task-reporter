use super::ClickupTimeEntry;
use crate::domain::model::{AsVec, TaskRecords};

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
}
