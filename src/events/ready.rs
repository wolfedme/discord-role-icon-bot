use serenity::all::{Context, Ready};
use tracing::info;

pub async fn on_ready(ctx: &Context, _: &Ready) {
    info!("Bot is ready");
    info!("Cached guilds: {}", ctx.cache.guild_count());
    info!("Cached users: {}", ctx.cache.user_count());
}
