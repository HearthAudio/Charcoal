use crate::Charcoal;

trait Player {
    fn play_from_http(&self);
    fn play_from_youtube(&self);
}

impl Player for Charcoal {
    fn play_from_http(&self) {

    }
    fn play_from_youtube(&self) {

    }
}