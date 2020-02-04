use serde::{Deserialize, Serialize};

use super::utility::*;

use crate::{impl_id, impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvolutionChain {
    pub id: i64,
    pub baby_trigger_item: Option<NamedAPIResource>,
    pub chain: ChainLink,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChainLink {
    pub is_baby: bool,
    pub species: NamedAPIResource,
    pub evolution_details: Vec<EvolutionDetail>,
    pub evolves_to: Vec<ChainLink>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvolutionDetail {
    pub item: Option<NamedAPIResource>,
    pub trigger: NamedAPIResource,
    pub gender: Option<u64>,
    pub held_item: Option<NamedAPIResource>,
    pub known_move: Option<NamedAPIResource>,
    pub known_move_type: Option<NamedAPIResource>,
    pub location: Option<NamedAPIResource>,
    pub min_level: Option<u64>,
    pub min_happiness: Option<u64>,
    pub min_beauty: Option<u64>,
    pub min_affection: Option<u64>,
    pub needs_overworld_rain: bool,
    pub party_species: Option<NamedAPIResource>,
    pub party_type: Option<NamedAPIResource>,
    pub relative_physical_stats: Option<u64>,
    pub time_of_day: String,
    pub trade_species: Option<NamedAPIResource>,
    pub turn_upside_down: bool,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvolutionTrigger {
    pub id: i64,
    pub name: String,
    pub names: Vec<Name>,
    pub pokemon_species: Vec<NamedAPIResource>,
}

set_endpoint!(EvolutionChain, "evolution-chain");
set_endpoint!(EvolutionTrigger, "evolution-trigger");

impl_id!(EvolutionChain);
impl_id_and_named!(EvolutionTrigger);
