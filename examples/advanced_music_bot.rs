use std::env;
use std::sync::Arc;
use std::time::Duration;
use hearth_interconnect::errors::ErrorReport;
use log::error;


// Import the `Context` to handle commands.
use serenity::client::Context;
use charcoal::serenity::{CharcoalKey, SerenityInit};
use charcoal::background::processor::IPCData;


use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::{
        StandardFramework,
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
    model::{channel::Message, gateway::Ready},
    prelude::GatewayIntents,
    Result as SerenityResult,
};
use serenity::http::Http;
use serenity::model::id::ChannelId;
use tokio::time::sleep;


use charcoal::actions::channel_manager::ChannelManager;
use charcoal::actions::player::Player;
use charcoal::{CharcoalConfig, PlayerObject, SSLConfig};
use charcoal::actions::track_manager::TrackManager;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(join, leave, play, ping,metadata,loopforever,pause,resume,youtube,volume,stoploop,looptimes,position)]
struct General;

async fn report_error(error_report: ErrorReport,http: Arc<Http>,channel_id: String) {
    let voice_channel_id = ChannelId::from(channel_id.parse::<u64>().unwrap());
    let msg = voice_channel_id.say(http, format!("Action failed with error: {:?}",error_report.error)).await;
    check_msg(msg);
}

#[tokio::main]
async fn main() {

    env_logger::init();


    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix("~"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        // Add a Kafka URL here to connect to the broker
        .register_charcoal("kafka-185690f4-maxall4-aea3.aivencloud.com:23552".to_string(),CharcoalConfig {
            ssl: Some(SSLConfig {
                ssl_ca: "ca.pem".to_string(),
                ssl_cert: "service.cert".to_string(),
                ssl_key: "service.key".to_string()
            }),
            kafka_topic: "communication".to_string()
        })
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}

#[command]
#[only_in(guilds)]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let r = ctx.data.read().await;

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let mut manager = r.get::<CharcoalKey>().unwrap().lock().await;
    let manager = manager.get_player(&guild_id.to_string());

    match manager {
        Some(manager) => {
            manager.pause_playback().await;
        },
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let r = ctx.data.read().await;

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let mut manager = r.get::<CharcoalKey>().unwrap().lock().await;
    let manager = manager.get_player(&guild_id.to_string());

    match manager {
        Some(manager) => {
            manager.resume_playback().await;
        },
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
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };
    let r = ctx.data.write().await;
    let mut manager = r.get::<CharcoalKey>().unwrap().lock().await;

    // If we have already created the player just join the channel
    println!("Joining");
    if manager.players.contains_key(&guild_id.to_string()) {
        let handler =  manager.players.get_mut(&guild_id.to_string()).unwrap();
        handler.join_channel(connect_to.to_string()).await;
        handler.register_error_callback(report_error,ctx.http.clone(),msg.channel_id.to_string()).await;
    } else {
        // If we have not created the player create it and then join the channel
        let mut handler = PlayerObject::new(guild_id.to_string(),manager.tx.clone()).await;
        handler.register_error_callback(report_error,ctx.http.clone(),msg.channel_id.to_string()).await;
        handler.create_job().await;
        // sleep(Duration::from_secs(1)).await;
        handler.join_channel(connect_to.to_string()).await;
        manager.players.insert(guild_id.to_string(), handler);

    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn metadata(ctx: &Context, msg: &Message) -> CommandResult {
    let r = ctx.data.read().await;

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let mut manager = r.get::<CharcoalKey>().unwrap().lock().await;
    let manager = manager.get_player(&guild_id.to_string());

    match manager {
        Some(manager) => {
            let meta = manager.get_metadata().await;
            println!("{:?}",meta);
        },
        None => {
            error!("Failed to get manager!");
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn loopforever(ctx: &Context, msg: &Message) -> CommandResult {
    let r = ctx.data.read().await;

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let mut manager = r.get::<CharcoalKey>().unwrap().lock().await;
    let manager = manager.get_player(&guild_id.to_string());
    match manager {
        Some(manager) => {
            let meta = manager.loop_indefinitely().await;
            check_msg(msg.channel_id.say(&ctx.http, "Looping forever!").await);
        },
        None => {
            error!("Failed to get manager!");
        }
    }


    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let r = ctx.data.read().await;

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let mut manager = r.get::<CharcoalKey>().unwrap().lock().await;
    let manager = manager.get_player(&guild_id.to_string());

    match manager {
        Some(manager) => {
            manager.exit_channel().await;
        },
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
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio").await);

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "Must provide a valid URL").await);

        return Ok(());
    }

    let r = ctx.data.read().await;
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = r.get::<CharcoalKey>();
    let mut mx = manager.unwrap().lock().await;
    let handler =  mx.players.get_mut(&guild_id.to_string()).unwrap();


    handler.play_from_http(url).await;
    check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn youtube(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio").await);

            return Ok(());
        },
    };

    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "Must provide a valid URL").await);

        return Ok(());
    }

    let r = ctx.data.read().await;
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = r.get::<CharcoalKey>();
    let mut mx = manager.unwrap().lock().await;
    let handler =  mx.players.get_mut(&guild_id.to_string()).unwrap();


    handler.play_from_youtube(url).await;
    check_msg(msg.channel_id.say(&ctx.http, "Playing song from YouTube").await);

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let volume = match args.single::<f32>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio").await);

            return Ok(());
        },
    };

    if volume >= 0.0 || volume <= 1.0 {
        let r = ctx.data.read().await;
        let guild = msg.guild(&ctx.cache).unwrap();
        let guild_id = guild.id;
        let manager = r.get::<CharcoalKey>();
        let mut mx = manager.unwrap().lock().await;
        let handler =  mx.players.get_mut(&guild_id.to_string()).unwrap();

        handler.set_playback_volume(volume).await;

        check_msg(msg.channel_id.say(&ctx.http, "Set volume").await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Volume must be between zero and one").await);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn stoploop(ctx: &Context, msg: &Message) -> CommandResult {

    let r = ctx.data.read().await;
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = r.get::<CharcoalKey>();
    let mut mx = manager.unwrap().lock().await;
    let handler =  mx.players.get_mut(&guild_id.to_string()).unwrap();


    handler.force_stop_loop().await;

    check_msg(msg.channel_id.say(&ctx.http, "Canceled Loop").await);

    Ok(())
}


#[command]
#[only_in(guilds)]
async fn looptimes(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let times = match args.single::<usize>() {
        Ok(times) => times,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide loop times").await);

            return Ok(());
        },
    };

    let r = ctx.data.read().await;
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = r.get::<CharcoalKey>();
    let mut mx = manager.unwrap().lock().await;
    let handler =  mx.players.get_mut(&guild_id.to_string()).unwrap();


    handler.loop_x_times(times).await;

    check_msg(msg.channel_id.say(&ctx.http, format!("Looping {} time(s)",times)).await);

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn position(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let position = match args.single::<u64>() {
        Ok(position) => position,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide song position").await);

            return Ok(());
        },
    };

    let r = ctx.data.read().await;
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    let manager = r.get::<CharcoalKey>();
    let mut mx = manager.unwrap().lock().await;
    let handler =  mx.players.get_mut(&guild_id.to_string()).unwrap();


    handler.seek_to_position(Duration::from_secs(position)).await;

    check_msg(msg.channel_id.say(&ctx.http, "Seeking...").await);

    Ok(())
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}