//! Provides ClientBuilder extension for super easy use with serenity
use std::sync::{Arc};
use futures::executor;

use serenity::prelude::{TypeMapKey};
use crate::{Charcoal, CharcoalConfig, init_charcoal};
// pub use serenity::client::ClientBuilder;
use serenity::*;
pub use serenity::client::ClientBuilder;
use tokio::sync::Mutex;

pub struct CharcoalKey;

impl TypeMapKey for CharcoalKey {
    type Value = Arc<Mutex<Charcoal>>;
}


pub trait SerenityInit {
    #[must_use]
    /// Initializes charcoal and registers it in the Serenity type-map
    fn register_charcoal(self,broker: String,config: CharcoalConfig) -> Self;
}

impl SerenityInit for ClientBuilder {
    fn register_charcoal(self,broker: String,config: CharcoalConfig) -> Self {
        let c = init_charcoal(broker,config);
        self.type_map_insert::<CharcoalKey>(executor::block_on(c))
    }
}
