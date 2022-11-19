use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::GuildId;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    // send a message when a member joins the server
    async fn guild_member_addition(&self, ctx: Context, guild_id: GuildId, new_member: Member) {
        let guild = guild_id.to_guild_cached(&ctx.cache).await.unwrap();
        let channel_id = guild
            .channels
            .values()
            .find(|c| c.name == "🌊・join")
            .unwrap()
            .id;

        if let Err(why) = channel_id.say(&ctx.http, format!("Welcome to the server, {}!", new_member.display_name())).await {
            println!("Error sending message: {:?}", why);
        }

        // send a privat message to the new member
        if let Err(why) = new_member.user.direct_message(&ctx, |m| {
            m.content("Welcome to the server! \n Chat in <#990521519283273730> and send your memes etc. in <#1043471832365416518>");
        }).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}