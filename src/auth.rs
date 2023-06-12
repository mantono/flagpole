use axum::{headers::Authorization, TypedHeader};
use http::HeaderValue;

pub struct ApiKey(String);

impl axum::headers::authorization::Credentials for ApiKey {
    const SCHEME: &'static str = "ApiKey";

    fn decode(value: &HeaderValue) -> Option<Self> {
        let prefix_len: usize = Self::SCHEME.len() + 1;
        if value.len() > prefix_len {
            let value: String =
                value.to_str().unwrap_or_default().get(prefix_len..).unwrap().to_string();
            Some(ApiKey(value))
        } else {
            None
        }
    }

    fn encode(&self) -> HeaderValue {
        HeaderValue::from_str(self.0.as_str()).unwrap()
    }
}

pub fn accept_auth(
    expected: &Option<String>,
    header: Option<TypedHeader<Authorization<ApiKey>>>,
) -> bool {
    let expected: &String = match expected {
        Some(expected) => expected,
        None => return true,
    };
    match header {
        Some(TypedHeader(Authorization(ApiKey(presented)))) => expected == &presented,
        None => false,
    }
}
