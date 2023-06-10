use std::env;

use hearth_interconnect::errors::ErrorReport;
use hearth_interconnect::messages::Metadata;
use log::error;
use std::time::Duration;

// Import the `Context` to handle commands.
use charcoal_client::{
    actions::{
        channel_manager::{exit_channel, join_channel},
        player::{play_from_http, play_from_youtube},
        standard::register_event_handler,
        track_manager::{
            force_stop_loop, get_metadata, loop_indefinitely, loop_x_times, pause_playback,
            resume_playback, seek_to_position, set_playback_volume,
        },
    },
    serenity::{CharcoalKey, SerenityInit},
};
use serenity::client::Context;

use charcoal_client::{
    get_handler_from_serenity, get_handler_from_serenity_mutable, CharcoalConfig, PlayerObjectData,
    SASLConfig,
};

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

use charcoal_client::actions::standard::CharcoalEventHandler;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(
    join,
    leave,
    play,
    ping,
    metadata,
    loopforever,
    pause,
    resume,
    youtube,
    volume,
    stoploop,
    looptimes,
    position
)]
struct General;

struct CustomEventHandler {}

impl CharcoalEventHandler for CustomEventHandler {
    fn handle_error(&self, error_report: ErrorReport) {
        println!("Uh oh got error in event handler: {:?}", error_report);
    }

    fn handle_metadata_response(&self, metadata: Metadata) {
        println!("Got metadata back in event handler: {:?}", metadata);
    }
}

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
                sasl: Some(SASLConfig {
                    kafka_username: env::var("KAFKA_USER")
                        .expect("Expected KAFKA_BROKER env variable"),
                    kafka_password: env::var("KAFKA_PASS")
                        .expect("Expected KAFKA_BROKER env variable"),
                }),
                ssl: None,
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
    let mut handler: Option<&PlayerObjectData> = None;
    get_handler_from_serenity!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            pause_playback(handler).await.unwrap();
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
    let mut handler: Option<&PlayerObjectData> = None;
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
            resume_playback(handler).await.unwrap();
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
    println!("Joining");
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
        println!("Using pre-existing player");
        // Get a write lock on the players HashMap
        let mut players = mx.players.write().await;
        // Get a mutable reference to said player
        let handler = players.get_mut(&guild_id.to_string()).expect(
            "This should never happen because we checked the key exists in the if check above",
        );
        // Join the channel
        join_channel(handler, connect_to.to_string(), false)
            .await
            .unwrap(); // We use false here so Charcoal does not create a pre-existing job
    } else {
        println!("Creating new player");
        // If we have not created the player create it and then join the channel
        let handler = PlayerObjectData::new(
            guild_id.to_string(),
            mx.to_bg_tx.clone(),
            mx.runtime.clone(),
        )
        .await;
        println!("Created new handler");
        // Make sure creating the PlayerObject worked
        match handler {
            Ok(mut handler) => {
                // Register an error callback so errors from the hearth server can be reported back to us
                register_event_handler(&handler, CustomEventHandler {}).await;
                // Join the channel
                println!("Registered error callback");
                join_channel(&handler, connect_to.to_string(), true)
                    .await
                    .unwrap(); // We use true here to tell Charcoal to create the Job
                println!("Joined channel");
                // Insert the newly created PlayerObject into the HashMap so we can use it later
                mx.players
                    .write()
                    .await
                    .insert(guild_id.to_string(), handler);
                println!("Inserted new player");
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
    let mut handler: Option<&mut PlayerObjectData> = None;
    // This get's a mutable PlayerObject instead of a constant one
    // Be careful where you use this as getting the playerobject as mutable locks the internal RwLock Mutex
    get_handler_from_serenity_mutable!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            get_metadata(handler).await.unwrap();
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
    let mut handler: Option<&PlayerObjectData> = None;
    get_handler_from_serenity!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            let _meta = loop_indefinitely(handler).await;
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
    let mut handler: Option<&PlayerObjectData> = None;
    get_handler_from_serenity!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            exit_channel(handler).await;
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
    let mut handler: Option<&mut PlayerObjectData> = None;
    get_handler_from_serenity_mutable!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            play_from_http(handler, url).await.unwrap();
            check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);
        }
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn youtube(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
    let mut handler: Option<&mut PlayerObjectData> = None;
    get_handler_from_serenity_mutable!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            play_from_youtube(handler, url).await.unwrap();
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Playing song from YouTube")
                    .await,
            );
        }
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let volume = match args.single::<f32>() {
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

    // Make sure that volume is between 0 and 1. As for performance reasons the Hearth server does not have soft-clipping enabled
    // So any values above 1 may clip
    if volume >= 0.0 && volume <= 1.0 {
        // Get the PlayerObject using a helper macro
        let mut handler: Option<&PlayerObjectData> = None;
        get_handler_from_serenity!(ctx, msg, handler);

        match handler {
            Some(handler) => {
                set_playback_volume(handler, volume).await.unwrap();
                check_msg(msg.channel_id.say(&ctx.http, "Set volume").await);
            }
            None => {
                error!("Failed to get manager!");
            }
        }
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Volume must be between zero and one")
                .await,
        );
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn stoploop(ctx: &Context, msg: &Message) -> CommandResult {
    // Get the PlayerObject using a helper macro
    let mut handler: Option<&PlayerObjectData> = None;
    get_handler_from_serenity!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            force_stop_loop(handler).await.unwrap();
            check_msg(msg.channel_id.say(&ctx.http, "Canceled Loop").await);
        }
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn looptimes(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let times = match args.single::<usize>() {
        Ok(times) => times,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide loop times")
                    .await,
            );

            return Ok(());
        }
    };

    // Get the PlayerObject using a helper macro
    let mut handler: Option<&PlayerObjectData> = None;
    get_handler_from_serenity!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            loop_x_times(handler, times).await.unwrap();
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Looping {} time(s)", times))
                    .await,
            );
        }
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn position(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let position = match args.single::<u64>() {
        Ok(position) => position,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide song position")
                    .await,
            );

            return Ok(());
        }
    };

    // Get the PlayerObject using a helper macro
    let mut handler: Option<&PlayerObjectData> = None;
    get_handler_from_serenity!(ctx, msg, handler);

    match handler {
        Some(handler) => {
            seek_to_position(handler, Duration::from_secs(position))
                .await
                .unwrap();
            check_msg(msg.channel_id.say(&ctx.http, "Seeking...").await);
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
