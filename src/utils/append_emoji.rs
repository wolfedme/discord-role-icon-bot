use tracing::{info, warn};

pub fn append_emoji(guild_member_name: String) -> String {
    info!(
        "Appending emoji to guild member name: {}",
        guild_member_name
    );
    warn!("TODO: Implement!");

    guild_member_name
}

pub fn contains_existing_role_icon(guild_member_name: String) -> bool {
    info!(
        "Detecting existing role icon in guild member name: {}",
        guild_member_name
    );
    warn!("TODO: Implement!");

    false
}
