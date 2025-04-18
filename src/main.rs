use std::error::Error;

use serenity::{all::GatewayIntents, Client};

// mod commands;
mod events;
mod settings;
// mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bot_token: String = settings::token::get_token();
    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client: Client = Client::builder(&bot_token, intents)
        .event_handler(events::Handler)
        .await
        .expect("Error when creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    Ok(())
}
