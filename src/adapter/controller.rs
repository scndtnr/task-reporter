use super::dto::{ByTaskAndTotalDtos, RequestDto};

#[derive(Debug, Clone)]
pub(crate) struct Controller;

impl Controller {
    pub(crate) async fn new() -> Self {
        Self
    }

    pub(crate) async fn aggregate_by_task_and_total_period(
        &self,
        dto: RequestDto,
    ) -> ByTaskAndTotalDtos {
        todo!();
    }

    pub(crate) async fn aggregate_by_charge_and_total_period(&self, dto: RequestDto) {
        todo!();
    }

    pub(crate) async fn aggregate_by_task_and_daily(&self, dto: RequestDto) {
        todo!();
    }

    pub(crate) async fn aggregate_by_charge_and_daily(&self, dto: RequestDto) {
        todo!();
    }
}
