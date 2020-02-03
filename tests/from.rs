#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use mockito::mock;
use pokerust::Berry;

static CHESTO_JSON: &str = r#"{"id":2,"name":"chesto","growth_time":3,"max_harvest":5,"natural_gift_power":60,"size":80,"smoothness":25,"soil_dryness":15,"firmness":{"name":"super-hard","url":"http:\/\/localhost:8000\/api\/v2\/berry-firmness\/5\/"},"flavors":[{"potency":0,"flavor":{"name":"spicy","url":"http:\/\/localhost:8000\/api\/v2\/berry-flavor\/1\/"}},{"potency":10,"flavor":{"name":"dry","url":"http:\/\/localhost:8000\/api\/v2\/berry-flavor\/2\/"}},{"potency":0,"flavor":{"name":"sweet","url":"http:\/\/localhost:8000\/api\/v2\/berry-flavor\/3\/"}},{"potency":0,"flavor":{"name":"bitter","url":"http:\/\/localhost:8000\/api\/v2\/berry-flavor\/4\/"}},{"potency":0,"flavor":{"name":"sour","url":"http:\/\/localhost:8000\/api\/v2\/berry-flavor\/5\/"}}],"item":{"name":"chesto-berry","url":"http:\/\/localhost:8000\/api\/v2\/item\/127\/"},"natural_gift_type":{"name":"water","url":"http:\/\/localhost:8000\/api\/v2\/type\/11\/"}}"#;

#[test]
fn berry_from_id() {
    let _m = mock("GET", "/api/v2/berry/2/")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(CHESTO_JSON)
        .create();

    let _berry = Berry::from_id(2).unwrap();
}

#[test]
fn berry_from_name() {
    let _m = mock("GET", "/api/v2/berry/chesto/")
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(CHESTO_JSON)
        .create();

    let _berry = Berry::from_name("chesto").unwrap();
}
