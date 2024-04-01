use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer, Message, ReplyMarkup, ReplyParameters},
};

#[cfg(test)]
mod tests;

/// Represents a phone contact.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Contact {
    /// First name.
    pub first_name: String,
    /// Phone number.
    pub phone_number: String,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Identifier in Telegram.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Integer>,
    /// Additional data in the form of a vCard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

impl Contact {
    /// Creates a new `Contact`.
    ///
    /// # Arguments
    ///
    /// * `first_name` - First name.
    /// * `phone_number` - Phone number.
    pub fn new<A, B>(first_name: A, phone_number: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            first_name: first_name.into(),
            phone_number: phone_number.into(),
            last_name: None,
            user_id: None,
            vcard: None,
        }
    }

    /// Sets a new last name.
    ///
    /// # Arguments
    ///
    /// * `value` - Last name.
    pub fn with_last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(value.into());
        self
    }

    /// Sets a new user ID.
    ///
    /// # Arguments
    ///
    /// * `value` - User ID.
    pub fn with_user_id(mut self, value: Integer) -> Self {
        self.user_id = Some(value);
        self
    }

    /// Sets a new vCard.
    ///
    /// # Arguments
    ///
    /// * `value` - Additional data about the contact in the form of a vCard; 0-2048 bytes.
    pub fn with_vcard<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.vcard = Some(value.into());
        self
    }
}

/// Sends a phone contact.
#[derive(Clone, Debug, Serialize)]
pub struct SendContact {
    chat_id: ChatId,
    first_name: String,
    phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
}

impl SendContact {
    /// Creates a new `SendContact`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `first_name` - First name.
    /// * `phone_number` - Phone number.
    pub fn new<A, B, C>(chat_id: A, first_name: B, phone_number: C) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
        C: Into<String>,
    {
        SendContact {
            chat_id: chat_id.into(),
            first_name: first_name.into(),
            phone_number: phone_number.into(),
            business_connection_id: None,
            disable_notification: None,
            last_name: None,
            message_thread_id: None,
            protect_content: None,
            reply_markup: None,
            reply_parameters: None,
            vcard: None,
        }
    }

    /// Sets a new business connection ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the business connection.
    pub fn with_business_connection_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.business_connection_id = Some(value.into());
        self
    }

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a last name.
    ///
    /// # Arguments
    ///
    /// * `value` - Contact's last name.
    pub fn with_last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(value.into());
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.reply_parameters = Some(value);
        self
    }

    /// Sets a new vCard.
    ///
    /// # Arguments
    ///
    /// * `value` - Additional data about the contact in the form of a vCard; 0-2048 bytes.
    pub fn with_vcard<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.vcard = Some(value.into());
        self
    }
}

impl Method for SendContact {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendContact", self)
    }
}
