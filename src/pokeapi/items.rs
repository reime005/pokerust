use serde::{Deserialize, Serialize};

use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub cost: u64,
    pub fling_power: Option<u64>,
    pub fling_effect: Option<NamedAPIResource>,
    pub attributes: Vec<NamedAPIResource>,
    pub category: NamedAPIResource, // incorrectly documented as ItemCategory
    pub effect_entries: Vec<VerboseEffect>,
    pub flavor_text_entries: Vec<VersionGroupFlavorText>,
    pub game_indices: Vec<GenerationGameIndex>,
    pub names: Vec<Name>,
    pub sprites: ItemSprites,
    pub held_by_pokemon: Vec<ItemHolderPokemon>,
    pub baby_trigger_for: Option<APIResource>,
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
    pub pokemon: String,
    pub version_details: Vec<ItemHolderPokemonVersionDetail>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemHolderPokemonVersionDetail {
    pub rarity: String,
    pub version: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemAttribute {
    pub id: i64,
    pub name: String,
    pub items: Vec<NamedAPIResource>,
    pub names: Vec<Name>,
    pub descriptions: Vec<Description>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemCategory {
    pub id: i64,
    pub name: String,
    pub items: Vec<NamedAPIResource>,
    pub names: Vec<Name>,
    pub pocket: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemFlingEffect {
    pub id: i64,
    pub name: String,
    pub effect_entries: Vec<Effect>,
    pub items: Vec<NamedAPIResource>, // incorrectly documented as NamedAPIResource
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemPocket {
    pub id: i64,
    pub name: String,
    pub categories: Vec<NamedAPIResource>,
    pub names: Vec<Name>,
}

set_endpoint!(Item, "item");
set_endpoint!(ItemAttribute, "item-attribute");
set_endpoint!(ItemCategory, "item-category");
set_endpoint!(ItemFlingEffect, "item-fling-effect");
set_endpoint!(ItemPocket, "item-pocket");

impl_id_and_named!(Item);
impl_id_and_named!(ItemAttribute);
impl_id_and_named!(ItemCategory);
impl_id_and_named!(ItemFlingEffect);
impl_id_and_named!(ItemPocket);
