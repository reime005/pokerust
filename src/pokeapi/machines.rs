use serde::{Deserialize, Serialize};

use super::utility::*;

use crate::{impl_id, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Machine {
    pub id: i64,
    pub item: NamedAPIResource,
    #[serde(rename = "move")]
    pub move_: NamedAPIResource,
    pub version_group: NamedAPIResource,
}

set_endpoint!(Machine, "machine");

impl_id!(Machine);
