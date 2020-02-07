use serde::{Deserialize, Serialize};

use super::berries::*;
use super::moves::*;
use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id, impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContestType {
    pub id: i64,
    pub name: String,
    pub berry_flavor: NamedAPIResource<BerryFlavor>,
    pub names: Vec<ContestName>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContestName {
    pub name: String,
    pub color: String,
    pub language: NamedAPIResource<Language>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContestEffect {
    pub id: i64,
    pub appeal: u64,
    pub jam: u64,
    pub effect_entries: Vec<Effect>,
    pub flavor_text_entries: Vec<FlavorText>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SuperContestEffect {
    pub id: i64,
    pub appeal: u64,
    pub flavor_text_entries: Vec<FlavorText>,
    pub moves: Vec<NamedAPIResource<Move>>,
}

set_endpoint!(ContestEffect, APIResourceList, "contest-effect");
set_endpoint!(SuperContestEffect, APIResourceList, "super-contest-effect");
set_endpoint!(ContestType, NamedAPIResourceList, "contest-type");

impl_id!(ContestEffect);
impl_id!(SuperContestEffect);
impl_id_and_named!(ContestType);
