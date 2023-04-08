use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ApiParams {
    QueryString(Vec<(String, Option<String>)>),
    RequestBody(String),
}

pub(crate) trait CanConvertToQueryString {}
pub(crate) trait CanConvertToRequestBody {}

pub(crate) trait ToQueryString {
    fn to_params_of_query_string(&self) -> ApiParams
    where
        Self: serde::Serialize,
    {
        let s = serde_json::to_string(self).unwrap();
        let mut m: HashMap<String, Option<String>> = serde_json::from_str(&s).unwrap();
        // query string用パラメータなので、階層構造は存在しないものとする
        let v: Vec<(String, Option<String>)> = m.drain().map(|(k, v)| (k, v)).collect();
        ApiParams::QueryString(v)
    }
}

pub(crate) trait ToRequestBody {
    fn to_params_of_request_body(&self) -> ApiParams
    where
        Self: serde::Serialize,
    {
        let post = PostEnvelope { post: self };
        let s = serde_json::to_string(&post).unwrap();
        ApiParams::RequestBody(s)
    }
}

impl<T: CanConvertToQueryString> ToQueryString for T {}
impl<T: CanConvertToRequestBody> ToRequestBody for T {}

#[derive(Debug, serde_derive::Serialize)]
pub(crate) struct PostEnvelope<T: serde::Serialize> {
    post: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    mod unit {
        use super::*;
        use serde_derive::Serialize;

        #[derive(Debug, Serialize)]
        struct FakeStruct {
            q: String,
            page: String,
            per_page: String,
            option: Option<String>,
        }

        impl CanConvertToQueryString for FakeStruct {}
        impl CanConvertToRequestBody for FakeStruct {}

        impl FakeStruct {
            fn new(none: bool) -> FakeStruct {
                FakeStruct {
                    q: String::from("It's query"),
                    page: String::from("1"),
                    per_page: String::from("10"),
                    option: if none {
                        None
                    } else {
                        Some(String::from("It's Option"))
                    },
                }
            }
        }

        /// QueryString変換：Option::Some(s)の場合は、sが出力される。
        #[test]
        fn use_to_params_of_query_string() {
            let q = FakeStruct::new(false);
            let query = q.to_params_of_query_string();
            println!("{:?}", q);
            println!("{:?}", query);
            if let ApiParams::QueryString(items) = query {
                for item in items {
                    match item {
                        (k, v) if k == "q" => {
                            assert_eq!(v.unwrap().as_str(), "It's query")
                        }
                        (k, v) if k == "page" => assert_eq!(v.unwrap().as_str(), "1"),
                        (k, v) if k == "per_page" => assert_eq!(v.unwrap().as_str(), "10"),
                        (k, v) if k == "option" => {
                            assert_eq!(v.unwrap().as_str(), "It's Option")
                        }
                        _ => unreachable!(),
                    }
                }
            } else {
                unreachable!("The 'query' must be QueryString.");
            }
        }
        /// QueryString変換：Option::Noneの場合はNoneのまま。
        /// WebApiでの扱いは分からないが、nullとして扱われそう。
        #[test]
        fn use_to_params_with_none_of_query_string() {
            let q = FakeStruct::new(true);
            let query = q.to_params_of_query_string();
            println!("{:?}", q);
            println!("{:?}", query);
            if let ApiParams::QueryString(items) = query {
                for item in items {
                    match item {
                        (k, v) if k == "q" => {
                            assert_eq!(v.unwrap().as_str(), "It's query")
                        }
                        (k, v) if k == "page" => assert_eq!(v.unwrap().as_str(), "1"),
                        (k, v) if k == "per_page" => assert_eq!(v.unwrap().as_str(), "10"),
                        (k, v) if k == "option" => assert_eq!(v, None),

                        _ => unreachable!(),
                    }
                }
            } else {
                unreachable!("The 'query' must be QueryString.");
            }
        }

        /// HttpBody変換：Option::Some(s)の場合は、sが出力される。
        #[test]
        fn use_to_params_of_request_body() {
            let q = FakeStruct::new(false);
            let body = q.to_params_of_request_body();
            println!("{:?}", q);
            println!("{:?}", body);
            if let ApiParams::RequestBody(s) = body {
                assert_eq!(
                s,
                "{\"post\":{\"q\":\"It's query\",\"page\":\"1\",\"per_page\":\"10\",\"option\":\"It's Option\"}}"
            );
            } else {
                unreachable!("The 'query' must be HttpBody.");
            }
        }
        /// HttpBody変換：Option::Noneの場合は、nullに変換されて出力される。
        #[test]
        fn use_to_params_with_none_of_request_body() {
            let q = FakeStruct::new(true);
            let body = q.to_params_of_request_body();
            println!("{:?}", q);
            println!("{:?}", body);
            if let ApiParams::RequestBody(s) = body {
                assert_eq!(
                    s,
                    "{\"post\":{\"q\":\"It's query\",\"page\":\"1\",\"per_page\":\"10\",\"option\":null}}"
                );
            } else {
                unreachable!("The 'query' must be HttpBody.");
            }
        }
    }
}
