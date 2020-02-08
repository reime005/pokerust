use mockito::mock;
use mockito::Mock;

use pokerust::ENDPOINT;

// Return true if we are using the mockito endpoint, panic if we are using the public endpoint, and
// false otherwise
pub fn check_endpoint() -> bool {
    match &*ENDPOINT.as_str() {
        "https://pokeapi.co/api/v2/" => panic!(
            r#"Please do not run tests using the public endpoint. Set POKERUST_ENDPOINT to "http://127.0.0.1:1234/api/v2/" for HTTP mocking on non-ignored tests"#
        ),
        "http://127.0.0.1:1234/api/v2/" => return true,
        _ => return false,
    }
}

pub fn make_mock(loc: &str, json: &str) -> Option<Mock> {
    if check_endpoint() {
        return Some(
            mock("GET", loc)
                .with_status(200)
                .with_header("content-type", "application/json; charset=utf-8")
                .with_body(json)
                .create(),
        );
    }
    None
}
