use serde::{Deserialize, Serialize};

use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncounterMethod {
    pub id: i64,
    pub name: String,
    pub order: u64,
    pub names: Vec<Name>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncounterCondition {
    pub id: i64,
    pub name: String,
    pub names: Vec<Name>,
    pub values: Vec<NamedAPIResource>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EncounterConditionValue {
    pub id: i64,
    pub name: String,
    pub condition: NamedAPIResource, // incorrectly documented as list NamedAPIResource
    pub names: Vec<Name>,
}

set_endpoint!(EncounterMethod, "encounter-method");
set_endpoint!(EncounterCondition, "encounter-condition");
set_endpoint!(EncounterConditionValue, "encounter-condition-value");

impl_id_and_named!(EncounterMethod);
impl_id_and_named!(EncounterCondition);
impl_id_and_named!(EncounterConditionValue);
