mod options;

use super::adapter::dto::RequestDto;
use crate::{
    adapter::Controller,
    cui::options::AggregateCondition,
    infra::{repository_impl::RepositoryImpls, web::BasicClient},
    usecase::UsecaseImpls,
};
use clap::Parser;
use clipboard_win::{formats, set_clipboard};
pub(super) use options::Opts;

#[derive(Debug, Clone)]
pub(super) struct Cui {
    controller: Controller<UsecaseImpls>,
    opts: Opts,
}

impl Cui {
    pub(super) async fn new() -> Self {
        let client = BasicClient::new();
        let repositories = RepositoryImpls::new(client);
        let usecases = UsecaseImpls::new(repositories);
        let controller = Controller::new(usecases).await;
        Self {
            controller,
            opts: Opts::parse(),
        }
    }

    pub(super) async fn process(&self) {
        tracing::debug!("cli args: {:#?}", self.opts);
        let dto: RequestDto = self.opts.clone().into();
        let result = match self.opts.clone().into() {
            AggregateCondition::TaskAndTotalPeriod => self.by_task_and_total_period(dto).await,
            AggregateCondition::ChargeAndTotalPeriod => self.by_charge_and_total_period(dto).await,
            AggregateCondition::TaskAndDaily => self.by_task_and_daily(dto).await,
            AggregateCondition::ChargeAndDaily => self.by_charge_and_daily(dto).await,
        };

        if self.opts.set_clipboard() {
            // クリップボードにコピーする（Windows限定）
            set_clipboard(formats::Unicode, result.to_string()).expect("Fail to set clipboard.");
        }

        // ログ出力する
        tracing::info!("{}", result);
    }

    pub(super) async fn by_task_and_total_period(&self, dto: RequestDto) -> String {
        tracing::debug!("by_task_and_total_period");
        self.controller
            .aggregate_by_task_and_total_period(dto)
            .await
            .to_string()
    }

    pub(super) async fn by_task_and_daily(&self, dto: RequestDto) -> String {
        tracing::debug!("by_task_and_daily");
        self.controller
            .aggregate_by_task_and_daily(dto)
            .await
            .to_string()
    }

    pub(super) async fn by_charge_and_total_period(&self, dto: RequestDto) -> String {
        tracing::debug!("by_charge_and_total_period");
        self.controller
            .aggregate_by_charge_and_total_period(dto)
            .await
            .to_string()
    }

    pub(super) async fn by_charge_and_daily(&self, dto: RequestDto) -> String {
        tracing::debug!("by_charge_and_daily");
        self.controller
            .aggregate_by_charge_and_daily(dto)
            .await
            .to_string()
    }
}
