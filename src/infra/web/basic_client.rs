use crate::env::set_dotenv;
use crate::infra::web::params::{ApiParams, AuthType, ContentType, HttpMethods};
use crate::infra::web::{BasicResponse, BasicResponseImpl};
use async_trait::async_trait;
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, RequestBuilder, Response};
use reqwest::{StatusCode, Url};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub(crate) struct BasicClient(pub Client);

impl BasicClient {
    pub fn new() -> Self {
        // dotenvを読み込む
        set_dotenv("task-reporter");
        Self(Client::new())
    }
}

impl Default for BasicClient {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) trait BasicProperty {
    fn client(&self) -> &Client;
    fn auth_type(&self) -> &AuthType;
    fn content_type(&self) -> &ContentType;
    fn api_endpoint(&self) -> &String;
    fn access_token(&self) -> &String;
    fn retry_on_rate_limit_exceeded(&self) -> &bool;
}

#[async_trait]
pub(crate) trait BasicClientApi: BasicProperty {
    async fn send_get(&self, path: &str, params: Option<ApiParams>) -> BasicResponseImpl {
        self.send_request(HttpMethods::Get, path, params).await
    }
    async fn send_post(&self, path: &str, params: Option<ApiParams>) -> BasicResponseImpl {
        self.send_request(HttpMethods::Post, path, params).await
    }
    async fn send_put(&self, path: &str, params: Option<ApiParams>) -> BasicResponseImpl {
        self.send_request(HttpMethods::Put, path, params).await
    }
    async fn send_patch(&self, path: &str, params: Option<ApiParams>) -> BasicResponseImpl {
        self.send_request(HttpMethods::Patch, path, params).await
    }
    async fn send_delete(&self, path: &str, params: Option<ApiParams>) -> BasicResponseImpl {
        self.send_request(HttpMethods::Delete, path, params).await
    }
    async fn send_request(
        &self,
        method: HttpMethods,
        path: &str,
        params: Option<ApiParams>,
    ) -> BasicResponseImpl {
        let url = self.build_request_url(path);
        let builder = self.build_request_method(method, url);
        let builder = self.build_request_auth_type(builder);
        let builder = self.build_request_content_type(builder);
        let builder = self.build_request_api_params(builder, params);

        // Too Many Requests(429)発生時、指定秒数待機した後に再試行するか否か
        let resp = if !self.retry_on_rate_limit_exceeded() {
            builder.send().await.unwrap()
        } else {
            send_request_retry_on_too_many_requests(builder).await
        };
        BasicResponseImpl::new(resp).await
    }

    // 以下はリクエスト処理を分割したもの

    /// リクエストするURLを組み立てる
    fn build_request_url(&self, path: &str) -> Url {
        Url::parse(self.api_endpoint()).unwrap().join(path).unwrap()
    }
    /// HTTPリクエストメソッド毎に処理する
    fn build_request_method(&self, method: HttpMethods, url: Url) -> RequestBuilder {
        match method {
            HttpMethods::Get => self.client().get(url),
            HttpMethods::Post => self.client().post(url),
            HttpMethods::Put => self.client().put(url),
            HttpMethods::Patch => self.client().patch(url),
            HttpMethods::Delete => self.client().delete(url),
        }
    }
    /// 認証方式（Bearer認証か、ただのAccessToken認証かで場合分け）
    fn build_request_auth_type(&self, builder: RequestBuilder) -> RequestBuilder {
        match &self.auth_type() {
            AuthType::Bearer => builder.bearer_auth(self.access_token()),
            AuthType::General => builder.header(AUTHORIZATION, self.access_token()),
        }
    }

    fn build_request_content_type(&self, builder: RequestBuilder) -> RequestBuilder {
        match &self.content_type() {
            ContentType::Json => {
                builder.header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            }
            ContentType::XWwwFormUrlencoded => builder.header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            ),
        }
    }

    /// Httpリクエストのパラメータを受け取り、  
    /// リクエストビルダにクエリストリングやボディとして設定する
    fn build_request_api_params(
        &self,
        builder: RequestBuilder,
        params: Option<ApiParams>,
    ) -> RequestBuilder {
        match params {
            Some(params) => match params {
                ApiParams::QueryString(query) => builder.query(&query),
                ApiParams::RequestBody(body) => builder.body(body),
            },
            None => builder,
        }
    }
}

//// 各サービス毎に更新までの残り秒数を取得し、その秒数＋5秒待機する。
/// 残り秒数を取得できない場合は60秒+5秒待機する。  
async fn send_request_retry_on_too_many_requests(builder: RequestBuilder) -> Response {
    loop {
        match builder.try_clone().unwrap().send().await {
            Ok(resp) if resp.status() == StatusCode::TOO_MANY_REQUESTS => {
                let wait_secs = BasicResponseImpl::new(resp)
                    .await
                    .x_ratelimit_reset_from_now();
                // 5秒余裕を持って待機させる
                wait_for(wait_secs + 5);
            }
            Ok(resp) => break resp,
            Err(e) => panic!("{:#?}", e),
        }
    }
}
/// 指定秒数の間、メインスレッドをブロックする
/// 10秒毎に "waiting..."のメッセージを表示する
#[tracing::instrument()]
fn wait_for(wait_secs: u64) {
    assert!(wait_secs > 0);
    for _ in 0..(wait_secs / 10) {
        tracing::info!("waiting...");
        thread::sleep(Duration::from_secs(10));
    }
    thread::sleep(Duration::from_secs(wait_secs % 10));
}
