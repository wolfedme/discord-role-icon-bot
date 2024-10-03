use serenity::all::{Context, GuildMemberUpdateEvent, Member};
use tracing::debug;

use crate::model::configuration::Configuration;
use crate::model::role::Role;
use crate::utils::converter::convert_role_ids_to_u64;

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

    debug!("Roles have changed, updating nickname");

    let event_roles_u64 = convert_role_ids_to_u64(event.roles.clone());
    let matching_roles = get_matching_roles(&event_roles_u64, &config.roles);
    debug!("Matching role ids: {:?}", matching_roles);

    // TODO: Support multiple roles with same weight
    let role_with_highest_weight = matching_roles.iter().max_by_key(|role| role.weight);
    debug!("Role with highest weight: {:?}", role_with_highest_weight);

    // TODO: Get nickname, otherwise username
    // TODO: Check if nickname starts with emoji
    // TODO: If not, prepend emoji of role with highest weight
    // TODO: Else, update emoji to emoji of role with highest weight

    // TODO: If no matching role, remove emoji if there's one

    // TODO: Update nickname
    // TODO: Log changes
    // TODO: Handle errors
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
