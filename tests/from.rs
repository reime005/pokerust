#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use mockito::mock;
use mockito::Mock;
use pokerust::*;

static CHESTO_JSON: &str = r#"{"id":2,"name":"chesto","growth_time":3,"max_harvest":5,"natural_gift_power":60,"size":80,"smoothness":25,"soil_dryness":15,"firmness":{"name":"super-hard","url":"http:\/\/localhost:8000\/api\/v2\/berry-firmness\/5\/"},"flavors":[{"potency":0,"flavor":{"name":"spicy","url":"http:\/\/localhost:8000\/api\/v2\/berry-flavor\/1\/"}},{"potency":10,"flavor":{"name":"dry","url":"http:\/\/localhost:8000\/api\/v2\/berry-flavor\/2\/"}},{"potency":0,"flavor":{"name":"sweet","url":"http:\/\/localhost:8000\/api\/v2\/berry-flavor\/3\/"}},{"potency":0,"flavor":{"name":"bitter","url":"http:\/\/localhost:8000\/api\/v2\/berry-flavor\/4\/"}},{"potency":0,"flavor":{"name":"sour","url":"http:\/\/localhost:8000\/api\/v2\/berry-flavor\/5\/"}}],"item":{"name":"chesto-berry","url":"http:\/\/localhost:8000\/api\/v2\/item\/127\/"},"natural_gift_type":{"name":"water","url":"http:\/\/localhost:8000\/api\/v2\/type\/11\/"}}"#;

fn make_mock(loc: &str, json: &str) -> Mock {
    mock("GET", loc)
        .with_status(200)
        .with_header("content-type", "application/json; charset=utf-8")
        .with_body(json)
        .create()
}

#[test]
fn berry_from_id() {
    let _m = make_mock("/api/v2/berry/2/", CHESTO_JSON);
    let _ = Berry::from_id(2).unwrap();
}

#[test]
fn berry_from_name() {
    let _m = make_mock("/api/v2/berry/chesto/", CHESTO_JSON);
    let _ = Berry::from_name("chesto").unwrap();
}

static SUPER_HARD_JSON: &str = r#"{"berries":[{"name":"chesto","url":"https://pokeapi.co/api/v2/berry/2/"},{"name":"aspear","url":"https://pokeapi.co/api/v2/berry/5/"},{"name":"oran","url":"https://pokeapi.co/api/v2/berry/7/"},{"name":"lum","url":"https://pokeapi.co/api/v2/berry/9/"},{"name":"aguav","url":"https://pokeapi.co/api/v2/berry/14/"},{"name":"wepear","url":"https://pokeapi.co/api/v2/berry/19/"},{"name":"nomel","url":"https://pokeapi.co/api/v2/berry/30/"},{"name":"occa","url":"https://pokeapi.co/api/v2/berry/36/"},{"name":"colbur","url":"https://pokeapi.co/api/v2/berry/50/"},{"name":"babiri","url":"https://pokeapi.co/api/v2/berry/51/"},{"name":"starf","url":"https://pokeapi.co/api/v2/berry/59/"},{"name":"custap","url":"https://pokeapi.co/api/v2/berry/62/"}],"id":5,"name":"super-hard","names":[{"language":{"name":"fr","url":"https://pokeapi.co/api/v2/language/5/"},"name":"Super ferme"},{"language":{"name":"en","url":"https://pokeapi.co/api/v2/language/9/"},"name":"Super Hard"}]}"#;

#[test]
fn berry_firmness_from_id() {
    let _m = make_mock("/api/v2/berry-firmness/5/", SUPER_HARD_JSON);
    let _ = BerryFirmness::from_id(5).unwrap();
}

#[test]
fn berry_firmness_from_name() {
    let _m = make_mock("/api/v2/berry-firmness/super-hard/", SUPER_HARD_JSON);
    let _ = BerryFirmness::from_name("super-hard").unwrap();
}

static SPICY_JSON: &str = r#"{"berries":[{"berry":{"name":"rowap","url":"https://pokeapi.co/api/v2/berry/64/"},"potency":10},{"berry":{"name":"leppa","url":"https://pokeapi.co/api/v2/berry/6/"},"potency":10},{"berry":{"name":"oran","url":"https://pokeapi.co/api/v2/berry/7/"},"potency":10},{"berry":{"name":"persim","url":"https://pokeapi.co/api/v2/berry/8/"},"potency":10},{"berry":{"name":"lum","url":"https://pokeapi.co/api/v2/berry/9/"},"potency":10},{"berry":{"name":"razz","url":"https://pokeapi.co/api/v2/berry/16/"},"potency":10},{"berry":{"name":"pinap","url":"https://pokeapi.co/api/v2/berry/20/"},"potency":10},{"berry":{"name":"pomeg","url":"https://pokeapi.co/api/v2/berry/21/"},"potency":10},{"berry":{"name":"qualot","url":"https://pokeapi.co/api/v2/berry/23/"},"potency":10},{"berry":{"name":"hondew","url":"https://pokeapi.co/api/v2/berry/24/"},"potency":10},{"berry":{"name":"nomel","url":"https://pokeapi.co/api/v2/berry/30/"},"potency":10},{"berry":{"name":"belue","url":"https://pokeapi.co/api/v2/berry/35/"},"potency":10},{"berry":{"name":"rindo","url":"https://pokeapi.co/api/v2/berry/39/"},"potency":10},{"berry":{"name":"shuca","url":"https://pokeapi.co/api/v2/berry/43/"},"potency":10},{"berry":{"name":"charti","url":"https://pokeapi.co/api/v2/berry/47/"},"potency":10},{"berry":{"name":"apicot","url":"https://pokeapi.co/api/v2/berry/57/"},"potency":10},{"berry":{"name":"cheri","url":"https://pokeapi.co/api/v2/berry/1/"},"potency":10},{"berry":{"name":"chople","url":"https://pokeapi.co/api/v2/berry/41/"},"potency":15},{"berry":{"name":"figy","url":"https://pokeapi.co/api/v2/berry/11/"},"potency":15},{"berry":{"name":"occa","url":"https://pokeapi.co/api/v2/berry/36/"},"potency":15},{"berry":{"name":"tamato","url":"https://pokeapi.co/api/v2/berry/26/"},"potency":20},{"berry":{"name":"tanga","url":"https://pokeapi.co/api/v2/berry/46/"},"potency":20},{"berry":{"name":"babiri","url":"https://pokeapi.co/api/v2/berry/51/"},"potency":25},{"berry":{"name":"starf","url":"https://pokeapi.co/api/v2/berry/59/"},"potency":30},{"berry":{"name":"liechi","url":"https://pokeapi.co/api/v2/berry/53/"},"potency":30},{"berry":{"name":"spelon","url":"https://pokeapi.co/api/v2/berry/31/"},"potency":30},{"berry":{"name":"petaya","url":"https://pokeapi.co/api/v2/berry/56/"},"potency":30},{"berry":{"name":"lansat","url":"https://pokeapi.co/api/v2/berry/58/"},"potency":30},{"berry":{"name":"enigma","url":"https://pokeapi.co/api/v2/berry/60/"},"potency":40}],"contest_type":{"name":"cool","url":"https://pokeapi.co/api/v2/contest-type/1/"},"id":1,"name":"spicy","names":[{"language":{"name":"fr","url":"https://pokeapi.co/api/v2/language/5/"},"name":"Épicé"},{"language":{"name":"en","url":"https://pokeapi.co/api/v2/language/9/"},"name":"Spicy"}]}"#;

#[test]
fn berry_flavor_from_id() {
    let _m = make_mock("/api/v2/berry-flavor/1/", SPICY_JSON);
    let _ = BerryFlavor::from_id(1).unwrap();
}

#[test]
fn berry_flavor_from_name() {
    let _m = make_mock("/api/v2/berry-flavor/spicy/", SPICY_JSON);
    let _ = BerryFlavor::from_name("spicy").unwrap();
}
