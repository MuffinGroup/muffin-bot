use std::env;

use serenity::{
    async_trait,
    http::Http,
    model::{channel::Message, gateway::Ready, prelude::{GuildChannel, ChannelType, RoleId}},
    prelude::*,
};

const HELP_MESSAGE: &str = "You are ugly";

const HELP_COMMAND: &str = "!thetruth";

const MUFFIN_MSG: &str = "Muffins are nice";

const MUFFIN_CMD: &str = "!muffin";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == MUFFIN_CMD {
            if let Err(why) = msg.channel_id.say(&ctx.http, MUFFIN_MSG).await {
                println!("Error sending message: {:?}", why);
            }
        }

        let role = RoleId(1130098693194928228);

        let guild_id = match msg.guild_id {
            Some(guild_id) => guild_id,
            None => return, // The message was not sent in a guild, so we can't proceed
        };


        if msg.content == "!create_channel" && msg.author.has_role(&ctx.http, guild_id, role).await.unwrap() {
            // Get the bot token from the environment
            let token =
                env::var("DISCORD_TOKEN").expect("Expected a bot token in the environment.");

            // Create an HTTP client
            let http = Http::new_with_token(&token);

            // Get the guild ID where the message was sent (assuming it's in a guild)
            let guild_id = match msg.guild_id {
                Some(guild_id) => guild_id,
                None => return, // The message was not sent in a guild, so we can't proceed
            };

            // Create a new text channel with a name
            if let Ok(new_channel) = guild_id
                .create_channel(&http, |c| {
                    c.name("new-channel").kind(ChannelType::Text)
                })
                .await
            {
                println!("New channel created: {}", new_channel.name);
            } else {
                println!("Failed to create a new channel.");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn channel_create(&self, _ctx: Context, _channel: &GuildChannel) {
        let name = &_channel.name;
        println!("New channel created: {}", name);

        // Get the bot token from the environment
        let token = env::var("DISCORD_TOKEN").expect("Expected a bot token in the environment.");

        // Create an HTTP client
        let http = Http::new_with_token(&token);

        // Send a message to the new channel
        let _ = _channel
            .id
            .send_message(&http, |m| m.content("Oh a new channel! How exciting!"))
            .await;
    }
}

#[tokio::main]
async fn main() {
    // Before executing this, set a new system variable called
    // DISCORD_TOKEN with the bot token as the value
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
