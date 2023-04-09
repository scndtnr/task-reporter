use crate::{domain::model::aggregated_by::TaskAndTotalPeriodRecords, usecase::Usecases};

use super::dto::RequestDto;

#[derive(Debug, Clone)]
pub(crate) struct Controller<U> {
    usecases: U,
}

impl<'a, U: Usecases> Controller<U> {
    pub(crate) async fn new(usecases: U) -> Self {
        Self { usecases }
    }

    pub(crate) async fn aggregate_by_task_and_total_period(
        &self,
        dto: RequestDto,
    ) -> TaskAndTotalPeriodRecords {
        self.usecases
            .aggregate_duration_use_case()
            .by_task_and_total_period(dto.start_date().clone(), dto.end_date().clone())
            .await
            .expect("Failed to process AggregateDurationUsecase: by_task_and_total_period")
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
