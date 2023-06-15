use axum::{headers::Authorization, TypedHeader};
use http::HeaderValue;

pub struct ApiKey(String);

impl axum::headers::authorization::Credentials for ApiKey {
    const SCHEME: &'static str = "ApiKey";

    fn decode(value: &HeaderValue) -> Option<Self> {
        let offset = Self::SCHEME.len() + 1;

        value.to_str().ok().and_then(|v| v.get(offset..)).map(|s| ApiKey(s.to_string()))
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
