use serde::Deserialize;

use super::*;

#[test]
fn check_matcher() {
    let result = matcher("http://github.com/:owner", "http://github.com/mrtnvgr").unwrap();

    assert_eq!(result.get("owner"), Some("mrtnvgr").as_ref());
}

#[test]
fn check_trait() {
    #[derive(Deserialize)]
    struct Search {
        domain: String,
        query: String,
    }

    impl FromPattern<Self> for Search {}

    let result = Search::from_pattern(":domain?q=:query", "google.com?q=cat+pictures").unwrap();

    assert_eq!(result.domain, "google.com");
    assert_eq!(result.query, "cat+pictures");
}
