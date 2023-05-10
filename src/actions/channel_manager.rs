use crate::Charcoal;

trait ChannelManager {
    fn join_channel(&self);
    fn exit_channel(&self);
}

impl ChannelManager for Charcoal {
    fn join_channel(&self) {

    }
    fn exit_channel(&self) {

    }
}