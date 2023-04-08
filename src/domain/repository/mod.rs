mod clickup;

pub use clickup::{ClickupTaskRepository, ClickupTimeEntryRepository};

pub trait Repositories {
    type ClickupTaskRepo: ClickupTaskRepository;
    type ClickupTimeEntryRepo: ClickupTimeEntryRepository;

    fn clickup_task_repository(&self) -> &Self::ClickupTaskRepo;
    fn clickup_time_entry_repository(&self) -> &Self::ClickupTimeEntryRepo;
}
