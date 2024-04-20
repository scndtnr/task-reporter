use crate::infra::web::basic_client::{BasicClientApi, BasicProperty};
use crate::infra::web::clickup::ClickupResponse;
use crate::infra::web::params::{ApiParams, AuthType, ContentType};
use crate::infra::web::BasicClient;
use derive_new::new;
use reqwest::Client;

#[derive(new, Debug, Clone)]
pub(crate) struct ClickupApi {
    pub(crate) client: BasicClient,
    pub(crate) auth_type: AuthType,
    pub(crate) content_type: ContentType,
    pub(crate) api_endpoint: String,
    pub(crate) access_token: String,
    pub(crate) team_ident: String,
    pub(crate) retry_on_rate_limit_exceeded: bool,
}

impl BasicProperty for ClickupApi {
    fn client(&self) -> &Client {
        &self.client.0
    }
    fn auth_type(&self) -> &AuthType {
        &self.auth_type
    }
    fn content_type(&self) -> &ContentType {
        &self.content_type
    }
    fn api_endpoint(&self) -> &String {
        &self.api_endpoint
    }
    fn access_token(&self) -> &String {
        &self.access_token
    }
    fn retry_on_rate_limit_exceeded(&self) -> &bool {
        &self.retry_on_rate_limit_exceeded
    }
}

impl BasicClientApi for ClickupApi {}

impl ClickupApi {
    pub(crate) fn team_id(&self) -> &str {
        &self.team_ident
    }

    #[allow(unused)]
    pub(crate) async fn teams(&self, params: Option<ApiParams>) -> ClickupResponse {
        self.send_get("/api/v2/team", params).await.into()
    }

    #[allow(unused)]
    pub(crate) async fn user(
        &self,
        user_id: impl Into<String>,
        params: Option<ApiParams>,
    ) -> ClickupResponse {
        self.send_get(
            &format!("/api/v2/team/{}/user/{}", self.team_id(), user_id.into()),
            params,
        )
        .await
        .into()
    }

    #[allow(unused)]
    pub(crate) async fn spaces(&self, params: Option<ApiParams>) -> ClickupResponse {
        self.send_get(&format!("/api/v2/team/{}/space?", self.team_id()), params)
            .await
            .into()
    }

    #[allow(unused)]
    pub(crate) async fn folders(
        &self,
        space_id: impl Into<String>,
        params: Option<ApiParams>,
    ) -> ClickupResponse {
        self.send_get(
            &format!("/api/v2/space/{}/folder?", space_id.into()),
            params,
        )
        .await
        .into()
    }

    #[allow(unused)]
    pub(crate) async fn folderless_lists(
        &self,
        space_id: impl Into<String>,
        params: Option<ApiParams>,
    ) -> ClickupResponse {
        self.send_get(&format!("/api/v2/space/{}/list?", space_id.into()), params)
            .await
            .into()
    }

    #[allow(unused)]
    pub(crate) async fn lists(
        &self,
        folder_id: impl Into<String>,
        params: Option<ApiParams>,
    ) -> ClickupResponse {
        self.send_get(
            &format!("/api/v2/folder/{}/list?", folder_id.into()),
            params,
        )
        .await
        .into()
    }

    #[allow(unused)]
    pub(crate) async fn list(
        &self,
        list_id: impl Into<String>,
        params: Option<ApiParams>,
    ) -> ClickupResponse {
        self.send_get(&format!("/api/v2/list/{}", list_id.into()), params)
            .await
            .into()
    }

    #[allow(unused)]
    pub(crate) async fn tasks(
        &self,
        list_id: impl Into<String>,
        params: Option<ApiParams>,
    ) -> ClickupResponse {
        self.send_get(&format!("/api/v2/list/{}/task?", list_id.into()), params)
            .await
            .into()
    }

    #[allow(unused)]
    pub(crate) async fn task(
        &self,
        task_id: impl Into<String>,
        params: Option<ApiParams>,
    ) -> ClickupResponse {
        self.send_get(&format!("/api/v2/task/{}/", task_id.into()), params)
            .await
            .into()
    }

    pub(crate) async fn filtered_team_tasks(&self, params: Option<ApiParams>) -> ClickupResponse {
        self.send_get(&format!("/api/v2/team/{}/task?", self.team_id()), params)
            .await
            .into()
    }

    pub(crate) async fn time_entries_within_a_date_range(
        &self,
        params: Option<ApiParams>,
    ) -> ClickupResponse {
        self.send_get(
            &format!("/api/v2/team/{}/time_entries?", self.team_id()),
            params,
        )
        .await
        .into()
    }

    #[allow(unused)]
    pub(crate) async fn stop_a_time_entry(&self, params: Option<ApiParams>) -> ClickupResponse {
        self.send_post(
            &format!("/api/v2/team/{}/time_entries/stop", self.team_id()),
            params,
        )
        .await
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::DateRange;
    use crate::infra::web::clickup::clickup_client::ClickupClient;
    use crate::infra::web::clickup::ParseClickupResponse;
    use crate::infra::web::BasicResponse;

    #[tokio::test]
    async fn check_response_header() {
        // teams()のpathを確認する
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().teams(None).await;
        assert_eq!(resp.0.path(), "/api/v2/team");
    }

    #[tokio::test]
    async fn check_ratelimit_remaining() {
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().teams(None).await;
        let remaining = resp.0.x_ratelimit_remaining().parse().unwrap();
        dbg!(remaining);
        assert!(
            20 < remaining,
            "API使用制限が残り20回を切りました。15分程度お待ちください。"
        );
    }

    #[tokio::test]
    async fn check_ratelimit_reset() {
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().teams(None).await;
        let reset_secs = resp.0.x_ratelimit_reset_from_now();
        println!("API limitrate will reset, after {} seconds.", reset_secs);
        assert!(reset_secs < (60 * 15));
    }

    #[tokio::test]
    async fn check_teams_0th_id_by_serde_value() {
        // teamsの内、0番目のチームのIDを確認する
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().teams(None).await;
        let teams = resp.try_to_serde_json_value().unwrap();
        let team_id = teams.get("teams").unwrap()[0]
            .get("id")
            .unwrap()
            .to_string();
        assert_eq!(
            team_id
                .strip_prefix('"')
                .unwrap()
                .strip_suffix('"')
                .unwrap(),
            client.api().team_id()
        );
    }

    #[tokio::test]
    async fn check_teams_0th_id() {
        // teamsの内、0番目のチームのIDを確認する
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().teams(None).await;
        let teams = resp.try_to_clickup_teams().unwrap();
        println!("{:?}", teams);
        assert_eq!(teams.teams[0].id, client.api().team_id());
    }

    #[tokio::test]
    #[should_panic = "ビジネスプランが必要です。: ClickupError { ECODE: \"TEAM_110\", err: \"Team must be on enterprise plan\" }"]
    async fn check_user() {
        // ユーザIDを取得する
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().teams(None).await;
        let user_id = resp.try_to_clickup_teams().unwrap().teams[0].members[0]
            .user
            .id;
        dbg!(&user_id);

        // ユーザ情報を取得する
        let resp = client.api().user(user_id.to_string(), None).await;
        let user = resp
            .try_to_serde_json_value()
            .expect("ビジネスプランが必要です。");
        println!("{:?}", user);
    }

    #[tokio::test]
    async fn check_spaces() {
        // 複数のスペースを取得する
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().spaces(None).await;
        let spaces = resp.try_to_clickup_spaces().unwrap();
        dbg!(&spaces);
        assert_eq!(spaces.spaces[2].name, "Development")
    }

    #[tokio::test]
    async fn check_folders() {
        // 複数のスペースIDを取得する
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().spaces(None).await;
        let spaces = resp.try_to_clickup_spaces().unwrap();
        let space_ids = spaces
            .spaces
            .iter()
            .map(|s| s.id.to_owned())
            .collect::<Vec<String>>();

        // 複数のフォルダを取得する
        let resp = client.api().folders(&space_ids[2], None).await;
        // let folders = resp.try_to_serde_json_value().unwrap();
        let folders = resp.try_to_clickup_folders().unwrap();
        dbg!(&folders);
        assert_eq!(folders.folders[0].name, "基礎学習")
    }

    #[tokio::test]
    async fn check_folderless_lists() {
        // 複数のスペースIDを取得する
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().spaces(None).await;
        let spaces = resp.try_to_clickup_spaces().unwrap();
        let space_ids = spaces
            .spaces
            .iter()
            .map(|s| s.id.to_owned())
            .collect::<Vec<String>>();

        // 複数のフォルダに紐づかないリストIDを取得する
        let resp = client.api().folderless_lists(&space_ids[2], None).await;
        let folderless_lists = resp.try_to_clickup_lists().unwrap();
        dbg!(&folderless_lists);
        assert_eq!(folderless_lists.lists[0].name, "開発ネタ")
    }

    #[tokio::test]
    async fn check_lists() {
        // 複数のスペースIDを取得する
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().spaces(None).await;
        let spaces = resp.try_to_clickup_spaces().unwrap();
        let space_ids = spaces
            .spaces
            .iter()
            .map(|s| s.id.to_owned())
            .collect::<Vec<String>>();

        // 複数のフォルダを取得する
        let resp = client.api().folders(&space_ids[2], None).await;
        // let folders = resp.try_to_serde_json_value().unwrap();
        let folder_ids = resp
            .try_to_clickup_folders()
            .unwrap()
            .folders
            .iter()
            .map(|f| f.id.to_owned())
            .collect::<Vec<String>>();

        // フォルダーに紐づくリストを取得する
        let resp = client.api().lists(&folder_ids[0], None).await;
        let lists = resp.try_to_clickup_lists().unwrap();
        dbg!(&lists);
        assert_eq!(lists.lists[2].name, "Rust学習");
    }

    #[tokio::test]
    async fn check_list() {
        // 複数のスペースIDを取得する
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().spaces(None).await;
        let spaces = resp.try_to_clickup_spaces().unwrap();
        let space_ids = spaces
            .spaces
            .iter()
            .map(|s| s.id.to_owned())
            .collect::<Vec<String>>();

        // 複数のフォルダに紐づかないリストIDを取得する
        let resp = client.api().folderless_lists(&space_ids[2], None).await;
        let folderless_list_ids = resp
            .try_to_clickup_lists()
            .unwrap()
            .lists
            .iter()
            .map(|f| f.id.to_owned())
            .collect::<Vec<String>>();

        // 指定したリストの情報を取得する
        let resp = client.api().list(&folderless_list_ids[0], None).await;
        let list = resp.try_to_clickup_list().unwrap();
        dbg!(&list);
        assert_eq!(list.name, "開発ネタ")
    }
    #[tokio::test]
    async fn check_tasks() {
        // 複数のスペースIDを取得する
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().spaces(None).await;
        let spaces = resp.try_to_clickup_spaces().unwrap();
        let space_ids = spaces
            .spaces
            .iter()
            .map(|s| s.id.to_owned())
            .collect::<Vec<String>>();

        // 複数のフォルダに紐づかないリストIDを取得する
        let resp = client.api().folderless_lists(&space_ids[2], None).await;
        let folderless_list_ids = resp
            .try_to_clickup_lists()
            .unwrap()
            .lists
            .iter()
            .map(|f| f.id.to_owned())
            .collect::<Vec<String>>();

        // 指定したリストに含まれるタスクの情報を取得する
        let resp = client.api().tasks(&folderless_list_ids[0], None).await;
        let mut tasks = resp.try_to_clickup_tasks().unwrap();
        tasks.tasks.reverse();
        dbg!(&tasks);
        assert_eq!(
            tasks.tasks[0].name.as_str(),
            "アパート探し（playwright,streamlit,db）"
        );
    }
    #[tokio::test]
    async fn check_task() {
        // 複数のスペースIDを取得する
        let client = ClickupClient::new(BasicClient::new());
        let resp = client.api().spaces(None).await;
        let spaces = resp.try_to_clickup_spaces().unwrap();
        let space_ids = spaces
            .spaces
            .iter()
            .map(|s| s.id.to_owned())
            .collect::<Vec<String>>();

        // 複数のフォルダに紐づかないリストIDを取得する
        let resp = client.api().folderless_lists(&space_ids[2], None).await;
        let folderless_list_ids = resp
            .try_to_clickup_lists()
            .unwrap()
            .lists
            .iter()
            .map(|f| f.id.to_owned())
            .collect::<Vec<String>>();

        // 指定したリストに含まれるタスクのIDを取得する
        let resp = client.api().tasks(&folderless_list_ids[0], None).await;
        let mut task_ids = resp
            .try_to_clickup_tasks()
            .unwrap()
            .tasks
            .iter()
            .map(|t| t.id.to_owned())
            .collect::<Vec<String>>();
        task_ids.reverse();

        // 指定したタスクの情報を取得する
        let resp = client.api().task(&task_ids[0], None).await;
        let task = resp.try_to_clickup_task().unwrap();
        dbg!(&task);
        assert_eq!(
            task.name.as_str(),
            "アパート探し（playwright,streamlit,db）"
        )
    }

    #[tokio::test]
    async fn check_filtered_team_tasks() {
        // 事前準備
        let dr = DateRange::new(Some("2023/04/03"), None);
        let client = ClickupClient::new(BasicClient::new());
        let params = client.params().filterd_team_tasks(
            0,
            true,
            true,
            dr.start_unixtime_millis(),
            dr.end_unixtime_millis(),
        );

        // リクエスト
        let resp = client.api().filtered_team_tasks(Some(params)).await;
        let tasks = resp.try_to_clickup_tasks().unwrap();
        dbg!(&tasks);
        assert_eq!(tasks.tasks[0].name.as_str(), "請求書をメール送付する")
    }
    #[tokio::test]
    async fn check_time_entries_within_a_date_range() {
        // 事前準備
        let dr = DateRange::new(Some("2022/01/05"), None);
        let client = ClickupClient::new(BasicClient::new());
        let params = client.params().time_entries_within_a_date_range(
            true,
            dr.start_unixtime_millis(),
            dr.end_unixtime_millis(),
        );

        // リクエスト
        let resp = client
            .api()
            .time_entries_within_a_date_range(Some(params))
            .await;
        let time_entries = resp.try_to_clickup_time_entries().unwrap();
        dbg!(&time_entries);
        assert_eq!(
            time_entries.data[0].task.name.as_str(),
            "Slackアプリ上でAggregator処理を実行した後にユーザへ返すメッセージを作る"
        )
    }
    #[tokio::test]
    #[ignore = "This api is post method."]
    async fn check_stop_a_time_entry() {
        todo!();
    }
}
