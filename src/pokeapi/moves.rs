use serde::{Deserialize, Serialize};

use super::pokemon::AbilityEffectChange;
use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

// #[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub id: i64,
    pub name: String,
    pub accuracy: u64,
    pub effect_chance: Option<u64>,
    pub pp: u64,
    pub priority: i64,
    pub power: Option<u64>,
    pub contest_combos: Option<ContestComboSets>,
    pub contest_type: NamedAPIResource,
    pub contest_effect: APIResource,
    pub damage_class: NamedAPIResource,
    pub effect_entries: Vec<VerboseEffect>,
    pub effect_changes: Vec<AbilityEffectChange>,
    pub flavor_text_entries: Vec<MoveFlavorText>,
    pub generation: NamedAPIResource,
    pub machines: Vec<MachineVersionDetail>,
    pub meta: MoveMetaData,
    pub names: Vec<Name>,
    pub past_values: Vec<PastMoveStatValues>,
    pub stat_changes: Vec<MoveStatChange>,
    pub super_contest_effect: APIResource,
    pub target: NamedAPIResource,
    #[serde(rename = "type")]
    pub type_: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContestComboSets {
    pub normal: ContestComboDetail,
    #[serde(rename = "super")]
    pub super_: ContestComboDetail,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContestComboDetail {
    pub use_before: Vec<NamedAPIResource>,
    pub use_after: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveFlavorText {
    pub flavor_text: String,
    pub language: NamedAPIResource, // incorrectly documented as list NamedAPIResource (Move)
    pub version_group: NamedAPIResource, // incorrectly documented as list NamedAPIResource (Move)
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveMetaData {
    pub ailment: NamedAPIResource,
    pub category: NamedAPIResource,
    pub min_hits: Option<u64>,
    pub max_hits: Option<u64>,
    pub min_turns: u64,
    pub max_turns: u64,
    pub drain: i64,
    pub healing: u64,
    pub crit_rate: u64,
    pub ailment_chance: u64,
    pub flinch_chance: u64,
    pub stat_chance: u64,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveStatChange {
    pub change: i64,
    pub stat: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PastMoveStatValues {
    pub accuracy: u64,
    pub effect_chance: Option<u64>,
    pub power: Option<u64>,
    pub pp: Option<u64>,
    pub effect_entries: Vec<VerboseEffect>,
    #[serde(rename = "type")]
    type_: Option<NamedAPIResource>,
    pub version_group: NamedAPIResource,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveAilment {
    pub id: i64,
    pub name: String,
    pub moves: Vec<NamedAPIResource>,
    pub names: Vec<Name>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveBattleStyle {
    pub id: i64,
    pub name: String,
    pub names: Vec<Name>,
}

// incorrectly documeted as ModelName
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveCategory {
    pub id: i64,
    pub name: String,
    pub moves: Vec<NamedAPIResource>,
    pub descriptions: Vec<Description>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveDamageClass {
    pub id: i64,
    pub name: String,
    pub descriptions: Vec<Description>,
    pub moves: Vec<NamedAPIResource>,
    pub names: Vec<Name>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveLearnMethod {
    pub id: i64,
    pub name: String,
    pub descriptions: Vec<Description>,
    pub names: Vec<Name>,
    pub version_groups: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveTarget {
    pub id: i64,
    pub name: String,
    pub descriptions: Vec<Description>,
    pub moves: Vec<NamedAPIResource>,
    pub names: Vec<Name>,
}

set_endpoint!(Move, "move");
set_endpoint!(MoveAilment, "move-ailment");
set_endpoint!(MoveBattleStyle, "move-battle-style");
set_endpoint!(MoveCategory, "move-category");
set_endpoint!(MoveDamageClass, "move-damage-class");
set_endpoint!(MoveTarget, "move-target");

impl_id_and_named!(Move);
impl_id_and_named!(MoveAilment);
impl_id_and_named!(MoveBattleStyle);
impl_id_and_named!(MoveCategory);
impl_id_and_named!(MoveDamageClass);
impl_id_and_named!(MoveTarget);
