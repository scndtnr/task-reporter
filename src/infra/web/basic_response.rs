use chrono::{DateTime, Utc};
use reqwest::{header::HeaderMap, Response};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BasicResponseImpl {
    text: String,
    status: u16,
    is_success: bool,
    headers: HeaderMap,
    cannot_be_a_base: bool,
    fragment: Option<String>,
    host: Option<String>,
    password: Option<String>,
    path: String,
    port: Option<u16>,
    query: Option<String>,
    scheme: String,
    username: String,
}

impl BasicResponseImpl {
    pub(crate) async fn new(response: Response) -> Self {
        let status = response.status().as_u16();
        let is_success = response.status().is_success();
        let headers = response.headers().clone();
        let cannot_be_a_base = response.url().cannot_be_a_base();
        let fragment = response.url().fragment().map(String::from);
        let host = response.url().host_str().map(String::from);
        let password = response.url().password().map(String::from);
        let path = response.url().path().to_string();
        let port = response.url().port();
        let query = response.url().query().map(String::from);
        let scheme = response.url().scheme().to_string();
        let username = response.url().username().to_string();
        let text = response.text().await.unwrap();
        Self {
            text,
            status,
            is_success,
            headers,
            cannot_be_a_base,
            fragment,
            host,
            password,
            path,
            port,
            query,
            scheme,
            username,
        }
    }
}

pub(crate) trait BasicResponse {
    fn text(&self) -> &str;
    fn status(&self) -> u16;
    fn is_success(&self) -> bool;
    fn content_type(&self) -> &str;
    fn date(&self) -> &str;
    fn x_ratelimit_limit(&self) -> &str;
    fn x_ratelimit_remaining(&self) -> &str;
    fn x_ratelimit_reset(&self) -> &str;
    fn x_ratelimit_reset_from_now(&self) -> u64;
    fn cannot_be_a_base(&self) -> bool;
    fn fragment(&self) -> Option<&str>;
    fn host(&self) -> Option<&str>;
    fn password(&self) -> Option<&str>;
    fn path(&self) -> &str;
    fn port(&self) -> Option<u16>;
    fn query(&self) -> Option<&str>;
    fn scheme(&self) -> &str;
    fn username(&self) -> &str;
}

impl BasicResponse for BasicResponseImpl {
    fn text(&self) -> &str {
        self.text.as_str()
    }
    fn status(&self) -> u16 {
        self.status
    }
    fn is_success(&self) -> bool {
        self.is_success
    }
    fn content_type(&self) -> &str {
        self.headers.get("content-type").unwrap().to_str().unwrap()
    }
    fn date(&self) -> &str {
        self.headers.get("date").unwrap().to_str().unwrap()
    }
    fn x_ratelimit_limit(&self) -> &str {
        self.headers
            .get("x-ratelimit-limit")
            .unwrap()
            .to_str()
            .unwrap()
    }
    fn x_ratelimit_remaining(&self) -> &str {
        self.headers
            .get("x-ratelimit-remaining")
            .unwrap()
            .to_str()
            .unwrap()
    }
    fn x_ratelimit_reset(&self) -> &str {
        self.headers
            .get("x-ratelimit-reset")
            .unwrap()
            .to_str()
            .unwrap()
    }
    fn x_ratelimit_reset_from_now(&self) -> u64 {
        match self.x_ratelimit_reset().parse::<i64>() {
            Ok(unixtime_secs) => {
                let now: DateTime<Utc> = Utc::now();
                (now.timestamp() - unixtime_secs).unsigned_abs()
            }
            Err(_) => 60,
        }
    }

    fn cannot_be_a_base(&self) -> bool {
        self.cannot_be_a_base
    }
    fn fragment(&self) -> Option<&str> {
        self.fragment.as_deref()
    }
    fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }
    fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }
    fn path(&self) -> &str {
        self.path.as_str()
    }
    fn port(&self) -> Option<u16> {
        self.port
    }
    fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }
    fn scheme(&self) -> &str {
        self.scheme.as_str()
    }
    fn username(&self) -> &str {
        self.username.as_str()
    }
}
