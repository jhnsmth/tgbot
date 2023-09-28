use serde::Serialize;

use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, EditMessageResult, InlineKeyboardMarkup, Integer, ParseMode, TextEntities, TextEntity},
};

/// Edit text and game messages sent by the bot or via the bot (for inline bots)
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageText {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageText {
    /// Creates a new EditMessageText
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * text - New text of the message
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, message_id: Integer, text: S) -> Self {
        EditMessageText {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageText
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * text - New text of the message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S, text: S) -> Self {
        EditMessageText {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Parse mode
    ///
    /// Entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.entities = None;
        self
    }

    /// List of special entities that appear in message text
    ///
    /// Parse mode will be set to None when this method is called
    pub fn entities<T>(mut self, entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Disables link previews for links in this message
    pub fn disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }

    /// Inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageText {
    type Response = EditMessageResult;

    fn into_request(self) -> Request {
        Request::json("editMessageText", self)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        request::{RequestBody, RequestMethod},
        types::InlineKeyboardButton,
    };

    use super::*;

    #[test]
    fn edit_message_text() {
        let request = EditMessageText::new(1, 2, "text")
            .parse_mode(ParseMode::Markdown)
            .disable_web_page_preview(true)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/editMessageText"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "chat_id": 1,
                    "message_id": 2,
                    "text": "text",
                    "parse_mode": "Markdown",
                    "disable_web_page_preview": true,
                    "reply_markup": {
                        "inline_keyboard": [
                            [
                                {
                                    "text": "text",
                                    "url": "url"
                                }
                            ]
                        ]
                    }
                })
            );
        } else {
            panic!("Unexpected request body");
        }

        let request = EditMessageText::with_inline_message_id("msg-id", "text")
            .entities(vec![TextEntity::bold(0..4)])
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/editMessageText"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "inline_message_id": "msg-id",
                    "text": "text",
                    "entities": [
                        {
                            "type": "bold",
                            "offset": 0,
                            "length": 4
                        }
                    ]
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}
