use serenity::{all::Context, model::guild::Member};
use tracing::{error, info};

use crate::model::configuration::Configuration;

// TODO: Feature toggle

pub async fn add_guest_role_to_new_member(ctx: &Context, member: &Member, config: &Configuration) {
    info!("Adding guest role to new member: {}", member.user.name);
    add_guest_role(&ctx, &member, &config).await;
}

async fn add_guest_role(ctx: &Context, member: &Member, config: &Configuration) {
    match member.add_role(&ctx.http, config.guest_role_id).await {
        Ok(_) => info!("Added guest role to {}", member.user.name),
        Err(why) => error!(
            "Could not add guest role to {}: {:?}",
            member.user.name, why
        ),
    };
}
