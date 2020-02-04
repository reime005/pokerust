#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod cache;
mod pokeapi;

#[macro_use]
mod util;

pub use pokeapi::berries::*;

pub trait FromId where Self: Sized {
    fn from_id(id: u64) -> Result<Self, minreq::Error>;
}

pub trait FromName where Self: Sized {
    fn from_name(id: &str) -> Result<Self, minreq::Error>;
}
