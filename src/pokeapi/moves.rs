use serde::{Deserialize, Serialize};

use super::contests::*;
use super::games::*;
use super::pokemon::AbilityEffectChange;
use super::pokemon::*;
use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

// #[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub id: i64,
    pub name: String,
    pub accuracy: Option<u64>,
    pub effect_chance: Option<u64>,
    pub pp: Option<u64>,
    pub priority: i64,
    pub power: Option<u64>,
    pub contest_combos: Option<ContestComboSets>,
    pub contest_type: Option<NamedAPIResource<ContestType>>,
    pub contest_effect: Option<APIResource<ContestEffect>>,
    pub damage_class: NamedAPIResource<MoveDamageClass>,
    pub effect_entries: Vec<VerboseEffect>,
    pub effect_changes: Vec<AbilityEffectChange>,
    pub flavor_text_entries: Vec<MoveFlavorText>,
    pub generation: NamedAPIResource<Generation>,
    pub machines: Vec<MachineVersionDetail>,
    pub meta: Option<MoveMetaData>,
    pub names: Vec<Name>,
    pub past_values: Vec<PastMoveStatValues>,
    pub stat_changes: Vec<MoveStatChange>,
    pub super_contest_effect: Option<APIResource<SuperContestEffect>>,
    pub target: NamedAPIResource<MoveTarget>,
    #[serde(rename = "type")]
    pub type_: NamedAPIResource<Type>,
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
    pub use_before: Option<Vec<NamedAPIResource<Move>>>,
    pub use_after: Option<Vec<NamedAPIResource<Move>>>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveFlavorText {
    pub flavor_text: String,
    pub language: NamedAPIResource<Language>, // incorrectly documented as list NamedAPIResource (Move)
    pub version_group: NamedAPIResource<VersionGroup>, // incorrectly documented as list NamedAPIResource (Move)
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveMetaData {
    pub ailment: NamedAPIResource<MoveAilment>,
    pub category: NamedAPIResource<MoveCategory>, // incorrectly documented as NamedApiResource (Move)
    pub min_hits: Option<u64>,
    pub max_hits: Option<u64>,
    pub min_turns: Option<u64>,
    pub max_turns: Option<u64>,
    pub drain: i64,
    pub healing: i64,
    pub crit_rate: u64,
    pub ailment_chance: u64,
    pub flinch_chance: u64,
    pub stat_chance: u64,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveStatChange {
    pub change: i64,
    pub stat: NamedAPIResource<Stat>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PastMoveStatValues {
    pub accuracy: Option<u64>,
    pub effect_chance: Option<u64>,
    pub power: Option<u64>,
    pub pp: Option<u64>,
    pub effect_entries: Vec<VerboseEffect>,
    #[serde(rename = "type")]
    type_: Option<NamedAPIResource<Type>>,
    pub version_group: NamedAPIResource<VersionGroup>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveAilment {
    pub id: i64,
    pub name: String,
    pub moves: Vec<NamedAPIResource<Move>>,
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
    pub moves: Vec<NamedAPIResource<Move>>,
    pub descriptions: Vec<Description>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveDamageClass {
    pub id: i64,
    pub name: String,
    pub descriptions: Vec<Description>,
    pub moves: Vec<NamedAPIResource<Move>>,
    pub names: Vec<Name>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveLearnMethod {
    pub id: i64,
    pub name: String,
    pub descriptions: Vec<Description>,
    pub names: Vec<Name>,
    pub version_groups: Vec<NamedAPIResource<VersionGroup>>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoveTarget {
    pub id: i64,
    pub name: String,
    pub descriptions: Vec<Description>,
    pub moves: Vec<NamedAPIResource<Move>>,
    pub names: Vec<Name>,
}

set_endpoint!(Move, NamedAPIResourceList, "move");
set_endpoint!(MoveAilment, NamedAPIResourceList, "move-ailment");
set_endpoint!(MoveBattleStyle, NamedAPIResourceList, "move-battle-style");
set_endpoint!(MoveCategory, NamedAPIResourceList, "move-category");
set_endpoint!(MoveDamageClass, NamedAPIResourceList, "move-damage-class");
set_endpoint!(MoveLearnMethod, NamedAPIResourceList, "move-learn-method");
set_endpoint!(MoveTarget, NamedAPIResourceList, "move-target");

impl_id_and_named!(Move);
impl_id_and_named!(MoveAilment);
impl_id_and_named!(MoveBattleStyle);
impl_id_and_named!(MoveCategory);
impl_id_and_named!(MoveDamageClass);
impl_id_and_named!(MoveLearnMethod);
impl_id_and_named!(MoveTarget);
