#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use serde::de::DeserializeOwned;

mod cache;
mod pokeapi;

#[macro_use]
mod util;

pub use pokeapi::berries::*;
pub use pokeapi::contests::*;
pub use pokeapi::encounters::*;
pub use pokeapi::evolution::*;
pub use pokeapi::games::*;
pub use pokeapi::items::*;
pub use pokeapi::locations::*;
pub use pokeapi::machines::*;
pub use pokeapi::moves::*;
pub use pokeapi::pokemon::*;
pub use pokeapi::resource_lists::*;
pub use pokeapi::utility::*;

pub trait Endpoint {
    type ResourceListKind;

    const ENDPOINT: &'static str;

    fn list(offset: usize, limit: usize) -> Result<Self::ResourceListKind, ::minreq::Error>;

    fn full_list() -> Result<Self::ResourceListKind, ::minreq::Error>;
}

pub trait Named {
    fn name(&self) -> &String;
}

pub trait Id {
    fn id(&self) -> &i64;
}

pub trait FromId: Id + Endpoint
where
    Self: Sized,
{
    fn from_id(id: i64) -> Result<Self, minreq::Error>;
}

impl<T: Endpoint + Id + DeserializeOwned> FromId for T {
    fn from_id(id: i64) -> Result<Self, ::minreq::Error> {
        crate::cache::get_resource(&format!("{}/{}/", T::ENDPOINT, id))?.json::<Self>()
    }
}

pub trait FromName: Named + Endpoint
where
    Self: Sized,
{
    fn from_name(id: &str) -> Result<Self, minreq::Error>;
}

impl<T: Endpoint + Named + DeserializeOwned> FromName for T {
    fn from_name(id: &str) -> Result<Self, ::minreq::Error> {
        crate::cache::get_resource(&format!("{}/{}/", T::ENDPOINT, id))?.json::<Self>()
    }
}
