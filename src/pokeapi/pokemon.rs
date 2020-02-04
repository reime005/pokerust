use serde::{Deserialize, Serialize};

use super::utility::*;

use crate::{impl_id, impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ability {
    pub id: i64,
    pub name: String,
    pub is_main_series: bool,
    pub generation: NamedAPIResource,
    pub names: Vec<Name>,
    pub effect_entries: Vec<VerboseEffect>,
    pub effect_changes: Vec<AbilityEffectChange>,
    pub flavor_text_entries: Vec<AbilityFlavorText>,
    pub pokemon: Vec<AbilityPokemon>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbilityEffectChange {
    pub effect_entries: Vec<Effect>,
    pub version_group: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbilityFlavorText {
    pub flavor_text: String,
    pub language: NamedAPIResource,
    pub version_group: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbilityPokemon {
    pub is_hidden: bool,
    pub slot: u64,
    pub pokemon: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Characteristic {
    pub id: i64,
    pub gene_modulo: i64,
    pub possible_values: Vec<u64>,
    pub highest_stat: NamedAPIResource, // not documented
    pub descriptions: Vec<Description>, // not documented
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EggGroup {
    pub id: i64,
    pub name: String,
    pub names: Vec<Name>,
    pub pokemon_species: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gender {
    pub id: i64,
    pub name: String,
    pub pokemon_species_details: Vec<PokemonSpeciesGender>,
    pub required_for_evolution: Vec<NamedAPIResource>,
}

// #[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonSpeciesGender {
    pub rate: i64,
    pub pokemon_species: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrowthRate {
    pub id: i64,
    pub name: String,
    pub formula: String,
    pub descriptions: Vec<Description>,
    pub levels: Vec<GrowthRateExperienceLevel>,
    pub pokemon_species: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GrowthRateExperienceLevel {
    pub level: u64,
    pub experience: u64,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Nature {
    pub id: i64,
    pub name: String,
    pub decreased_stat: Option<NamedAPIResource>,
    pub increased_stat: Option<NamedAPIResource>,
    pub hates_flavor: Option<NamedAPIResource>,
    pub likes_flavor: Option<NamedAPIResource>,
    pub pokeathlon_stat_changes: Vec<NatureStatChange>,
    pub move_battle_style_preferences: Vec<MoveBattleStylePreference>,
    pub names: Vec<Name>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NatureStatChange {
    pub max_change: i64,
    pub pokeathlon_stat: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveBattleStylePreference {
    pub low_hp_preference: u64,
    pub high_hp_preference: u64,
    pub move_battle_style: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokeathlonStat {
    pub id: i64,
    pub name: String,
    pub names: Vec<Name>,
    pub affecting_natures: NaturePokeathlonStatAffectSets,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NaturePokeathlonStatAffectSets {
    pub increase: Vec<NaturePokeathlonStatAffect>,
    pub decrease: Vec<NaturePokeathlonStatAffect>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NaturePokeathlonStatAffect {
    pub max_change: i64,
    pub nature: NamedAPIResource,
}

// #[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pokemon {
    pub id: i64,
    pub name: String,
    pub base_experience: u64,
    pub height: u64,
    pub is_default: bool,
    pub order: u64,
    pub weight: u64,
    pub abilities: Vec<PokemonAbility>,
    pub forms: Vec<NamedAPIResource>,
    pub game_indices: Vec<VersionGameIndex>,
    pub held_items: Vec<PokemonHeldItem>,
    pub location_area_encounters: String, // TODO: implement a way to retrieve these
    pub moves: Vec<PokemonMove>,
    pub sprites: PokemonSprites,
    pub species: NamedAPIResource,
    pub stats: Vec<PokemonStat>,
    pub types: Vec<PokemonType>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: u64,
    pub ability: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonType {
    pub slot: u64,
    #[serde(rename = "type")]
    pub type_: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonHeldItem {
    pub item: NamedAPIResource,
    pub version_details: Vec<PokemonHeldItemVersion>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonHeldItemVersion {
    pub version: NamedAPIResource,
    pub rarity: u64,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub move_: NamedAPIResource,
    pub version_group_details: Vec<PokemonMoveVersion>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonMoveVersion {
    pub move_learn_method: NamedAPIResource,
    pub version_group: NamedAPIResource,
    pub level_learned_at: u64,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonStat {
    pub stat: NamedAPIResource,
    pub effort: u64,
    pub base_stat: u64,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonSprites {
    pub front_default: String,
    pub front_shiny: String,
    pub front_female: String,
    pub front_shiny_female: String,
    pub back_default: String,
    pub back_shiny: String,
    pub back_female: String,
    pub back_shiny_female: String,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocationAreaEncounter {
    pub location_area: NamedAPIResource,
    pub version_details: Vec<VersionEncounterDetail>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonColor {
    pub id: i64,
    pub name: String,
    pub names: Vec<Name>,
    pub pokemon_species: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonForm {
    pub id: i64,
    pub name: String,
    pub order: u64,
    pub form_order: u64,
    pub is_default: bool,
    pub is_battle_only: bool,
    pub is_mega: bool,
    pub form_name: String,
    pub pokemon: NamedAPIResource,
    pub sprites: PokemonFormSprites,
    pub version_group: NamedAPIResource,
    pub names: Vec<Name>,
    pub form_names: Vec<Name>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonFormSprites {
    pub front_default: String,
    pub front_shiny: Option<String>,
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonHabitat {
    pub id: i64,
    pub name: String,
    pub names: Vec<Name>,
    pub pokemon_species: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonShape {
    pub id: i64,
    pub name: String,
    pub awesome_names: Vec<AwesomeName>,
    pub names: Vec<Name>,
    pub pokemon_species: Vec<NamedAPIResource>, // incorrectly documented as list PokemonSpecies
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AwesomeName {
    pub awesome_name: String,
    pub language: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonSpecies {
    pub id: i64,
    pub name: String,
    pub order: u64,
    pub gender_rate: u64,
    pub capture_rate: u64,
    pub base_happiness: u64,
    pub is_baby: bool,
    pub hatch_counter: u64,
    pub has_gender_differences: bool,
    pub forms_switchable: bool,
    pub growth_rate: NamedAPIResource,
    pub pokedex_numbers: Vec<PokemonSpeciesDexEntry>,
    pub egg_groups: Vec<NamedAPIResource>,
    pub color: NamedAPIResource,
    pub shape: NamedAPIResource,
    pub evolves_from_species: Option<NamedAPIResource>,
    pub evolution_chain: APIResource,
    pub habitat: NamedAPIResource,
    pub generation: NamedAPIResource,
    pub names: Vec<Name>,
    pub pal_park_encounters: Vec<PalParkEncounterArea>,
    pub flavor_text_entries: Vec<FlavorText>,
    pub form_descriptions: Vec<Description>,
    pub genera: Vec<Genus>,
    pub varieties: Vec<PokemonSpeciesVariety>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Genus {
    pub genus: String,
    pub language: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonSpeciesDexEntry {
    pub entry_number: u64,
    pub pokedex: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PalParkEncounterArea {
    pub base_score: u64,
    pub rate: u64,
    pub area: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PokemonSpeciesVariety {
    pub is_default: bool,
    pub pokemon: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stat {
    pub id: i64,
    pub name: String,
    pub game_index: u64,
    pub is_battle_only: bool,
    pub affecting_moves: MoveStatAffectSets,
    pub affecting_natures: NatureStatAffectSets,
    pub characteristics: Vec<APIResource>, // incorrectly documented as APIResource
    pub move_damage_class: Option<NamedAPIResource>,
    pub names: Vec<Name>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveStatAffectSets {
    pub increase: Vec<MoveStatAffect>,
    pub decrease: Vec<MoveStatAffect>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveStatAffect {
    pub change: i64,
    #[serde(rename = "move")]
    pub move_: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NatureStatAffectSets {
    pub increase: Vec<NamedAPIResource>,
    pub decrease: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Type {
    pub id: i64,
    pub name: String,
    pub damage_relations: TypeRelations,
    pub game_indices: Vec<GenerationGameIndex>,
    pub generation: NamedAPIResource,
    pub move_damage_class: NamedAPIResource,
    pub names: Vec<Name>,
    pub pokemon: Vec<TypePokemon>,
    pub moves: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypePokemon {
    pub slot: u64,
    pub pokemon: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeRelations {
    pub no_damage_to: Vec<NamedAPIResource>,
    pub half_damage_to: Vec<NamedAPIResource>,
    pub double_damage_to: Vec<NamedAPIResource>,
    pub no_damage_from: Vec<NamedAPIResource>,
    pub half_damage_from: Vec<NamedAPIResource>,
    pub double_damage_from: Vec<NamedAPIResource>,
}

set_endpoint!(Ability, "ability");
set_endpoint!(Characteristic, "characteristic");
set_endpoint!(EggGroup, "egg-group");
set_endpoint!(Gender, "gender");
set_endpoint!(GrowthRate, "growth-rate");
set_endpoint!(Nature, "nature");
set_endpoint!(PokeathlonStat, "pokeathlon-stat");
set_endpoint!(Pokemon, "pokemon");
set_endpoint!(PokemonColor, "pokemon-color");
set_endpoint!(PokemonForm, "pokemon-form");
set_endpoint!(PokemonHabitat, "pokemon-habitat");
set_endpoint!(PokemonShape, "pokemon-shape");
set_endpoint!(PokemonSpecies, "pokemon-species");
set_endpoint!(Stat, "stat");
set_endpoint!(Type, "type");

impl_id!(Characteristic);
impl_id_and_named!(Ability);
impl_id_and_named!(EggGroup);
impl_id_and_named!(Gender);
impl_id_and_named!(GrowthRate);
impl_id_and_named!(Nature);
impl_id_and_named!(PokeathlonStat);
impl_id_and_named!(Pokemon);
impl_id_and_named!(PokemonColor);
impl_id_and_named!(PokemonForm);
impl_id_and_named!(PokemonHabitat);
impl_id_and_named!(PokemonShape);
impl_id_and_named!(PokemonSpecies);
impl_id_and_named!(Stat);
impl_id_and_named!(Type);
