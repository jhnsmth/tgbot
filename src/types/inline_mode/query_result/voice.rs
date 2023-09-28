use serde::Serialize;

use crate::types::{
    inline_mode::message_content::InputMessageContent, parse_mode::ParseMode, primitive::Integer,
    reply_markup::InlineKeyboardMarkup, text::TextEntity, TextEntities,
};

/// Link to a voice recording in an .ogg container encoded with OPUS
///
/// By default, this voice recording will be sent by the user
/// Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the the voice message
#[derive(Clone, Debug, Serialize)]
pub struct InlineQueryResultVoice {
    id: String,
    voice_url: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultVoice {
    /// Creates a new InlineQueryResultVoice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * voice_url - A valid URL for the voice recording
    /// * title - Recording title
    pub fn new<I, U, T>(id: I, voice_url: U, title: T) -> Self
    where
        I: Into<String>,
        U: Into<String>,
        T: Into<String>,
    {
        InlineQueryResultVoice {
            id: id.into(),
            voice_url: voice_url.into(),
            title: title.into(),
            caption: None,
            caption_entities: None,
            parse_mode: None,
            voice_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Caption, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// List of special entities that appear in the caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn caption_entities<T>(mut self, caption_entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(caption_entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets parse mode
    ///
    /// Caption entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Recording duration in seconds
    pub fn voice_duration(mut self, voice_duration: Integer) -> Self {
        self.voice_duration = Some(voice_duration);
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the voice recording
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}
