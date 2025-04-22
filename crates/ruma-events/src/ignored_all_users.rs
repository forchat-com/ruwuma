use serde::{Deserialize, Serialize};
use ruma_macros::EventContent;

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.ignored_all_users", kind = GlobalAccountData)]
pub struct IgnoredAllUsersEventContent {
    pub enabled: String,
}

impl IgnoredAllUsersEventContent {
    pub fn new(enabled: bool) -> Self {
        if enabled {
            return Self {
                enabled: "enable".to_string(),
            }
        }
        Self {
            enabled: "disable".to_string(),
        }
    }
}
