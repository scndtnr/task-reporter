use crate::env::get_env_var;
use crate::infra::web::clickup::{ClickupApi, ClickupParamsBuilder};
use crate::infra::web::params::{AuthType, ContentType};
use crate::infra::web::BasicClient;

#[derive(Debug, Clone)]
pub(crate) struct ClickupClient {
    api: ClickupApi,
    params: ClickupParamsBuilder,
}

impl ClickupClient {
    pub(crate) fn new(client: BasicClient) -> Self {
        let api = ClickupApi::new(
            client,
            AuthType::General,
            ContentType::Json,
            get_env_var("APP_CLICKUP_API_ENDPOINT").unwrap(),
            get_env_var("APP_CLICKUP_ACCESS_TOKEN").unwrap(),
            get_env_var("APP_CLICKUP_TEAM_IDENT").unwrap(),
            true,
        );
        let params = ClickupParamsBuilder;
        Self { api, params }
    }

    pub(crate) fn api(&self) -> &ClickupApi {
        &self.api
    }

    pub(crate) fn params(&self) -> &ClickupParamsBuilder {
        &self.params
    }
}
