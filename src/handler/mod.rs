use std::future::Future;

use crate::types::Update;

pub use self::longpoll::*;
#[cfg(feature = "webhook")]
pub use self::webhook::*;

mod longpoll;

#[cfg(feature = "webhook")]
mod webhook;

/// Represents an update handler for processing updates received from the Telegram Bot API.
pub trait UpdateHandler {
    /// Type of a future returned by [`Self::handle`] method
    type Future: Future<Output = ()>;

    /// Handles a received update.
    ///
    /// # Arguments
    ///
    /// * `update` - The received update from the Telegram Bot API.
    fn handle(&self, update: Update) -> Self::Future;
}
