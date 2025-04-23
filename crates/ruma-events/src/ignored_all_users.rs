use serde::{Deserialize, Serialize};
use ruma_macros::EventContent;

#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.ignored_all_users", kind = GlobalAccountData)]
pub struct IgnoredAllUsersEventContent {
    pub enabled: bool,
}

impl IgnoredAllUsersEventContent {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_matches2::assert_matches;
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

    use super::IgnoredAllUsersEventContent;
    use crate::AnyGlobalAccountDataEvent;

    #[test]
    fn serialization() {
        let ignored_all_users_event_content = IgnoredAllUsersEventContent::new(true);

        let json = json!({
            "enabled": true,
        });

        assert_eq!(to_json_value(ignored_all_users_event_content).unwrap(), json);
    }

    #[test]
    fn deserialization() {
        let json = json!({
            "content": {
               "enabled": true
            },
            "type": "m.ignored_all_users"
        });

        assert_matches!(
            from_json_value::<AnyGlobalAccountDataEvent>(json),
            Ok(AnyGlobalAccountDataEvent::IgnoredAllUsers(ev))
        );
        assert_eq!(
            ev.content.enabled,
            true
        );
    }
}