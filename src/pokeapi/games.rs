use serde::{Deserialize, Serialize};

use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Generation {
    pub id: i64,
    pub name: String,
    pub abilities: Vec<NamedAPIResource>,
    pub names: Vec<Name>,
    pub main_region: NamedAPIResource,
    pub moves: Vec<NamedAPIResource>,
    pub pokemon_species: Vec<NamedAPIResource>,
    pub types: Vec<NamedAPIResource>,
    pub version_groups: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pokedex {
    pub id: i64,
    pub name: String,
    pub is_main_series: bool,
    pub descriptions: Vec<Description>,
    pub names: Vec<Name>,
    pub pokemon_entries: Vec<PokemonEntry>,
    pub region: Option<NamedAPIResource>,
    pub version_groups: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonEntry {
    pub entry_number: u64,
    pub pokemon_species: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub id: i64,
    pub name: String,
    pub names: Vec<Name>,
    pub version_group: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionGroup {
    pub id: i64,
    pub name: String,
    pub order: u64,
    pub generation: NamedAPIResource,
    pub move_learn_methods: Vec<NamedAPIResource>,
    pub pokedexes: Vec<NamedAPIResource>,
    pub regions: Vec<NamedAPIResource>,
    pub versions: Vec<NamedAPIResource>,
}

set_endpoint!(Generation, NamedAPIResourceList, "generation");
set_endpoint!(Pokedex, NamedAPIResourceList, "pokedex");
set_endpoint!(Version, NamedAPIResourceList, "version");
set_endpoint!(VersionGroup, NamedAPIResourceList, "version-group");

impl_id_and_named!(Generation);
impl_id_and_named!(Pokedex);
impl_id_and_named!(Version);
impl_id_and_named!(VersionGroup);
