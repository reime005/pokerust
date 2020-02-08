use serde::{Deserialize, Serialize};

use super::evolution::*;
use super::games::*;
use super::resource_lists::*;
use super::utility::*;
use super::pokemon::*;

use crate::{impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub cost: u64,
    pub fling_power: Option<u64>,
    pub fling_effect: Option<NamedAPIResource<ItemFlingEffect>>,
    pub attributes: Vec<NamedAPIResource<ItemAttribute>>,
    pub category: NamedAPIResource<ItemCategory>, // incorrectly documented as ItemCategory
    pub effect_entries: Vec<VerboseEffect>,
    pub flavor_text_entries: Vec<VersionGroupFlavorText>,
    pub game_indices: Vec<GenerationGameIndex>,
    pub names: Vec<Name>,
    pub sprites: ItemSprites,
    pub held_by_pokemon: Vec<ItemHolderPokemon>,
    pub baby_trigger_for: Option<APIResource<EvolutionChain>>,
    pub machines: Vec<MachineVersionDetail>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemSprites {
    pub default: String,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemHolderPokemon {
    pub pokemon: NamedAPIResource<Pokemon>, // incorrectly documented as string
    pub version_details: Vec<ItemHolderPokemonVersionDetail>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemHolderPokemonVersionDetail {
    pub rarity: u64, // incorrectly documented as string
    pub version: NamedAPIResource<Version>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemAttribute {
    pub id: i64,
    pub name: String,
    pub items: Vec<NamedAPIResource<Item>>,
    pub names: Vec<Name>,
    pub descriptions: Vec<Description>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemCategory {
    pub id: i64,
    pub name: String,
    pub items: Vec<NamedAPIResource<Item>>,
    pub names: Vec<Name>,
    pub pocket: NamedAPIResource<ItemPocket>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemFlingEffect {
    pub id: i64,
    pub name: String,
    pub effect_entries: Vec<Effect>,
    pub items: Vec<NamedAPIResource<Item>>, // incorrectly documented as NamedAPIResource
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemPocket {
    pub id: i64,
    pub name: String,
    pub categories: Vec<NamedAPIResource<ItemCategory>>,
    pub names: Vec<Name>,
}

set_endpoint!(Item, NamedAPIResourceList, "item");
set_endpoint!(ItemAttribute, NamedAPIResourceList, "item-attribute");
set_endpoint!(ItemCategory, NamedAPIResourceList, "item-category");
set_endpoint!(ItemFlingEffect, NamedAPIResourceList, "item-fling-effect");
set_endpoint!(ItemPocket, NamedAPIResourceList, "item-pocket");

impl_id_and_named!(Item);
impl_id_and_named!(ItemAttribute);
impl_id_and_named!(ItemCategory);
impl_id_and_named!(ItemFlingEffect);
impl_id_and_named!(ItemPocket);
