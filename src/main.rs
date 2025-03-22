use std::env;

use serenity::{all::{Context, EventHandler, GatewayIntents, Message}, async_trait, Client};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message){
        if msg.content == "!test"{
            if let Err(why) = msg.channel_id.say(&ctx.http, "Hallo!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Get token from env
    dotenv::from_filename(".env.dev").ok();

    let token: String = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Error when creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}