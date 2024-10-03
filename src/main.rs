use std::env;

use events::Handler;
use model::configuration::Configuration;
use serenity::prelude::*;

mod commands;
mod events;
mod logger;
mod model;
mod utils;

#[tokio::main]
async fn main() {
    logger::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected API token in the env variables");
    let intents = GatewayIntents::GUILD_MEMBERS;
    let config = Configuration::load().expect("Could not load configuration");

    let mut client = Client::builder(token, intents)
        .event_handler(Handler { config })
        .await
        .expect("Error creating client");

    // start
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
