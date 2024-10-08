use serenity::{all::Context, model::guild::Member};
use tracing::{debug, error, info};

use crate::{
    model::configuration::Configuration,
    utils::channel_logger::{log_to_channel, LogType},
};

pub async fn add_guest_role_to_new_member(ctx: &Context, member: &Member, config: &Configuration) {
    info!("Adding guest role to new member: {}", member.user.name);
    add_guest_role(&ctx, &member, &config).await;
}

async fn add_guest_role(ctx: &Context, member: &Member, config: &Configuration) {
    if !config.features.assign_guest_role_on_join.enabled {
        debug!("assign_guest_role_on_member_join.enabled == false");
        return;
    }
    match member
        .add_role(&ctx.http, config.features.assign_guest_role_on_join.role_id)
        .await
    {
        Ok(_) => {
            let msg = format!("Added guest role to {}", member.user.name);
            info!(msg);
            log_to_channel(
                &ctx.http,
                &config.features.log_to_channel,
                msg.as_str(),
                LogType::Info,
            )
            .await
        }
        Err(why) => {
            let msg = format!(
                "Could not add guest role to {}: {:?}",
                member.user.name, why
            );
            error!(msg);
            log_to_channel(
                &ctx.http,
                &config.features.log_to_channel,
                msg.as_str(),
                LogType::Error,
            )
            .await
        }
    };
}
