mod options;

use super::adapter::dto::RequestDto;
use crate::{adapter::Controller, cui::options::AggregateCondition};
use clap::Parser;
pub(super) use options::Opts;

#[derive(Debug, Clone)]
pub(super) struct Cui {
    controller: Controller,
    opts: Opts,
}

impl Cui {
    pub(super) async fn new() -> Self {
        Self {
            controller: Controller::new().await,
            opts: Opts::parse(),
        }
    }

    pub(super) async fn process(&self) {
        tracing::debug!("cli args: {:#?}", self.opts);
        let dto: RequestDto = self.opts.clone().into();
        match self.opts.clone().into() {
            AggregateCondition::TaskAndTotalPeriod => self.by_task_and_total_period(dto).await,
            AggregateCondition::ChargeAndTotalPeriod => self.by_charge_and_total_period(dto).await,
            AggregateCondition::TaskAndDaily => self.by_task_and_daily(dto).await,
            AggregateCondition::ChargeAndDaily => self.by_charge_and_daily(dto).await,
        }
    }

    pub(super) async fn by_task_and_total_period(&self, dto: RequestDto) {
        tracing::debug!("by_task_and_total_period");
        let dto = self
            .controller
            .aggregate_by_task_and_total_period(dto)
            .await;
        todo!();
    }

    pub(super) async fn by_charge_and_total_period(&self, dto: RequestDto) {
        tracing::debug!("by_charge_and_total_period");
        todo!();
    }

    pub(super) async fn by_task_and_daily(&self, dto: RequestDto) {
        tracing::debug!("by_task_and_daily");
        todo!();
    }

    pub(super) async fn by_charge_and_daily(&self, dto: RequestDto) {
        tracing::debug!("by_charge_and_daily");
        todo!();
    }
}
