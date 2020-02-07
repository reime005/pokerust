use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{impl_id_and_named, impl_named, set_endpoint, Named};

use super::encounters::*;
use super::games::*;
use super::get_api_loc_from_url;
use super::machines::*;
use super::resource_lists::*;
use crate::cache::get_resource;

use std::marker::PhantomData;

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
pub struct APIResource<T> {
    pub url: String,
    #[serde(skip)]
    resource_type: PhantomData<*const T>,
}

impl<T> APIResource<T>
where
    T: DeserializeOwned,
{
    pub fn get(&self) -> Result<T, minreq::Error> {
        get_resource(get_api_loc_from_url(&self.url))?.json::<T>()
    }
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Description {
    pub description: String,
    pub language: NamedAPIResource<Language>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Effect {
    pub effect: String,
    pub language: NamedAPIResource<Language>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Encounter {
    pub min_level: u64,
    pub max_level: u64,
    pub condition_values: Vec<NamedAPIResource<EncounterConditionValue>>,
    pub chance: u64,
    pub method: NamedAPIResource<EncounterMethod>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlavorText {
    pub flavor_text: String,
    pub language: NamedAPIResource<Language>,
    pub version: Option<NamedAPIResource<Version>>, // sometimes this isn't provided, e.g. https://pokeapi.co/api/v2/contest-effect/9/
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenerationGameIndex {
    pub game_index: u64,
    pub generation: NamedAPIResource<Generation>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MachineVersionDetail {
    pub machine: APIResource<Machine>,
    pub version_group: NamedAPIResource<VersionGroup>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub name: String,
    pub language: NamedAPIResource<Language>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedAPIResource<T> {
    pub name: String,
    pub url: String,
    #[serde(skip)]
    resource_type: PhantomData<*const T>,
}

impl<T> Named for NamedAPIResource<T> {
    fn name(&self) -> &String {
        &self.name
    }
}

impl<T> NamedAPIResource<T>
where
    T: DeserializeOwned,
{
    pub fn get(&self) -> Result<T, minreq::Error> {
        get_resource(get_api_loc_from_url(&self.url))?.json::<T>()
    }
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VerboseEffect {
    pub effect: String,
    pub short_effect: String,
    pub language: NamedAPIResource<Language>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionEncounterDetail {
    pub version: NamedAPIResource<Version>,
    pub max_chance: u64,
    pub encounter_details: Vec<Encounter>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionGameIndex {
    pub game_index: u64,
    pub version: NamedAPIResource<Version>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionGroupFlavorText {
    pub text: String,
    pub language: NamedAPIResource<Language>,
    pub version_group: NamedAPIResource<VersionGroup>,
}

set_endpoint!(Language, NamedAPIResourceList, "language");

impl_named!(Name);
impl_id_and_named!(Language);
