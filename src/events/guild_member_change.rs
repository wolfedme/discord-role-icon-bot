use serenity::all::{Context, EditMember, GuildMemberUpdateEvent, Member};
use tracing::{debug, error, info, warn};

use crate::model::configuration::Configuration;
use crate::model::role::Role;
use crate::utils::channel_logger::{log_to_channel, LogType};
use crate::utils::converter::convert_role_ids_to_u64;
use crate::utils::emoji_changer::{add_role_icon, contains_existing_role_icon};

// TODO: Log to channel

pub async fn prepend_icon_to_name(
    ctx: &Context,
    config: &Configuration,
    before: &Option<Member>,
    after: &Option<Member>,
    event: &GuildMemberUpdateEvent,
) {
    if !roles_have_changed(before, after, event) {
        info!("Roles have not changed, no need to update nickname");
        return;
    }

    let member = match ctx.http.get_member(event.guild_id, event.user.id).await {
        Ok(member) => member,
        Err(why) => {
            error!("Could not get member: {:?}", why);
            return;
        }
    };

    debug!("Roles have changed, updating nickname");

    let event_roles_u64 = convert_role_ids_to_u64(event.roles.clone());
    let matching_roles = get_matching_roles(&event_roles_u64, &config.roles);
    debug!("Matching role ids: {:?}", matching_roles);

    // TODO: Support multiple roles with same weight
    let role_with_highest_weight = matching_roles.iter().max_by_key(|role| role.weight);
    let role_to_change = match role_with_highest_weight {
        Some(role) => role,
        None => {
            warn!("Role not included in configuration - skipping.");
            return;
        }
    };
    debug!("Role with highest weight: {:?}", role_with_highest_weight);
    let username = member.display_name();
    debug!("Username: {}", username);

    match contains_existing_role_icon(&username.to_string(), &config.roles) {
        Some(role) => {
            if role.id == role_to_change.id {
                info!("Role icon already present in username, skipping");
                return;
            }
        }
        _ => {}
    }

    let mut new_username = add_role_icon(username.to_string(), &config.roles, role_to_change.id);

    match new_username.len() > 32 {
        true => {
            warn!("New nickname is longer than 32 characters, truncating");
            new_username = new_username[..32].to_string();
        }
        false => debug!("New nickname is shorter than 32 characters"),
    }

    match member
        .guild_id
        .edit_member(
            &ctx.http,
            member.user.id,
            EditMember::default().nickname(new_username),
        )
        .await
    {
        // TODO: Include role name in log message, mention member. Rich message -> before and after
        Ok(_) => {
            let msg = format!(
                "Updated nickname for <@{}>\n\nOld nickname: {}\nBasedOnRole: <@&{}>",
                member.user.id, username, role_to_change.id
            );
            info!(msg);
            log_to_channel(
                &ctx.http,
                &config.features.log_to_channel,
                &msg,
                LogType::Info,
            )
            .await
        }
        Err(e) => {
            let msg = format!(
                "Could not update nickname for <@{}>: {:?}",
                member.user.id, e
            );
            error!(msg);
            log_to_channel(
                &ctx.http,
                &config.features.log_to_channel,
                &msg,
                LogType::Error,
            )
            .await
        }
    }

    // TODO: Tests
    // TODO: Update on own namechange -> not important, nice to have
}

fn roles_have_changed(
    before: &Option<Member>,
    after: &Option<Member>,
    event: &GuildMemberUpdateEvent,
) -> bool {
    match (before, after) {
        (Some(before), Some(after)) => {
            debug!("Cache available. Comparing before and after");
            let before_roles = &before.roles;
            let after_roles = &after.roles;
            let event_roles = &event.roles;
            before_roles != after_roles || before_roles != event_roles
        }
        _ => {
            debug!(
                "Cannot determine if roles have changed, no cache available for user {:?}",
                event.user.id
            );
            if event.roles.is_empty() {
                info!("User has no roles");
                false
            } else {
                true
            }
        }
    }
}

fn get_matching_roles(roles_from_event: &Vec<u64>, roles_from_config: &Vec<Role>) -> Vec<Role> {
    roles_from_config
        .iter()
        .filter(|role| roles_from_event.contains(&role.id))
        .cloned()
        .collect()
}

// Required for future feature
fn _before_after_are_same(before: &Option<Member>, after: &Option<Member>) -> bool {
    match (before, after) {
        (Some(before), Some(after)) => {
            let before_nick = before.nick.as_ref().unwrap_or(&before.user.name);
            let after_nick = after.nick.as_ref().unwrap_or(&after.user.name);
            before_nick == after_nick
        }
        _ => false,
    }
}
