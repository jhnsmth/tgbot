use crate::types::{
    animation::Animation,
    audio::Audio,
    chat::Chat,
    contact::Contact,
    dice::Dice,
    document::Document,
    game::Game,
    location::{Location, ProximityAlertTriggered},
    passport::PassportData,
    payments::{Invoice, SuccessfulPayment},
    photo_size::PhotoSize,
    poll::Poll,
    primitive::Integer,
    reply_markup::InlineKeyboardMarkup,
    stickers::Sticker,
    text::RawTextEntity,
    user::User,
    venue::Venue,
    video::Video,
    video_note::VideoNote,
    voice::Voice,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct RawMessage {
    pub message_id: Integer,
    pub from: Option<User>,
    pub sender_chat: Option<Chat>,
    pub date: Integer,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<Integer>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    pub forward_date: Option<Integer>,
    pub reply_to_message: Option<Box<RawMessage>>,
    pub via_bot: Option<User>,
    pub edit_date: Option<Integer>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<RawTextEntity>>,
    pub caption_entities: Option<Vec<RawTextEntity>>,
    pub audio: Option<Audio>,
    pub animation: Option<Animation>,
    pub dice: Option<Dice>,
    pub document: Option<Document>,
    pub game: Option<Game>,
    pub photo: Option<Vec<PhotoSize>>,
    pub poll: Option<Poll>,
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub voice: Option<Voice>,
    pub video_note: Option<VideoNote>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<Integer>,
    pub migrate_from_chat_id: Option<Integer>,
    pub pinned_message: Option<Box<RawMessage>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
    pub connected_website: Option<String>,
    pub passport_data: Option<PassportData>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
