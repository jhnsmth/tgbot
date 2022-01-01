use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer},
};
use serde::Serialize;

/// Add a message to the list of pinned messages in a chat
///
/// If the chat is not a private chat, the bot must be an administrator
/// in the chat for this to work and must have the 'can_pin_messages'
/// admin right in a supergroup or 'can_edit_messages' admin right in a channel.
#[derive(Clone, Debug, Serialize)]
pub struct PinChatMessage {
    chat_id: ChatId,
    message_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}

impl PinChatMessage {
    /// Creates a new PinChatMessage
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of a message to pin
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        PinChatMessage {
            chat_id: chat_id.into(),
            message_id,
            disable_notification: None,
        }
    }

    /// Pass True, if it is not necessary to send a notification to all chat members about the new pinned message
    ///
    /// Notifications are always disabled in channels
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
}

impl Method for PinChatMessage {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("pinChatMessage", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn pin_chat_message() {
        let request = PinChatMessage::new(1, 2).disable_notification(true).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/pinChatMessage"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["message_id"], 2);
            assert!(data["disable_notification"].as_bool().unwrap());
        } else {
            panic!("Unexpected request body");
        }
    }
}
