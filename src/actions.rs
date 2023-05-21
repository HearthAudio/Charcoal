//! Actions that can be used to interact with a Hearth server

/// Provides basic functionality to create a job on the hearth server, join a channel, and exit a channel
pub mod channel_manager;

/// Allows you to start playback using an HttpRequest or from a Youtube URL
pub mod player;

/// Provides functionality that can be used once you start playing a track such as: looping, pausing, and resuming.
pub mod track_manager;
pub mod standard;