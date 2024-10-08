use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Features {
    pub log_to_channel: LogToChannel,
    pub assign_guest_role_on_join: AssignGuestRoleOnJoin,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LogToChannel {
    pub enabled: bool,
    pub channel_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AssignGuestRoleOnJoin {
    pub enabled: bool,
    pub role_id: u64,
}

impl Default for Features {
    fn default() -> Self {
        Features {
            log_to_channel: LogToChannel {
                enabled: false,
                channel_id: 0,
            },
            assign_guest_role_on_join: AssignGuestRoleOnJoin {
                enabled: false,
                role_id: 0,
            },
        }
    }
}
