use serenity::all::{Context, EventHandler, Guild, GuildId, GuildMemberUpdateEvent, Member, Ready};
use tracing::{debug, info};

use crate::model::configuration::Configuration;

mod guild_member_add;
mod guild_member_change;
mod ready;

pub struct Handler {
    pub config: Configuration,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        debug!("Member added: {:?}", member);
        guild_member_add::add_guest_role_to_new_member(&ctx, &member, &self.config).await;
    }

    async fn guild_create(&self, _ctx: Context, guild: Guild, _is_new: Option<bool>) {
        debug!("Guild created/joined: {:?}", guild);
    }

    async fn guild_member_update(
        &self,
        ctx: Context,
        old_if_available: Option<Member>,
        new: Option<Member>,
        event: GuildMemberUpdateEvent,
    ) {
        ctx.cache.update(&mut event.clone());
        print_debug_info(&ctx, &old_if_available, &new, &event).await;

        guild_member_change::prepend_icon_to_name(
            &ctx,
            &self.config,
            &old_if_available,
            &new,
            &event,
        )
        .await;
    }

    async fn ready(&self, ctx: Context, event: Ready) {
        ready::on_ready(&ctx, &event).await;
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache ready");
        println!("{} unknown members", ctx.cache.unknown_members());
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

    info!(
        "
            Member_changed

            Before: {before:?}
            After: {after:?}
            Roles_Event: {event_roles:?}
            Member_Event_Nick: {event_nick:?}

            Members in cache: {members_cache:?}
            Guilds in cache: {guilds_cache:?}
        ",
        before = format!("{:?}", before),
        after = format!("{:?}", after),
        event_roles = roles_as_string,
        event_nick = nick,
        members_cache = ctx.cache.user_count(),
        guilds_cache = ctx.cache.guild_count(),
    );
}
