//! Provides functions to wait for actions to complete
//! You should probably avoid using these when possible as they are rather inefficient
#[derive(Clone)]
pub struct AwaitAction {
    pub(crate) action_completed: bool
}

impl AwaitAction {
    /// Blocks this thread until action is completed
    pub async fn subscribe(&self) {
        loop {
            if self.action_completed {
                break
            }
        }
    }
}