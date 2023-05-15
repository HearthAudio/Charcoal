use std::env;
use std::sync::Arc;

// Import the `Context` to handle commands.
use serenity::client::Context;
use charcoal::serenity::{CharcoalKey, ClientBuilder, SerenityInit};

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
use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;
use charcoal::actions::channel_manager::ChannelManager;
use charcoal::actions::player::Player;
use charcoal::{Charcoal, PlayerObject};
use charcoal::actions::track_manager::TrackManager;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
#[commands(join, leave, play, ping,metadata)]
struct General;

#[tokio::main]
async fn main() {

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
        .register_charcoal("kafka-185690f4-maxall4-aea3.aivencloud.com:23552".to_string())
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
    let manager = r.get::<CharcoalKey>().unwrap();

    let mut handler = PlayerObject::new().await;
    handler.join_channel(guild_id.to_string(),connect_to.to_string()).await;

    // r.insert::<PlayerObjectKey>(Arc::new(Mutex::new(handler)));
    manager.lock().await.players.insert(guild_id.to_string(), handler);
    println!("Inserted PLY");
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

    let meta = manager.get_metadata().await;
    println!("{:?}",meta.unwrap());
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

    manager.exit_channel().await;

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
    println!("Getting manager");
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;
    println!("GET: {}",guild_id.to_string());
    let mut manager = r.get::<CharcoalKey>();
    let mut mx = manager.unwrap().lock().await;
    let handler =  mx.players.get_mut(&guild_id.to_string()).unwrap();
    println!("GOT MANAGER");


    handler.play_from_http(url).await;
    check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);

    Ok(())
}


/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}