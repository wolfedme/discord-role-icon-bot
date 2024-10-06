use serenity::all::{Context, EditMember, GuildMemberUpdateEvent, Member};
use tracing::{debug, error, info, warn};

use crate::model::configuration::Configuration;
use crate::model::role::Role;
use crate::utils::converter::convert_role_ids_to_u64;
use crate::utils::emoji_changer::add_role_icon;

pub async fn prepend_icon_to_name(
    ctx: &Context,
    config: &Configuration,
    before: &Option<Member>,
    after: &Option<Member>,
    event: &GuildMemberUpdateEvent,
) {
    print_debug_info(ctx, before, after, event).await;
    if !roles_have_changed(before, after, event) {
        debug!("Roles have not changed, no need to update nickname");
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
            warn!("No matching role found");
            &Role::default()
        }
    };
    debug!("Role with highest weight: {:?}", role_with_highest_weight);
    let username = member.display_name();
    debug!("Username: {}", username);

    let new_username = add_role_icon(username.to_string(), &config.roles, role_to_change.id);

    match member
        .guild_id
        .edit_member(
            &ctx.http,
            member.user.id,
            EditMember::default().nickname(new_username),
        )
        .await
    {
        Ok(_) => info!("Updated nickname for {}", member.user.name),
        Err(e) => error!(
            "Could not update nickname for {}: {:?}",
            member.user.name, e
        ),
    }

    // TODO: Tests

    // TODO: Before & after nickname same -> do nothing
    // TODO: If longer than 32 characters -> truncate
    // TODO: Nickname empty, use username
    // TODO: Nickname not empty, prepend icon
    // TODO: Check for existing icon and remove
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

async fn print_debug_info(
    ctx: &Context,
    before: &Option<Member>,
    after: &Option<Member>,
    event: &GuildMemberUpdateEvent,
) {
    let roles_as_string: String = event
        .roles
        .iter()
        .map(|role_id| role_id.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let nick = event
        .user
        .nick_in(&ctx.http, event.guild_id)
        .await
        .unwrap_or_else(|| "None".to_string());

    debug!(
        "
            Member_changed

            Before: {before:?}
            After: {after:?}
            Roles_Event: {event_roles:?}
            Member_Event_Nick: {event_nick:?}
        ",
        before = format!("{:?}", before),
        after = format!("{:?}", after),
        event_roles = roles_as_string,
        event_nick = nick
    );
}
