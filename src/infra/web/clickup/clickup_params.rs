use crate::infra::web::params::{ApiParams, CanConvertToQueryString, ToQueryString};
use derive_new::new;
use serde_derive::Serialize;

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ClickupParamsBuilder;

impl ClickupParamsBuilder {
    fn convert_bool_to_lower_string(&self, bool: bool) -> String {
        if bool {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
    /// 引数のbool値では、全部小文字の文字列を求めらている。
    pub(crate) fn filterd_team_tasks(
        &self,
        page: u32,
        subtasks: bool,
        include_closed: bool,
        date_updated_gt: i64,
        date_updated_lt: i64,
    ) -> ApiParams {
        ClickupGetParamsOfFilteredTeamTasks::new(
            page.to_string(),
            self.convert_bool_to_lower_string(subtasks),
            self.convert_bool_to_lower_string(include_closed),
            date_updated_gt.to_string(),
            date_updated_lt.to_string(),
        )
        .to_params_of_query_string()
    }

    /// 引数のbool値では、全部小文字の文字列を求めらている。
    pub(crate) fn time_entries_within_a_date_range(
        &self,
        include_location_names: bool,
        start_date: i64,
        end_date: i64,
    ) -> ApiParams {
        ClickupGetParamsOfTimeEntriesWithinADateRange::new(
            self.convert_bool_to_lower_string(include_location_names),
            start_date.to_string(),
            end_date.to_string(),
        )
        .to_params_of_query_string()
    }
}

impl CanConvertToQueryString for ClickupGetParamsOfFilteredTeamTasks {}
impl CanConvertToQueryString for ClickupGetParamsOfTimeEntriesWithinADateRange {}

#[derive(new, Debug, Serialize)]
pub(crate) struct ClickupGetParamsOfFilteredTeamTasks {
    page: String,
    // bool値だが、全部小文字の文字列を求められている "true" or "talse"
    subtasks: String,
    include_closed: String,
    // 下記2つは数値だが、文字列を求められている。
    date_updated_gt: String,
    date_updated_lt: String,
}

#[derive(new, Debug, Serialize)]
pub(crate) struct ClickupGetParamsOfTimeEntriesWithinADateRange {
    // bool値だが、全部小文字の文字列を求められている "true" or "talse"
    include_location_names: String,
    // 下記2つは数値だが、文字列を求められている。
    start_date: String,
    end_date: String,
}
