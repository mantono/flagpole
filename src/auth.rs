use std::{hint::black_box, io::Read};

use axum::{headers::Authorization, TypedHeader};
use http::HeaderValue;
use sha2::Digest;

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
        Some(TypedHeader(Authorization(ApiKey(presented)))) => const_comp(expected, &presented),
        None => false,
    }
}

/// Function for comparing two strings in equal time. I.e. the similarity of the strings should
/// have no bearing on the time it takes to compare them.
///
/// A naive solution to compare two strings for equality would most likely return at the first byte
/// that differs, which means that the more similar two strings are, the longer the execution time
/// for comparing the strings. This would make such naive implementation susceptible to a
/// [timing attack](https://en.wikipedia.org/wiki/Timing_attack), which should ideally be avoided.
fn const_comp(i0: impl AsRef<[u8]>, i1: impl AsRef<[u8]>) -> bool {
    // Hash inputs so we always get equal length when comparing, so we do not risk leaking the
    // length of the expected API key via a timing attack.
    let h0 = sha2::Sha384::digest(i0);
    let h1 = sha2::Sha384::digest(i1);
    // The documentation for black box explicitly states that  _"this function does not offer any
    // guarantees for cryptographic or security purposes"_. But the other two options are to
    // either
    // 1. Take no measures at all to prevent unwanted optimizations
    // 2. Try to hand roll a constant time comparison algorithm implemented in assembler
    //
    // The first option seems worse than taking no action at all, and the later is not really
    // feasible.
    black_box(
        h0.bytes()
            .zip(h1.bytes())
            .fold(0, |acc, (x, y)| acc | (x.unwrap() ^ y.unwrap()))
            == 0,
    )
}

#[cfg(test)]
mod tests {
    use crate::auth::const_comp;

    #[test]
    fn compare_on_equal() {
        assert!(const_comp("foo", "foo"));
    }

    #[test]
    fn compare_on_not_equal() {
        assert!(!const_comp("foo", "bar"));
    }
}
