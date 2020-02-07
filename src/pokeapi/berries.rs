use serde::{Deserialize, Serialize};

use super::contests::*;
use super::items::*;
use super::pokemon::*;
use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Berry {
    pub id: i64,
    pub name: String,
    pub growth_time: u64,
    pub max_harvest: u64,
    pub natural_gift_power: u64,
    pub size: u64,
    pub smoothness: u64,
    pub soil_dryness: u64,
    pub firmness: NamedAPIResource<BerryFirmness>,
    pub flavors: Vec<BerryFlavorMap>,
    pub item: NamedAPIResource<Item>,
    pub natural_gift_type: NamedAPIResource<Type>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BerryFlavorMap {
    pub potency: u64,
    pub flavor: NamedAPIResource<BerryFlavor>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BerryFirmness {
    pub id: i64,
    pub name: String,
    pub berries: Vec<NamedAPIResource<Berry>>,
    pub names: Vec<Name>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BerryFlavor {
    pub id: i64,
    pub name: String,
    pub berries: Vec<FlavorBerryMap>,
    pub contest_type: NamedAPIResource<ContestType>,
    pub names: Vec<Name>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlavorBerryMap {
    pub potency: u64,
    pub berry: NamedAPIResource<Berry>,
}

set_endpoint!(Berry, NamedAPIResourceList, "berry");
set_endpoint!(BerryFirmness, NamedAPIResourceList, "berry-firmness");
set_endpoint!(BerryFlavor, NamedAPIResourceList, "berry-flavor");

impl_id_and_named!(Berry);
impl_id_and_named!(BerryFirmness);
impl_id_and_named!(BerryFlavor);
