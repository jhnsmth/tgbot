use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Float, Integer, Message, ReplyMarkup},
};
use serde::Serialize;

/// Send information about a venue
#[derive(Clone, Debug, Serialize)]
pub struct SendVenue {
    chat_id: ChatId,
    latitude: Float,
    longitude: Float,
    title: String,
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_type: Option<String>,
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

impl SendVenue {
    /// Creates a new SendVenue with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * latitude - Latitude of the venue
    /// * longitude - Longitude of the venue
    /// * title - Name of the venue
    /// * address - Address of the venue
    pub fn new<C, T, A>(chat_id: C, latitude: Float, longitude: Float, title: T, address: A) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
        A: Into<String>,
    {
        SendVenue {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    /// Foursquare identifier of the venue
    pub fn foursquare_id<S: Into<String>>(mut self, foursquare_id: S) -> Self {
        self.foursquare_id = Some(foursquare_id.into());
        self
    }

    /// Foursquare type of the venue, if known
    ///
    /// For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”
    pub fn foursquare_type<S: Into<String>>(mut self, foursquare_type: S) -> Self {
        self.foursquare_type = Some(foursquare_type.into());
        self
    }

    /// Google Places identifier of the venue
    pub fn google_place_id<S: Into<String>>(mut self, google_place_id: S) -> Self {
        self.google_place_id = Some(google_place_id.into());
        self
    }

    /// Google Places type of the venue.
    ///
    /// <https://developers.google.com/places/web-service/supported_types>
    pub fn google_place_type<S: Into<String>>(mut self, google_place_type: S) -> Self {
        self.google_place_type = Some(google_place_type.into());
        self
    }

    // Sends the message silently
    /// Users will receive a notification with no sound
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }

    /// If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: Integer) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }

    /// Pass True, if the message should be sent even
    /// if the specified replied-to message is not found
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }

    /// Additional interface options
    pub fn reply_markup<R: Into<ReplyMarkup>>(mut self, reply_markup: R) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for SendVenue {
    type Response = Message;

    fn into_request(self) -> Request {
        Request::json("sendVenue", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::ForceReply,
    };
    use serde_json::Value;

    #[test]
    fn send_venue() {
        let request = SendVenue::new(1, 2.0, 3.0, "title", "addr")
            .foursquare_id("f-id")
            .foursquare_type("f-type")
            .google_place_id("g-id")
            .google_place_type("g-type")
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(request.build_url("base-url", "token"), "base-url/bottoken/sendVenue");
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "chat_id": 1,
                    "latitude": 2.0,
                    "longitude": 3.0,
                    "title": "title",
                    "address": "addr",
                    "foursquare_id": "f-id",
                    "foursquare_type": "f-type",
                    "google_place_id": "g-id",
                    "google_place_type": "g-type",
                    "disable_notification": true,
                    "protect_content": true,
                    "reply_to_message_id": 1,
                    "allow_sending_without_reply": true,
                    "reply_markup": {"force_reply": true}
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}
