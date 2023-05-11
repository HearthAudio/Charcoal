//! Provides ClientBuilder extension for super easy use with serenity
use serenity::prelude::{TypeMap, TypeMapKey};
use tokio::sync::RwLockReadGuard;
use crate::{Charcoal, init_charcoal};
// pub use serenity::client::ClientBuilder;
use serenity::*;
pub use serenity::client::ClientBuilder;

pub struct CharcoalKey;

impl TypeMapKey for CharcoalKey {
    type Value = Charcoal;
}

pub trait SerenityInit {
    #[must_use]
    fn register_charcoal(self) -> Self;
}

impl SerenityInit for ClientBuilder {
    fn register_charcoal(self) -> Self {
        self.type_map_insert::<CharcoalKey>(init_charcoal())
    }
}

// pub use crate::serenity::ClientBuilder;