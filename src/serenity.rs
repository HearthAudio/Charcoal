//! Provides ClientBuilder extension for super easy use with serenity
use std::sync::{Arc};
use futures::executor;
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


pub trait SerenityInit {
    #[must_use]
    fn register_charcoal(self,broker: String) -> Self;
}

impl SerenityInit for ClientBuilder {
    fn register_charcoal(self,broker: String) -> Self {
        let c = init_charcoal(broker);
        self.type_map_insert::<CharcoalKey>(executor::block_on(c))
    }
}

// pub use crate::serenity::ClientBuilder;