use emojis::Emoji;
use serde::{Deserialize, Serialize};

// TODO: Update config.example json and readme
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Role {
    pub id: u64,
    pub name: String,
    pub symbol: &'static Emoji,
    pub weight: u8,
}

impl Role {
    pub fn default() -> Role {
        Role {
            id: 0,
            name: String::from(""),
            symbol: emojis::get("❓").unwrap(),
            weight: 0,
        }
    }
}
