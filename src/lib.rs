//! # pokerust
//!
//! Rust wrapper for the pokeapi <https://pokeapi.co/>
//!
//! ## Basic Usage
//!
//! Get an object from an API by id
//! ```ignore,no_run
//! use pokerust::{Berry, FromId};
//!
//! fn main() {
//!     let berry = Berry::from_id(1).unwrap();
//! }
//! ```
//! or by name
//! ```ignore,no_run
//! use pokerust::{Berry, FromName};
//!
//! fn main() {
//!     let berry = Berry::from_name("cheri").unwrap();
//! }
//! ```
//! API responses are automatically cached.
//!
//! You can also fetch the resource lists:
//! ```ignore,no_run
//! use pokerust::{Item, Endpoint, List};
//!
//! fn main() {
//!     let items = Item::list(5, 20).unwrap();  // ?offset=5&limit=20
//!
//!     // get the lists referenced in the next and previous fields
//!     items.previous_list().unwrap();
//!     items.next_list().unwrap();
//!
//!     // you can also just get the full list
//!     let all_items = Item::full_list().unwrap();
//! }
//! ```
//!
//! To get resources pointed to by `(Named)APIResource`, use `get()`:
//! ```ignore,no_run
//! # use pokerust::{Berry, FromName};
//! # fn main() {
//! let berry = Berry::from_name("cheri").unwrap();
//! let berry_item = berry.item.get().unwrap(); // berry_item is an Item
//! # }
//! ```
//! This can be chained:
//! ```ignore,no_run
//! # use pokerust::{PokemonSpecies, FromName};
//! # fn main() {
//! let marill = PokemonSpecies::from_name("marill").unwrap();
//! let sea_incense = marill.evolution_chain.get().unwrap().baby_trigger_item.unwrap().get().unwrap();
//! # }
//! ```
//!
//! The location of the pokeapi used can be changed by setting the
//! POKERUST_ENDPOINT environment variable. Defaults to the public instance at
//! <https://pokeapi.co/api/v2/>. Please consult the pokeapi documentation and read
//! the fair use policy before using the public API instance.

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use serde::de::DeserializeOwned;

use crate::cache::get_resource;

mod cache;
mod pokeapi;
mod util;

pub use cache::ENDPOINT;
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

/// Trait for API objects with an associated endpoint.
pub trait Endpoint {
    type ResourceListKind;

    const ENDPOINT: &'static str;

    /// Get a list of these API objects with the given offset and limit.
    fn list(offset: usize, limit: usize) -> Result<Self::ResourceListKind, minreq::Error>;

    /// Get the complete list of these API objects.
    fn full_list() -> Result<Self::ResourceListKind, minreq::Error>;
}

/// Trait for API objects with a name.
pub trait Named {
    /// Get the name of this object.
    fn name(&self) -> &String;
}

/// Trait for API objects with an id.
pub trait Id {
    /// Get the id of this object.
    fn id(&self) -> i16;
}

pub trait FromId: Id + Endpoint
where
    Self: Sized,
{
    /// Retrieve the API object of this type with this id.
    fn from_id(id: i16) -> Result<Self, minreq::Error>;
}

impl<T: Endpoint + Id + DeserializeOwned> FromId for T {
    fn from_id(id: i16) -> Result<Self, minreq::Error> {
        get_resource(&format!("{}/{}/", T::ENDPOINT, id))?.json::<Self>()
    }
}

pub trait FromName: Named + Endpoint
where
    Self: Sized,
{
    /// Retrieve the API object of this type with this name.
    fn from_name(id: &str) -> Result<Self, minreq::Error>;
}

impl<T: Endpoint + Named + DeserializeOwned> FromName for T {
    fn from_name(id: &str) -> Result<Self, minreq::Error> {
        get_resource(&format!("{}/{}/", T::ENDPOINT, id))?.json::<Self>()
    }
}
