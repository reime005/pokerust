use serde::{Deserialize, Serialize};

use super::utility::*;

use crate::cache::get_resource;
use crate::impl_from_id_and_name;
use crate::{FromId, FromName};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Berry {
    pub id: u64,
    pub name: String,
    pub growth_time: u64,
    pub max_harvest: u64,
    pub natural_gift_power: u64,
    pub size: u64,
    pub smoothness: u64,
    pub soil_dryness: u64,
    pub firmness: NamedAPIResource,
    pub flavors: Vec<BerryFlavorMap>,
    pub item: NamedAPIResource,
    pub natural_gift_type: NamedAPIResource,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BerryFlavorMap {
    pub potency: u64,
    pub flavor: NamedAPIResource,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BerryFirmness {
    pub id: u64,
    pub name: String,
    pub berries: Vec<NamedAPIResource>,
    pub names: Vec<Name>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BerryFlavor {
    pub id: u64,
    pub name: String,
    pub berries: Vec<FlavorBerryMap>,
    pub contest_type: NamedAPIResource,
    pub names: Vec<Name>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlavorBerryMap {
    pub potency: u64,
    pub berry: NamedAPIResource,
}

impl_from_id_and_name!(Berry, "berry");
impl_from_id_and_name!(BerryFirmness, "berry-firmness");
impl_from_id_and_name!(BerryFlavor, "berry-flavor");
