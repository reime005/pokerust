use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::utility::*;

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct APIResourceList<T> {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<APIResource<T>>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedAPIResourceList<T> {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<NamedAPIResource<T>>,
}

pub trait List
where
    Self: Sized,
{
    fn count(&self) -> &u64;

    fn next_list(&self) -> Result<Option<Self>, minreq::Error>;

    fn previous_list(&self) -> Result<Option<Self>, minreq::Error>;
}

/// Gets the location of an API resource from a full url, minus the url
/// and common prefix, e.g. "https://pokeapi.co/api/v2/"
fn get_api_loc_from_url(url: &str) -> &str {
    let pre = "api/v2/";
    &url[(url.rfind(pre).unwrap() + pre.len())..]
}

impl<T> List for NamedAPIResourceList<T>
where
    T: DeserializeOwned,
{
    fn count(&self) -> &u64 {
        &self.count
    }

    fn next_list(&self) -> Result<Option<Self>, minreq::Error> {
        if let Some(loc) = &self.next {
            let list = crate::cache::get_resource(get_api_loc_from_url(&loc))?.json::<Self>()?;
            Ok(Some(list))
        } else {
            Ok(None)
        }
    }

    fn previous_list(&self) -> Result<Option<Self>, minreq::Error> {
        if let Some(loc) = &self.next {
            let list = crate::cache::get_resource(get_api_loc_from_url(&loc))?.json::<Self>()?;
            Ok(Some(list))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_api_loc_from_url() {
        assert_eq!(
            get_api_loc_from_url("https://pokeapi.co/api/v2/ability/?offset=20&limit=20"),
            "ability/?offset=20&limit=20"
        );
        assert_eq!(
            get_api_loc_from_url("http://localhost:8000/api/v2/pokemon/?limit=0&offset=42"),
            "pokemon/?limit=0&offset=42"
        );
        assert_eq!(
            get_api_loc_from_url("https://pokeapi.co/api/v2/api/v2/ability/?offset=20&limit=20"),
            "ability/?offset=20&limit=20"
        );
    }
}
