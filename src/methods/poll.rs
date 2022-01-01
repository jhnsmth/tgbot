use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, InlineKeyboardMarkup, Integer, Message, ParseMode, Poll, PollKind, ReplyMarkup, TextEntity},
};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
struct PollParameters {
    chat_id: ChatId,
    question: String,
    options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    kind: Option<PollKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allows_multiple_answers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    correct_option_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_entities: Option<Vec<TextEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_date: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl PollParameters {
    fn new(chat_id: ChatId, question: String, kind: PollKind) -> Self {
        Self {
            chat_id,
            question,
            options: Vec::new(),
            is_anonymous: None,
            kind: Some(kind),
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

/// Use this method to send a quiz
///
/// On success, the sent Message is returned
#[derive(Clone, Debug, Serialize)]
pub struct SendQuiz {
    #[serde(flatten)]
    inner: PollParameters,
}

impl SendQuiz {
    /// Creates a new SendQuiz
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * question - Quiz question, 1-300 characters
    pub fn new<C, Q>(chat_id: C, question: Q) -> Self
    where
        C: Into<ChatId>,
        Q: Into<String>,
    {
        Self {
            inner: PollParameters::new(chat_id.into(), question.into(), PollKind::Quiz),
        }
    }

    /// Adds an answer option (1-100 characters)
    pub fn option<O>(mut self, option: O) -> Self
    where
        O: Into<String>,
    {
        self.inner.options.push(option.into());
        self
    }

    /// True, if the quiz needs to be anonymous, defaults to True
    #[allow(clippy::wrong_self_convention)]
    pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.inner.is_anonymous = Some(is_anonymous);
        self
    }

    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    pub fn correct_option_id(mut self, correct_option_id: Integer) -> Self {
        self.inner.correct_option_id = Some(correct_option_id);
        self
    }

    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon
    ///
    /// 0-200 characters with at most 2 line feeds after entities parsing
    pub fn explanation<E: Into<String>>(mut self, explanation: E) -> Self {
        self.inner.explanation = Some(explanation.into());
        self
    }

    /// Mode for parsing entities in the explanation
    ///
    /// Explanation entities will be set to None when this method is called
    pub fn explanation_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.inner.explanation_parse_mode = Some(parse_mode);
        self.inner.explanation_entities = None;
        self
    }

    /// List of special entities that appear in the poll explanation
    ///
    /// Explanation parse mode will be set to None when this method is called
    pub fn explanation_entities(mut self, entities: Vec<TextEntity>) -> Self {
        self.inner.explanation_entities = Some(entities);
        self.inner.explanation_parse_mode = None;
        self
    }

    /// Amount of time in seconds the poll will be active after creation, 5-600
    ///
    /// Can't be used together with close_date (close_date will be set to None)
    pub fn open_period(mut self, period: Integer) -> Self {
        self.inner.open_period = Some(period);
        self.inner.close_date = None;
        self
    }

    /// Point in time (Unix timestamp) when the poll will be automatically closed
    ///
    /// Must be at least 5 and no more than 600 seconds in the future.
    /// Can't be used together with open_period (open_period will be set to None)
    pub fn close_date(mut self, close_date: Integer) -> Self {
        self.inner.close_date = Some(close_date);
        self.inner.open_period = None;
        self
    }

    /// Pass True, if the poll needs to be immediately closed
    #[allow(clippy::wrong_self_convention)]
    pub fn is_closed(mut self, is_closed: bool) -> Self {
        self.inner.is_closed = Some(is_closed);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.inner.disable_notification = Some(disable_notification);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.inner.protect_content = Some(protect_content);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.inner.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.inner.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R>(mut self, reply_markup: R) -> Self
    where
        R: Into<ReplyMarkup>,
    {
        self.inner.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendQuiz {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::json("sendPoll", self)
    }
}

/// Use this method to send a native poll
///
/// On success, the sent Message is returned
#[derive(Clone, Debug, Serialize)]
pub struct SendPoll {
    #[serde(flatten)]
    inner: PollParameters,
}

impl SendPoll {
    /// Creates a new SendPoll
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * question - Poll question, 1-300 characters
    pub fn new<C, Q>(chat_id: C, question: Q) -> Self
    where
        C: Into<ChatId>,
        Q: Into<String>,
    {
        Self {
            inner: PollParameters::new(chat_id.into(), question.into(), PollKind::Regular),
        }
    }

    /// Adds an answer option (1-100 characters)
    pub fn option<O>(mut self, option: O) -> Self
    where
        O: Into<String>,
    {
        self.inner.options.push(option.into());
        self
    }

    /// True, if the poll needs to be anonymous, defaults to True
    #[allow(clippy::wrong_self_convention)]
    pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.inner.is_anonymous = Some(is_anonymous);
        self
    }

    /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
    pub fn allows_multiple_answers(mut self, allows_multiple_answers: bool) -> Self {
        self.inner.allows_multiple_answers = Some(allows_multiple_answers);
        self
    }

    /// Amount of time in seconds the poll will be active after creation, 5-600
    ///
    /// Can't be used together with close_date (close_date will be set to None)
    pub fn open_period(mut self, period: Integer) -> Self {
        self.inner.open_period = Some(period);
        self.inner.close_date = None;
        self
    }

    /// Point in time (Unix timestamp) when the poll will be automatically closed
    ///
    /// Must be at least 5 and no more than 600 seconds in the future.
    /// Can't be used together with open_period (open_period will be set to None)
    pub fn close_date(mut self, close_date: Integer) -> Self {
        self.inner.close_date = Some(close_date);
        self.inner.open_period = None;
        self
    }

    /// Pass True, if the poll needs to be immediately closed
    #[allow(clippy::wrong_self_convention)]
    pub fn is_closed(mut self, is_closed: bool) -> Self {
        self.inner.is_closed = Some(is_closed);
        self
    }

    /// Sends the message silently
    ///
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.inner.disable_notification = Some(disable_notification);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.inner.protect_content = Some(protect_content);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.inner.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.inner.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R>(mut self, reply_markup: R) -> Self
    where
        R: Into<ReplyMarkup>,
    {
        self.inner.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendPoll {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::json("sendPoll", self)
    }
}

/// Use this method to stop a poll which was sent by the bot
///
/// On success, the stopped Poll with the final results is returned
#[derive(Clone, Debug, Serialize)]
pub struct StopPoll {
    chat_id: ChatId,
    message_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

/// Use this method to stop a quiz which was sent by the bot
///
/// On success, the stopped Quiz with the final results is returned
pub type StopQuiz = StopPoll;

impl StopPoll {
    /// Creates a new StopPoll
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the original message with the poll
    pub fn new<C>(chat_id: C, message_id: Integer) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }

    /// A JSON-serialized object for a new message inline keyboard
    pub fn reply_markup<R: Into<InlineKeyboardMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for StopPoll {
    type Response = Poll;

    fn into_request(self) -> Request {
        Request::json("stopPoll", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::{ForceReply, InlineKeyboardButton},
    };
    use serde_json::Value;

    #[test]
    fn send_quiz() {
        let request = SendQuiz::new(1, "Q")
            .option("O1")
            .option("O2")
            .is_anonymous(false)
            .correct_option_id(0)
            .is_closed(false)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendPoll");
        match request.into_body() {
            RequestBody::Json(data) => {
                let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
                assert_eq!(
                    data,
                    serde_json::json!({
                        "chat_id": 1,
                        "question": "Q",
                        "type": "quiz",
                        "options": ["O1", "O2"],
                        "is_anonymous": false,
                        "is_closed": false,
                        "correct_option_id": 0,
                        "disable_notification": true,
                        "protect_content": true,
                        "reply_to_message_id": 1,
                        "allow_sending_without_reply": true,
                        "reply_markup": {
                            "force_reply": true
                        }
                    })
                );
            }
            data => panic!("Unexpected request data: {:?}", data),
        }
    }

    #[test]
    fn send_poll() {
        let request = SendPoll::new(1, "Q")
            .option("O1")
            .option("O2")
            .is_anonymous(false)
            .allows_multiple_answers(true)
            .is_closed(false)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendPoll");
        match request.into_body() {
            RequestBody::Json(data) => {
                let data: Value = serde_json::from_str(&data.unwrap()).unwrap();

                assert_eq!(
                    data,
                    serde_json::json!({
                        "chat_id": 1,
                        "question": "Q",
                        "type": "regular",
                        "options": ["O1", "O2"],
                        "is_anonymous": false,
                        "is_closed": false,
                        "allows_multiple_answers": true,
                        "disable_notification": true,
                        "protect_content": true,
                        "reply_to_message_id": 1,
                        "allow_sending_without_reply": true,
                        "reply_markup": {
                            "force_reply": true
                        }
                    })
                );
            }
            data => panic!("Unexpected request data: {:?}", data),
        }
    }

    #[test]
    fn stop_poll() {
        let request = StopPoll::new(1, 2)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/stopPoll");
        match request.into_body() {
            RequestBody::Json(data) => {
                let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
                assert_eq!(data["chat_id"], 1);
                assert_eq!(data["message_id"], 2);
                assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
            }
            data => panic!("Unexpected request data: {:?}", data),
        }
    }
}
