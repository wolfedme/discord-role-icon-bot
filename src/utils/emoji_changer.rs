use emojis::Emoji;
use tracing::{debug, warn};

use crate::model::role::Role;

pub fn add_role_icon(
    guild_member_name: String,
    config_roles: &Vec<Role>,
    desired_role_id: u64,
) -> String {
    debug!(
        "Appending emoji to guild member name: {}",
        guild_member_name
    );

    let contains_existing_role_icon = contains_existing_role_icon(&guild_member_name, config_roles);
    let cleaned_name: String = match contains_existing_role_icon.is_some() {
        true => {
            let val_symbol = contains_existing_role_icon.clone().unwrap().symbol;
            debug!("Removing {:?} from name", val_symbol);
            remove_existing_role_icon(&guild_member_name, val_symbol)
        }
        _ => {
            debug!("No existing role icon detected");
            guild_member_name
        }
    };

    let desired_role_icon = match config_roles.iter().find(|role| role.id == desired_role_id) {
        Some(role) => role.symbol.as_str(),
        None => {
            warn!("No role found with id: {}", desired_role_id);
            emojis::get("❓").unwrap().as_str()
        }
    };

    format!("{} {}", desired_role_icon, cleaned_name).to_string()
}

fn remove_existing_role_icon(guild_member_name: &String, symbol_to_remove: &Emoji) -> String {
    debug!(
        "Removing existing role icon from guild member name: {}",
        guild_member_name
    );

    let new_name = guild_member_name.clone();
    let symbol = symbol_to_remove.as_str();
    let symbol_with_space = symbol.to_string() + " ";
    let new_name = match new_name.starts_with(&symbol_with_space) {
        true => new_name.trim_start_matches(&symbol_with_space),
        false => new_name.trim_start_matches(symbol),
    };
    new_name.to_string()
}

pub fn contains_existing_role_icon(
    guild_member_name: &String,
    config_roles: &Vec<Role>,
) -> Option<Role> {
    debug!(
        "Detecting existing role icon in guild member name: {}",
        guild_member_name
    );

    config_roles
        .iter()
        .find(|role| guild_member_name.starts_with(&role.symbol.as_str()))
        .cloned()
}

#[test]
fn test_append_emoji() {
    let guild_member_name = "🦞 TestUser".to_string();
    let guild_member_name2 = "🦞TestUser".to_string();
    let guild_member_name3 = "TestUser".to_string();

    let config_roles = vec![
        Role {
            id: 1,
            name: "TestRole".to_string(),
            symbol: emojis::get("🦞").unwrap(),
            weight: 1,
        },
        Role {
            id: 2,
            name: "TestRole2".to_string(),
            symbol: emojis::get("🔓").unwrap(),
            weight: 1,
        },
    ];
    let desired_role_id = 2;

    let result_with_space = add_role_icon(guild_member_name, &config_roles, desired_role_id);
    let result_without_space = add_role_icon(guild_member_name2, &config_roles, desired_role_id);
    let result_without_emoji = add_role_icon(guild_member_name3, &config_roles, desired_role_id);
    let result_not_existing = add_role_icon("TestUser".to_string(), &config_roles, 42069);

    let expected = "🔓 TestUser".to_string();
    assert_eq!(result_with_space, expected);
    assert_eq!(result_without_space, expected);
    assert_eq!(result_without_emoji, expected);
    assert_eq!(result_not_existing, "❓ TestUser".to_string());
}
