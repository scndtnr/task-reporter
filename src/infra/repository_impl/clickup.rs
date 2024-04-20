use crate::domain::{
    model::clickup::{ClickupTask, ClickupTasks, ClickupTimeEntries, ClickupTimeEntry},
    model::DateRange,
    repository::{ClickupTaskRepository, ClickupTimeEntryRepository},
};
use crate::infra::web::{
    clickup::{ClickupClient, ParseClickupResponse},
    BasicClient, BasicResponse,
};
use anyhow::{bail, Result};
use async_trait::async_trait;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub(crate) struct ClickupRepositoryImpl<T> {
    client: ClickupClient,
    _marker: PhantomData<fn() -> T>,
}

impl<T> ClickupRepositoryImpl<T> {
    pub(crate) fn new(client: BasicClient) -> Self {
        Self {
            client: ClickupClient::new(client),
            _marker: PhantomData,
        }
    }
}

/// ClickUp API v2 の 日時範囲指定パラメータ start は、
/// linuxtime(ミリ秒) が一致するデータは取得しない挙動をしている。
/// つまり「start <= target」 ではなく 「start < target」と思われる
/// 本CLIではstartが一致するデータも取得したいので、パラメータから1ミリ秒引いておく
/// https://clickup.com/api/clickupreference/operation/Gettimeentrieswithinadaterange/
fn clickup_date_start_milles(date_range: &DateRange) -> i64 {
    date_range.start().timestamp_millis() - 1
}

#[async_trait]
impl ClickupTaskRepository for ClickupRepositoryImpl<ClickupTask> {
    #[tracing::instrument(
        name="find_tasks",
        level="debug",
        skip_all,
        fields(
            start=%date_range.start_datetime_str(),
            end=%date_range.end_datetime_str()
        )
    )]
    async fn find_tasks_by_date_range(&self, date_range: &DateRange) -> Result<ClickupTasks> {
        self.tasks_pagination(date_range).await
    }

    async fn tasks_pagination(&self, date_range: &DateRange) -> Result<ClickupTasks> {
        let per_page = 100;
        let mut is_not_last_page = true;
        let mut page = 0;
        let mut clickup_tasks = Vec::new();

        while is_not_last_page {
            let params = self.client.params().filterd_team_tasks(
                page,
                true,
                true,
                clickup_date_start_milles(date_range),
                date_range.end().timestamp_millis(),
            );
            let resp = self.client.api().filtered_team_tasks(Some(params)).await;

            tracing::trace!(
                "\nCurrent page is: {}. \nAPI limit Remaining: {}.\nAPI limit will reset, after {} seconds.",
                page,
                resp.0.x_ratelimit_remaining(),
                resp.0.x_ratelimit_reset_from_now(),
            );

            match resp.try_to_clickup_tasks() {
                Ok(mut current_tasks) => {
                    // 最終ページかどうか判定する
                    is_not_last_page = current_tasks.tasks.len() == per_page;
                    // ページ指定を更新する
                    page += 1;
                    // 現在のページのタスクをDTO配列に加える
                    clickup_tasks.append(current_tasks.tasks.as_mut());
                }
                Err(e) => bail!("{:?}", e),
            }
        }

        // レスポンス構造体(DTO)をEntityに変換する
        let tasks: Vec<ClickupTask> = clickup_tasks.into_iter().map(|t| t.into()).collect();
        tracing::trace!("Tasks Hit: {}", tasks.len());
        Ok(ClickupTasks::new(tasks))
    }
}

#[async_trait]
impl ClickupTimeEntryRepository for ClickupRepositoryImpl<ClickupTimeEntry> {
    #[tracing::instrument(
        name="find_time_entries",
        level="debug",
        skip_all,
        fields(
            start=%date_range.start_datetime_str(),
            end=%date_range.end_datetime_str()
        )
    )]
    async fn find_time_entries_by_date_range(
        &self,
        date_range: &DateRange,
    ) -> Result<ClickupTimeEntries> {
        let params = self.client.params().time_entries_within_a_date_range(
            true,
            clickup_date_start_milles(date_range),
            date_range.end().timestamp_millis(),
        );
        let resp = self
            .client
            .api()
            .time_entries_within_a_date_range(Some(params))
            .await;

        tracing::trace!(
            "\nAPI limit Remaining: {}.\nAPI limit will reset, after {} seconds.",
            resp.0.x_ratelimit_remaining(),
            resp.0.x_ratelimit_reset_from_now(),
        );

        match resp.try_to_clickup_time_entries() {
            Ok(time_entries) => {
                tracing::trace!("TimeEntries Hit: {}", time_entries.data.len());
                Ok(time_entries.into())
            }
            Err(e) => bail!("{:?}", e),
        }
    }
}
