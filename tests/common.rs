use mockito::mock;
use mockito::Mock;

pub fn make_mock(loc: &str, json: &str) -> Mock {
    mock("GET", loc)
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(json)
        .create()
}
