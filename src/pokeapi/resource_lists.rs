use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::get_api_loc_from_url;
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
