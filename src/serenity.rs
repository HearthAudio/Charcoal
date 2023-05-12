//! Provides ClientBuilder extension for super easy use with serenity
use std::sync::{Arc};
use kafka::producer::Producer;
use serenity::prelude::{TypeMap, TypeMapKey};
use crate::{Charcoal, init_charcoal};
// pub use serenity::client::ClientBuilder;
use serenity::*;
pub use serenity::client::ClientBuilder;
use tokio::sync::Mutex;

pub struct CharcoalKey;

impl TypeMapKey for CharcoalKey {
    type Value = Arc<Mutex<Charcoal>>;
}

#[async_trait]
pub trait SerenityInit {
    #[must_use]
    async fn register_charcoal(self,broker: String) -> Self;
}

#[async_trait]
impl SerenityInit for ClientBuilder {
    async fn register_charcoal(self,broker: String) -> Self {
        self.type_map_insert::<CharcoalKey>(init_charcoal(broker).await)
    }
}

// pub use crate::serenity::ClientBuilder;