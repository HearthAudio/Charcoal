//! Provides ClientBuilder extension for super easy use with serenity
use futures::executor;
use std::sync::Arc;

use crate::{init_charcoal, Charcoal, CharcoalConfig};
use serenity::prelude::TypeMapKey;
// pub use serenity::client::ClientBuilder;
pub use serenity::client::ClientBuilder;
use serenity::*;
use tokio::sync::Mutex;

pub struct CharcoalKey;

impl TypeMapKey for CharcoalKey {
    type Value = Arc<Mutex<Charcoal>>;
}

pub trait SerenityInit {
    #[must_use]
    /// Initializes charcoal and registers it in the Serenity type-map
    fn register_charcoal(self, broker: String, config: CharcoalConfig) -> Self;
}

impl SerenityInit for ClientBuilder {
    fn register_charcoal(self, broker: String, config: CharcoalConfig) -> Self {
        let c = init_charcoal(broker, config);
        self.type_map_insert::<CharcoalKey>(executor::block_on(c))
    }
}

#[macro_export]
macro_rules! get_handler_from_serenity_mutable {
    ($ctx: expr,$msg: expr,$reference: ident) => {
        let r = $ctx.data.read().await;
        // Get the GuildID
        let guild = $msg.guild(&$ctx.cache).unwrap();
        let guild_id = guild.id;
        // Get the charcoal manager from the serenity typemap
        let manager = r.get::<CharcoalKey>();
        let mut mx = manager.unwrap().lock().await;
        // Get the PlayerObject
        let mut players = mx.players.write().await;
        $reference = players.get_mut(&guild_id.to_string());
    };
}

#[macro_export]
macro_rules! get_handler_from_serenity {
    ($ctx: expr,$msg: expr,$reference: ident) => {
        let r = $ctx.data.read().await;
        // Get the GuildID
        let guild = $msg.guild(&$ctx.cache).unwrap();
        let guild_id = guild.id;
        // Get the charcoal manager from the serenity typemap
        let manager = r.get::<CharcoalKey>();
        let mut mx = manager.unwrap().lock().await;
        // Get the PlayerObject
        let players = mx.players.read().await;
        $reference = players.get(&guild_id.to_string());
    };
}
