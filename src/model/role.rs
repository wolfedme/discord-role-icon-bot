use emojis::Emoji;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    id: String,
    name: String,
    symbol: &'static Emoji,
}
