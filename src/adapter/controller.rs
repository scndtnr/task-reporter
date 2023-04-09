use crate::{
    domain::model::aggregated_by::{
        ChargeAndDailyRecords, ChargeAndTotalPeriodRecords, TaskAndDailyRecords,
        TaskAndTotalPeriodRecords,
    },
    usecase::Usecases,
};

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

    pub(crate) async fn aggregate_by_task_and_daily(&self, dto: RequestDto) -> TaskAndDailyRecords {
        self.usecases
            .aggregate_duration_use_case()
            .by_task_and_daily(dto.start_date().clone(), dto.end_date().clone())
            .await
            .expect("Failed to process AggregateDurationUsecase: by_task_and_daily")
    }

    pub(crate) async fn aggregate_by_charge_and_total_period(
        &self,
        dto: RequestDto,
    ) -> ChargeAndTotalPeriodRecords {
        self.usecases
            .aggregate_duration_use_case()
            .by_charge_and_total_period(dto.start_date().clone(), dto.end_date().clone())
            .await
            .expect("Failed to process AggregateDurationUsecase: by_charge_and_total_period")
    }

    pub(crate) async fn aggregate_by_charge_and_daily(
        &self,
        dto: RequestDto,
    ) -> ChargeAndDailyRecords {
        self.usecases
            .aggregate_duration_use_case()
            .by_charge_and_daily(dto.start_date().clone(), dto.end_date().clone())
            .await
            .expect("Failed to process AggregateDurationUsecase: by_charge_and_daily")
    }
}
