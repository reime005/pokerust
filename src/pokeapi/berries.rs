use serde::{Deserialize, Serialize};

use super::utility::NamedApiResource;

use crate::cache::get_resource;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BerryFlavorMap {
    potency: u64,
    flavor: NamedApiResource,
}

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
    pub firmness: NamedApiResource,
    pub flavors: Vec<BerryFlavorMap>,
    pub item: NamedApiResource,
    pub natural_gift_type: NamedApiResource,
}

impl Berry {
    pub fn from_id(id: u64) -> Result<Self, minreq::Error> {
        get_resource(&format!("berry/{}/", id))?.json::<Self>()
    }

    pub fn from_name(name: &str) -> Result<Self, minreq::Error> {
        get_resource(&format!("berry/{}/", name))?.json::<Self>()
    }
}
