use std::env;

use log::error;

// Import the `Context` to handle commands.
use charcoal_client::serenity::{CharcoalKey, SerenityInit};
use serenity::client::Context;

// IMPORTANT NOTE:
// This example uses unwrap()s on the Results<> from charcoal
// In practice you should handle these error's properly
// unwrap()s are used here for simplicity.

use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::GatewayIntents,
    Result as SerenityResult,
};

use charcoal_client::actions::channel_manager::ChannelManager;
use charcoal_client::actions::player::Player;
use charcoal_client::actions::track_manager::TrackManager;
use charcoal_client::{
    get_handler_from_serenity, get_handler_from_serenity_mutable, CharcoalConfig, PlayerObject,
    SSLConfig,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(join, leave, play, ping, metadata, loopforever, pause, resume)]
struct General;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        // Add a Kafka URL here to connect to the broker
        .register_charcoal(
            env::var("KAFKA_BROKER").expect("Expected KAFKA_BROKER env variable"),
            CharcoalConfig {
                ssl: Some(SSLConfig {
                    ssl_ca: "ca.pem".to_string(),
                    ssl_cert: "service.cert".to_string(),
                    ssl_key: "service.key".to_string(),
                }),
                kafka_topic: "communication".to_string(),
            },
        )
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}

#[command]
#[only_in(guilds)]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    // Get the PlayerObject using a helper macro
    let mut handler: Option<&PlayerObject> = None;
    get_handler_from_serenity!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            handler.pause_playback().await.unwrap();
        }
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    // Get the PlayerObject using a helper macro
    let mut handler: Option<&PlayerObject> = None;
    get_handler_from_serenity!(ctx, msg, handler);

    // If you don't want to use the macro you can also get the PlayerObject like this
    // As it is pretty much equivalent to the above macro:

    // // Get the serenity typemap
    // let r = ctx.data.read().await;
    // // Get the GuildID
    // let guild = msg.guild(&ctx.cache).unwrap();
    // let guild_id = guild.id;
    // // Get the charcoal manager from the serenity typemap
    // let manager = r.get::<CharcoalKey>();
    // let mut mx = manager.unwrap().lock().await;
    // // Get the PlayerObject
    // let mut players = mx.players.write().await;
    // let handler =  players.get_mut(&guild_id.to_string());

    match handler {
        Some(handler) => {
            handler.resume_playback().await.unwrap();
        }
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    // Get the manager from the serenity typemap
    let r = ctx.data.write().await;
    let manager = r.get::<CharcoalKey>();
    let mx = manager.unwrap().lock().await;

    // Check if we have already created the player by checking if the player's GuildID exists in the Players HashMap
    // Stored inside of the Charcoal Instance.
    // If we have already created the player just join the channel
    if mx.players.read().await.contains_key(&guild_id.to_string()) {
        // Get a write lock on the players HashMap
        let mut players = mx.players.write().await;
        // Get a mutable reference to said player
        let handler = players.get_mut(&guild_id.to_string()).expect(
            "This should never happen because we checked the key exists in the if check above",
        );
        // Join the channel
        handler.join_channel(connect_to.to_string()).await;
    } else {
        // If we have not created the player create it and then join the channel
        let handler = PlayerObject::new(guild_id.to_string(), mx.tx.clone()).await;
        // Make sure creating the PlayerObject worked
        match handler {
            Ok(mut handler) => {
                // Join the channel
                handler.join_channel(connect_to.to_string()).await;
                // Insert the newly created PlayerObject into the HashMap so we can use it later
                mx.players
                    .write()
                    .await
                    .insert(guild_id.to_string(), handler);
            }
            Err(e) => {
                // If creating the job failed send an error message
                check_msg(
                    msg.channel_id
                        .say(
                            &ctx.http,
                            format!("Failed to register PlayerObject with error: {}", e),
                        )
                        .await,
                );
            }
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn metadata(ctx: &Context, msg: &Message) -> CommandResult {
    // Get the PlayerObject using a helper macro
    let mut handler: Option<&mut PlayerObject> = None;
    // This get's a mutable PlayerObject instead of a constant one
    // Be careful where you use this as getting the playerobject as mutable locks the internal RwLock Mutex
    get_handler_from_serenity_mutable!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            let meta = handler.get_metadata().await.unwrap();
            println!("{:?}", meta);
        }
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn loopforever(ctx: &Context, msg: &Message) -> CommandResult {
    // Get the PlayerObject using a helper macro
    let mut handler: Option<&PlayerObject> = None;
    get_handler_from_serenity!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            let _meta = handler.loop_indefinitely().await;
            check_msg(msg.channel_id.say(&ctx.http, "Looping forever!").await);
        }
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    // Get the PlayerObject using a helper macro
    let mut handler: Option<&PlayerObject> = None;
    get_handler_from_serenity!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            handler.exit_channel().await;
        }
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

#[command]
async fn ping(context: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&context.http, "Pong!").await);

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a URL to a video or audio")
                    .await,
            );

            return Ok(());
        }
    };

    if !url.starts_with("http") {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Must provide a valid URL")
                .await,
        );

        return Ok(());
    }

    // Get the PlayerObject using a helper macro
    let mut handler: Option<&mut PlayerObject> = None;
    get_handler_from_serenity_mutable!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            handler.play_from_http(url).await.unwrap();
            check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);
        }
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
