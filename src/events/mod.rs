use serenity::{
    all::{Context, EventHandler, Message},
    async_trait,
};

pub mod message;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        if let Err(e) = message::handle(&ctx, &new_message).await {
            println!("Error when handling message: {}", e);
        }
    }
}

pub struct Handler;
