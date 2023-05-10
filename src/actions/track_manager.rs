use crate::PlayerObject;

trait TrackManager {
    fn set_playback_volume(&self);
    fn force_stop_loop(&self);
    fn loop_indefinitely(&self);
    fn loop_x_times(&self);
    fn seek_to_position(&self);
    fn resume_playback(&self);
    fn pause_playback(&self);
}

impl TrackManager for PlayerObject {
    fn set_playback_volume(&self) {}
    fn force_stop_loop(&self) {}
    fn loop_indefinitely(&self) {}
    fn loop_x_times(&self) {}
    fn seek_to_position(&self) {}
    fn resume_playback(&self) {}
    fn pause_playback(&self) {}
}