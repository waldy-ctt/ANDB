use serenity::all::{Context, Message};

pub async fn handle(ctx: &Context, msg: &Message) -> Result<(), serenity::Error> {
    if msg.author.bot {
        return Ok(());
    }

    if !msg.content.starts_with("|") {
        return Ok(());
    }

    println!("Message received: {}", msg.content);

    Ok(())
}
