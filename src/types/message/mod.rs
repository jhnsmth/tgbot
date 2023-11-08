use serde::{Deserialize, Serialize};

use crate::types::{Chat, InlineKeyboardMarkup, Integer, Text, User};

pub use self::{command::*, data::*, forward::*, methods::*, sender::*};

#[cfg(test)]
mod tests;

mod command;
mod data;
mod forward;
mod methods;
mod sender;

/// Represents a result of editMessage* requests
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
#[serde(untagged)]
pub enum EditMessageResult {
    /// Returned if edited message is sent by the bot
    Message(Message),
    /// Returned if edited message is NOT sent by the bot
    Bool(bool),
}

/// Represents a message
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message {
    /// Chat the message belongs to
    pub chat: Chat,
    /// Date the message was sent in Unix time
    pub date: Integer,
    /// Date the message was last edited in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<Integer>,
    /// Whether the message can't be forwarded
    #[serde(default)]
    pub has_protected_content: bool,
    /// Unique message identifier inside the chat
    #[serde(rename = "message_id")]
    pub id: Integer,
    /// Whether the message is a channel post that
    /// was automatically forwarded
    /// to the connected discussion group
    #[serde(default)]
    pub is_automatic_forward: bool,
    /// Sender of the message
    #[serde(flatten)]
    pub sender: MessageSender,
    /// Author signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    /// Forwarded data
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<Forward>,
    /// Whether the message media is covered by a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    /// Whether the message is sent to a forum topic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,
    /// Unique identifier of a media message group this message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<Integer>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// For replies, the original message
    ///
    /// Note that the Message object in this field will not contain further
    /// `reply_to` fields even if it itself is a reply.
    #[serde(rename = "reply_to_message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<Box<Message>>,
    /// Bot through which the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,

    /// Contains message data
    #[serde(flatten)]
    pub data: MessageData,
}

impl Message {
    /// Creates a new Message
    ///
    /// # Arguments
    ///
    /// * id - Unique message identifier inside the chat
    /// * date - Date the message was sent in Unix time
    /// * chat - Chat the message belongs to
    /// * data - Data of the message
    /// * sender - Sender of the message
    pub fn new<A, B>(id: Integer, date: Integer, chat: A, data: MessageData, sender: B) -> Self
    where
        A: Into<Chat>,
        B: Into<MessageSender>,
    {
        Self {
            chat: chat.into(),
            data,
            date,
            edit_date: None,
            has_protected_content: false,
            id,
            is_automatic_forward: false,
            sender: sender.into(),
            author_signature: None,
            forward: None,
            has_media_spoiler: None,
            is_topic_message: None,
            media_group_id: None,
            message_thread_id: None,
            reply_markup: None,
            reply_to: None,
            via_bot: None,
        }
    }

    /// Returns true if message has edited and false otherwise
    pub fn is_edited(&self) -> bool {
        self.edit_date.is_some()
    }

    /// Returns a text of the message (includes caption)
    pub fn get_text(&self) -> Option<&Text> {
        match self.data {
            MessageData::Text(ref text)
            | MessageData::Audio(MessageDataAudio {
                caption: Some(ref text),
                ..
            })
            | MessageData::Document(MessageDataDocument {
                caption: Some(ref text),
                ..
            })
            | MessageData::Photo(MessageDataPhoto {
                caption: Some(ref text),
                ..
            })
            | MessageData::Video(MessageDataVideo {
                caption: Some(ref text),
                ..
            })
            | MessageData::Voice(MessageDataVoice {
                caption: Some(ref text),
                ..
            }) => Some(text),
            _ => None,
        }
    }

    /// Sets a new chat
    ///
    /// # Arguments
    ///
    /// * value - Chat
    pub fn with_chat<T>(mut self, value: T) -> Self
    where
        T: Into<Chat>,
    {
        self.chat = value.into();
        self
    }

    /// Sets a new data
    ///
    /// # Arguments
    ///
    /// * value - Data of the message
    pub fn with_data(mut self, value: MessageData) -> Self {
        self.data = value;
        self
    }

    /// Sets a new date
    ///
    /// # Arguments
    ///
    /// * value - Date; Unix timestamp
    pub fn with_date(mut self, value: Integer) -> Self {
        self.date = value;
        self
    }

    /// Sets a new edit date
    ///
    /// # Arguments
    ///
    /// * value - Edit date; Unix timestamp
    pub fn with_edit_date(mut self, value: Integer) -> Self {
        self.edit_date = Some(value);
        self
    }

    /// Sets a new value for the `has_protected_content` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether the message can't be forwarded
    pub fn with_has_protected_content(mut self, value: bool) -> Self {
        self.has_protected_content = value;
        self
    }

    /// Sets a new ID
    ///
    /// # Arguments
    ///
    /// * value - Unique identifier inside a chat
    pub fn with_id(mut self, value: Integer) -> Self {
        self.id = value;
        self
    }

    /// Sets a new value for the `is_automatic_forward` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether the message was automatically forwarded
    pub fn with_is_automatic_forward(mut self, value: bool) -> Self {
        self.is_automatic_forward = value;
        self
    }

    /// Sets a new sender
    ///
    /// # Arguments
    ///
    /// * value - Sender
    pub fn with_sender<T>(mut self, value: T) -> Self
    where
        T: Into<MessageSender>,
    {
        self.sender = value.into();
        self
    }

    /// Sets a new author signature
    ///
    /// # Arguments
    ///
    /// * value - Author signature
    pub fn with_author_signature<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.author_signature = Some(value.into());
        self
    }

    /// Sets a new forward
    ///
    /// # Arguments
    ///
    /// * value - Forward
    pub fn with_forward(mut self, value: Forward) -> Self {
        self.forward = Some(value);
        self
    }

    /// Sets a new value for the `has_media_spoiler` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether the message has a media spoiler
    pub fn with_has_media_spoiler(mut self, value: bool) -> Self {
        self.has_media_spoiler = Some(value);
        self
    }

    /// Sets a new value for the `is_topic_message` flag
    ///
    /// # Arguments
    ///
    /// * value - Whether the message is a topic message
    pub fn with_is_topic_message(mut self, value: bool) -> Self {
        self.is_topic_message = Some(value);
        self
    }

    /// Sets a new media group ID
    ///
    /// # Arguments
    ///
    /// * value - Media group ID
    pub fn with_media_group_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.media_group_id = Some(value.into());
        self
    }

    /// Sets a new message thread ID
    ///
    /// # Arguments
    ///
    /// * value - Message thread ID
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new reply markup
    ///
    /// # Arguments
    ///
    /// * value - Inline keyboard
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets a new reply_to
    ///
    /// # Arguments
    ///
    /// * value - For replies, the original message
    pub fn with_reply_to(mut self, value: Message) -> Self {
        self.reply_to = Some(Box::new(value));
        self
    }

    /// Sets a new bot
    ///
    /// # Arguments
    ///
    /// * value - Bot through which the message was sent
    pub fn with_via_bot(mut self, value: User) -> Self {
        self.via_bot = Some(value);
        self
    }
}

/// Represents an unique message identifier
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: Integer,
}
