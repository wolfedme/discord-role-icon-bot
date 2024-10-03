use serenity::{
    all::{Context, EventHandler, GuildMemberUpdateEvent, Member, Ready},
    async_trait,
};
use tracing::debug;

use crate::model::configuration::Configuration;

mod guild_member_add;
mod guild_member_change;
mod ready;

pub struct Handler {
    pub config: Configuration,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        debug!("Member added: {:?}", member);
        guild_member_add::add_guest_role_to_new_member(&ctx, &member, &self.config).await;
    }

    async fn guild_member_update(
        &self,
        ctx: Context,
        before: Option<Member>,
        after: Option<Member>,
        event: GuildMemberUpdateEvent,
    ) {
        debug!(
            "Member updated: {:?}\nBefore: {:?}\nAfter: {:?}",
            event, before, after
        );
        guild_member_change::prepend_icon_to_name(&ctx, &self.config, &before, &after, &event)
            .await;
    }

    async fn ready(&self, ctx: Context, event: Ready) {
        ready::on_ready(&ctx, &event).await;
    }
}
