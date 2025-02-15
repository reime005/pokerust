use serde::{Deserialize, Serialize};

use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    pub id: i64,
    pub name: String,
    pub region: NamedAPIResource,
    pub names: Vec<Name>,
    pub game_indices: Vec<GenerationGameIndex>,
    pub areas: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocationArea {
    pub id: i64,
    pub name: String,
    pub game_index: u64,
    pub encounter_method_rates: Vec<EncouterMethodRate>,
    pub location: NamedAPIResource,
    pub names: Vec<Name>,
    pub pokemon_encounters: Vec<PokemonEncouter>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncouterMethodRate {
    pub encounter_method: NamedAPIResource,
    pub version_details: Vec<EncounterVersionDetails>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncounterVersionDetails {
    pub rate: u64,
    pub version: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonEncouter {
    pub pokemon: NamedAPIResource,
    pub version_details: Vec<VersionEncounterDetail>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PalParkArea {
    pub id: i64,
    pub name: String,
    pub names: Vec<Name>,
    pub pokemon_encounters: Vec<PalParkEncounterSpecies>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PalParkEncounterSpecies {
    pub base_score: u64,
    pub rate: u64,
    pub pokemon_species: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Region {
    pub id: i64,
    pub locations: Vec<NamedAPIResource>,
    pub name: String,
    pub names: Vec<Name>,
    pub main_generation: NamedAPIResource,
    pub pokedexes: Vec<NamedAPIResource>,
    pub version_groups: Vec<NamedAPIResource>,
}

set_endpoint!(Location, "location");
set_endpoint!(LocationArea, "location-area");
set_endpoint!(PalParkArea, "pal-park-area");
set_endpoint!(Region, "region");

impl_id_and_named!(Location);
impl_id_and_named!(LocationArea);
impl_id_and_named!(PalParkArea);
impl_id_and_named!(Region);
