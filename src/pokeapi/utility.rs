use serde::{Deserialize, Serialize};

use crate::{impl_id_and_named, impl_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Language {
    pub id: i64,
    pub name: String,
    pub official: bool,
    pub iso639: String,
    pub iso3166: String,
    pub names: Vec<Name>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct APIResource {
    pub url: String,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Description {
    pub description: String,
    pub language: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Effect {
    pub effect: String,
    pub language: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Encounter {
    pub min_level: u64,
    pub max_level: u64,
    pub condition_values: Vec<NamedAPIResource>,
    pub chance: u64,
    pub method: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlavorText {
    pub flavor_text: String,
    pub language: NamedAPIResource,
    pub version: Option<NamedAPIResource>, // sometimes this isn't provided, e.g. https://pokeapi.co/api/v2/contest-effect/9/
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenerationGameIndex {
    pub game_index: u64,
    pub generation: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MachineVersionDetail {
    pub machine: APIResource,
    pub version_group: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub name: String,
    pub language: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedAPIResource {
    pub name: String,
    pub url: String,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VerboseEffect {
    pub effect: String,
    pub short_effect: String,
    pub language: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionEncounterDetail {
    pub version: NamedAPIResource,
    pub max_chance: u64,
    pub encounter_details: Vec<Encounter>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionGameIndex {
    pub game_index: u64,
    pub version: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionGroupFlavorText {
    pub text: String,
    pub language: NamedAPIResource,
    pub version_group: NamedAPIResource,
}

set_endpoint!(Language, "language");

impl_named!(Name);
impl_named!(NamedAPIResource);
impl_id_and_named!(Language);
