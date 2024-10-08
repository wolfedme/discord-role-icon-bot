use std::{env, time::Duration};

use events::Handler;
use model::configuration::Configuration;
use serenity::{all::Settings, prelude::*};
use utils::channel_logger::log_to_channel;

mod commands;
mod events;
mod logger;
mod model;
mod utils;

#[tokio::main]
async fn main() {
    logger::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected API token in the env variables");
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let config = Configuration::load().expect("Could not load configuration");
    let log_feature = &config.features.log_to_channel.clone();

    let mut settings = Settings::default();
    settings.max_messages = 10;
    settings.time_to_live = Duration::from_secs(60);

    let mut client = Client::builder(token, intents)
        .event_handler(Handler { config })
        .cache_settings(settings)
        .await
        .expect("Error creating client");

    // start
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
        log_to_channel(
            &client.http,
            log_feature,
            why.to_string().as_str(),
            utils::channel_logger::LogType::Error,
        )
        .await;
    }
}
