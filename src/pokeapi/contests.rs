use serde::{Deserialize, Serialize};

use super::utility::*;

use crate::{impl_id, impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContestType {
    pub id: i64,
    pub name: String,
    pub berry_flavor: NamedAPIResource,
    pub names: Vec<ContestName>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContestName {
    pub name: String,
    pub color: String,
    pub language: NamedAPIResource,
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
    pub moves: Vec<NamedAPIResource>,
}

set_endpoint!(ContestEffect, "contest-effect");
set_endpoint!(SuperContestEffect, "super-contest-effect");
set_endpoint!(ContestType, "contest-type");

impl_id!(ContestEffect);
impl_id!(SuperContestEffect);
impl_id_and_named!(ContestType);
