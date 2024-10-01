mod guild_member_addition;
mod guild_member_change;
mod ready;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    data: &BotData,
) -> Result<(), Error> {
    // TODO
}
