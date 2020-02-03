#![warn(clippy::all, clippy::pedantic)]
#![feature(proc_macro_hygiene)]

mod cache;
mod pokeapi;

pub use pokeapi::berries::*;
