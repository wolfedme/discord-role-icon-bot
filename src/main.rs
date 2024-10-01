use model::configuration::Configuration;
use serenity::prelude::*;
use tracing::info;

mod commands;
mod events;
mod logger;
mod model;

#[tokio::main]
async fn main() {
    logger::init();
    // let token = env::var("DISCORD_TOKEN").expect("Expected API token in the env variables");
    let token = "remove me";
    let intents = GatewayIntents::GUILD_MEMBERS;
    let config = Configuration::load().expect("Could not load configuration");

    let mut client = Client::builder(token, intents)
        .await
        .expect("Error creating client");

    info!("Loaded configuration: {:?}", config);

    // start
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
