use std::env;

use serenity::prelude::*;

mod commands;
mod events;
mod logger;

#[tokio::main]
async fn main() {
    logger::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected API token in the env variables");
    let intents = GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(token, intents)
        .await
        .expect("Error creating client");

    // start
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
