use serde::{Deserialize, Serialize};

use super::utility::*;

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct APIResourceList {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<APIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedAPIResourceList {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<NamedAPIResource>,
}

fn get_query_from_url(url: &str) -> &str {
    &url[(url.rfind('/').unwrap() + 1)..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_query_from_url() {
        assert_eq!(get_query_from_url("https://pokeapi.co/api/v2/ability/?offset=20&limit=20"), "?offset=20&limit=20");
        assert_eq!(get_query_from_url("http://localhost:8000/api/v2/pokemon/?limit=0&offset=42"), "?limit=0&offset=42");
    }
}
