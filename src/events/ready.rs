use serenity::all::{Context, Ready};
use tracing::{debug, info};

pub async fn on_ready(ctx: &Context, _: &Ready) {
    info!("Bot is ready");
    write_log_into_channel(&ctx).await;
}
// TODO: Error handling
async fn write_log_into_channel(_: &Context) {
    debug!("Writing log into channel");
    // TODO: Implement
}
