# Changelog

## 0.17.0 (02.02.2022)

- Removed `Config`.
- Added `Api::with_client` method.
- Spawn a new task for each incoming update in `LongPoll`.
- Retry a request on timeout error.
- Added `message_auto_delete_time` field to chat-related structs.

## 0.16.0 (01.01.2022)

- Added `Update::get_user_id` and `Update::get_user_username` methods.
- Added Bot API 5.6 support.
  - Added `protect_content` parameter to `send*` methods.
  - Added spoiler text entity support.
- Rust 2021 edition.

## 0.15.0 (27.12.2021)

- Updated tokio version to 1.15.
- Added Bot API 5.5 support.
  - Add `has_protected_content` parameter to `ChannelChat`, `GroupChat`, `SupergroupChat` and `Message` structs.
  - Add `has_private_forwards` parameter to `PrivateChat` struct.
  - Added `BanChatSenderChat` method.
  - Added `UnbanChatSenderChat` method.
- Added Bot API 5.4 support.
  - Added `ChatJoinRequest` struct.
  - Added `ApproveChatJoinRequest` method.
  - Added `DeclineChatJoinRequest` method.
  - Added `name` parameter to `EditChatInviteLink`, `CreateChatInviteLink` methods and `ChatInviteLink` struct.
  - Added `pending_join_request_count` parameter to `CharInviteLink` struct.
  - Added `creates_join_request` parameter to `EditChatInviteLink`, `CreateChatInviteLink` methods and `ChatInviteLink` struct.
  - Added `ChooseSticker` variant to `ChatAction` enum.

## 0.14.0 (06.09.2021)

- Added Bot API 5.3 support.
  - Added `BotCommandScope` enum.
  - Added `scope` and `language_code` parameters to `Set/GetMyCommands` methods.
  - Added `DeleteMyCommands` method.
  - Added `input_field_placeholder` parameter to `ReplyKeyboardMarkup` struct.
  - Added `input_field_placeholder` parameter to `ForceReply` struct.
  - Renamed `KickChatMember` method to `BanChatMember`.
  - Renamed `GetChatMembersCount` to `GetChatMemberCount`.
- Added Bot API 5.2 support.
  - Added `InputMessageContentInvoice` struct.
  - Changed type of `chat_id` argument from `Integer` to `ChatId` in `SendInvoice` method.
  - Added `max_tip_amount` and `suggested_tip_amounts` parameters to `SendInvoice` method.
  - Changed `SendInvoice::provider_data` method to accept a `&Serialize` type.
  - `SendInvoice.start_parameter` is optional now.
  - Added `InlineQuery.chat_type` field.
  - Added `MessageData::VoiceChatScheduled` variant.
  - Renamed `ChatAction::{RecordAudio, UploadAudio}` to `ChatAction::{RecordVoice, UploadVoice}`.
- Fixed parsing of command arguments when bot name is a part of the command.

## 0.13.0 (14.03.2021)

- Added Bot API 5.1 support.
  - Added `ChatMemberUpdated` struct.
  - Added `ChatInviteLink` struct.
  - Added `CreateChatInviteLink`, `EditChatInviteLink` and `RevokeChatInviteLink` methods.
  - Added voice chat service messages support.
    - `MessageData::VoiceChatStarted`.
    - `MessageData::VoiceChatEnded`.
    - `MessageData::VoiceChatParticipantsInvited`.
  - Added `AutoDeleteTimerChanged` service message support.
  - Added `ChatMemberAdministrator.can_manage_voice_chats` field.
  - Added `can_manage_voice_chats` parameter to `PromoteChatMember` method.
  - Added `revoke_messages` parameter to `KickChatMember` method.
  - Added `can_manage_chat` field to `ChatMemberAdministrator` struct.
  - Added `can_manage_chat` parameter to `PromoteChatMember` method.
  - Added `Bowling` dice.
- New `UpdateHandler` API.
  - Removed `async_trait` dependency.
  - `handle` method now takes an immutable reference to `self`.
  - Added `SyncedUpdateHandler` wrapper for non-sync handlers.
- New `TextEntity` API.
  - Use `Vec1` instead of `Vec` to guarantee that entities are not empty.
  - `TextEntityPosition` now contains `u32` values instead of `i64`.
  - You can convert ranges into `TextEntityPosition`.
- Added `Text::as_ref()` method to get data as `&str`.
- Implemented `String` to `Text` conversion.
- Added `UpdateKind::Unknown` for updates introduced in new Bot API versions.
- Added `Chat::get_id()` and `Chat::get_username()` methods.
- async-stream updated to 0.3 version.
- tokio updated to 1.3 version.
- Fixed error timeout in longpoll.

## 0.12.1 (11.01.2021)

- Fixed a panic when getting a list of bot commands.

## 0.12.0 (05.01.2021)

- Tokio 1.0 support

## 0.11.0 (15.11.2020)

- Added Bot API 5.0 support.
    - Allowed to specify text entities instead of specifying the `parse_mode`.
      - Added `entities` parameter to `SendMessage` and `EditMessageText` methods.
      - Added `explanation_entities` parameter to `SendQuiz` method
      - Added `caption_entities` parameter to
        `EditMessageCaption`, `SendAnimation`, `SendAudio`, `SendDocument`,
        `SendPhoto`, `SendVideo` and `SendVoice` methods.
      - Added `caption_entities` field to
        `InlineQueryResultAudio`, `InlineQueryResultDocument`, `InlineQueryResultGif`,
        `InlineQueryResultMpeg4Gif`, `InlineQueryResultPhoto`, `InlineQueryResultVideo`,
        `InlineQueryResultVoice`, `InlineQueryResultCachedAudio`, `InlineQueryResultCachedDocument`,
        `InlineQueryResultCachedMpeg4Gif`, `InlineQueryResultCachedPhoto`,
        `InlineQueryResultCachedMpeg4Gif`, `InlineQueryResultCachedVideo`,
        `InlineQueryResultCachedVoice`, `InputMediaAnimation`,
        `InputMediaAudio`, `InputMediaDocument`, `InputMediaPhoto` and `InputMediaVideo` structs.
      - Added `entities` field to `InputMessageContentText` struct.
    - Added football and slot machine dices.
    - Added `allow_sending_without_reply` parameter.
      Affected methods:
      - `SendMessage`
      - `SendPhoto`
      - `SendGame`
      - `SendPoll`
      - `SendAnimation`
      - `SendAudio`
      - `SendContact`
      - `SendDice`
      - `SendDocument`
      - `SendInvoice`
      - `SendLocation`
      - `SendMediaGroup`
      - `SendVenue`
      - `SendVideo`
      - `SendVideoNote`
      - `SendVoice`
      - `SendSticker`
    - Added `google_place_id` and `google_place_type` fields.
      Affected structs and methods:
      - `SendVenue` method.
      - `Venue` struct.
      - `InlineQueryResultVenue` struct.
      - `InputMessageContentVenue` struct.
    - Maximum poll question length increased to 300 characters.
    - Added `CopyMessage` method.
    - Reworked `TextEntity`.
      - Removed `Message.commands` field, use `Text::get_bot_commands()` instead.
      - Removed `TextEntityData`, `TextEntityMention` and `TextEntityLink` structs.
      - Added `TextEntityPosition` struct.
      - Added support of serializing `TextEntity`.
      - Added `Text::get_bot_commands()` method.
      - `User` struct now implements `serde::Serialize`
    - Added `author_signature` to message kinds for group chats.
    - Added `is_anonymous` parameter `PromoteChatMember` method.
    - Added `is_anonymous` field to `ChatMemberAdministrator` and `ChatMemberCreator` structs.
    - Added `sender_chat` field to `Message` struct.
    - Added horizontal accuracy support for location.
      - Added `horizontal_accuracy` field to
        `Location`, `InlineQueryResultLocation`
        and `InputMessageContentLocation` struct.
      - Added `horizontal_accuracy` parameter to `SendLocation` and `EditMessageLiveLocation` method.
    - Added proximity alert support for location.
      - Added `proximity_alert_radius` field to
        `Location`, `InlineQueryResultLocation`
        and `InputMessageContentLocation` struct.
      - Added `proximity_alert_radius` parameter to `SendLocation`
        and `EditMessageLiveLocation` method.
      - Added `ProximityAlertTriggered` service message.
    - Added heading support for location.
      - Added `heading` field to
        `Location`, `InlineQueryResultLocation` and `InputMessageContentLocation` struct.
      - Added `heading` parameter to `SendLocation` and `EditMessageLiveLocation` method.
    - Added `live_location` field to `Location` struct.
    - Added support for `Audio` and `Video` to `MediaGroup`.
    - Added `UnpinAllChatMessages` method.
    - Added `message_id` parameter to `UnpinChatMessage` method.
    - Added `disable_content_type_detection` field to `InputMediaDocument` struct.
    - Added `disable_content_type_detection` parameter to `SendDocument` method.
    - Added `file_name` to `Audio` and `Video` structs.
    - Added `only_if_banned` parameter to `UnbanChatMember` method.
    - Updated chat-related structs.
      - Added `ChatLocation` struct.
      - Added `linked_chat_id` field to `ChannelChat`.
      - Removed `all_members_are_administrators` field from `GroupChat`.
      - Added `bio` and `pinned_message` fields to `PrivateChat`.
      - Added `linked_chat_id` and `location` fields to `SupergroupChat`.
    - Added `drop_pending_updates` parameter to `SetWebhook` and `DeleteWebhook` methods.
    - Added information about IP address in webhooks.
      - Added `ip_address` parameter to `SetWebhook` method.
      - Added `ip_address` field to `WebhookInfo` struct.
    - Added `close` method.
    - Added `LogOut` method.

## 0.10.0 (20.06.2020)

- Added Bot API 4.9 support.
  * Added `via_bot` field to `Message` struct.
  * Added `thumb_mime_type` to `InlineQueryResultGif` and `InlineQueryResultMpeg4Gif` structs.
  * Added basketball dice. `DiceKind` is non-exhaustive now.
- Fixed a bug with proxy without username/password.

## 0.9.0 (26.04.2020)

- Added Bot API 4.8 support
  * `Poll` converted to enum.
  * Added support for quiz explanation.
  * Added `open_period` and `close_date` to poll/quiz.
  * `Dice.value` field is private now, use `value()` method instead.
  * Added DiceKind enum and `kind()` method to `Dice`.
- Public access to InlineKeyboard data.

## 0.8.0 (01.04.2020)

- Added Bot API 4.7 support
  * Added `SendDice` method.
  * Added `MessageData::Dice` variant with `Dice` type.
  * Added `BotCommand` type.
  * Added `SetMyCommands` and `GetMyCommands` methods.
  * Added TGS stickers support.
  * Added `thumb` field to `StickerSet` struct.
  * Added `SetStickerSetThumb` method.
- Renamed some message entity structs
  * `BotCommand` -> `TextEntityBotCommand`
  * `TextLink` -> `TextEntityLink`
  * `TextMention` -> `TextEntityMention`
- Added some missing trait derives

## 0.7.1 (16.03.2020)

- Impl `Eq` for `ChatId` and `UserId`.

## 0.7.0 (08.03.2020)

- `ChatId` type implements `Display` trait instead of `ToString`.
-  Added `Display` and `Serialize` trait implementations for `UserId` type.
- `ResponseError` fields are private now and `ResponseParameters` struct was removed,
   use corresponding methods instead.
- Added `Copy`, `Hash`, `PartialEq` and `PartialOrd` trait implementations for several types.
- Reduced memory usage when sending files.
  `InputFileReader` now takes `tokio::io::AsyncRead` instead of `std::io::Read`.
  `InputFile::path` is async and returns a `tokio::io::Result`.
- `download_file` method now returns a stream of bytes.
- Added `Command` type.

## 0.6.0 (27.01.2020)

- Added Bot API 4.6 support.
- Added `ResponseError::can_retry()` method.
- Added `ParseMode::escape()` method.
- Added `get_full_name`, `get_link` and `get_mention` methods to `User` type.
- Removed deprecated `RestrictChatMember` methods.

## 0.5.1 (10.01.2020)

- Return socks proxy support.
- Deprecate some `RestrictChatMember` methods (to be removed in 0.6.0).

## 0.5.0 (07.01.2020)

- Added async/await support.
- Added Bot API 4.5 support.
- Added rustls support.
- Fixed some errors when parsing update.
- Temporarily disabled socks proxy support.

## 0.4.4 (31.07.2019)

- Added Bot API 4.4 support.
- Added information about response data to poll request error.
- `RestrictChatMember` methods:
  `can_send_messages`,
  `can_send_media_messages`,
  `can_send_other_messages`,
  `can_add_web_page_previews` are now deprecated and will be removed in 0.5.0.
  Use `with_permissions` method instead.

## 0.4.3 (07.07.2019)

- Ignore invalid updates in webhook handler.

## 0.4.2 (01.06.2019)

- Added Bot API 4.3 support.

## 0.4.1 (28.05.2019)

- Fixed ChatMemberAdministrator parsing.

## 0.4.0 (07.05.2019)

- Added Bot API 4.2 support.
- Added `prelude` module.
- Added `ChatMember::is_member()` method.
- Added `UpdatesStream::should_retry()` method.
- Added `Api::download_file()` method.
- Added option to set custom base url for Telegram API (see `Config` struct).
- Added multipart support (you can upload files using multipart/form-data now).
- Added `UserId` type.
- You can (de)serialize callback data using `serde_json` (see `CallbackQuery` and `InlineKeyboardButton` types).
- `Api::new()` now takes a `Config` object.
- `Api::execute()` takes a method by value instead of reference.
- `Method::get_request(&self)` replaced by `Method::into_request(self)`
- `RequestBuilder` is now private.
- `WebhookServiceFactory` is now private.
- Removed use of `Mutex` in `WebhookService`.
- Removed `WebhookServiceFactoryError`.
- Removed `InlineKeyboardButton::new()` method as the user must use exactly one of the option fields.

## 0.3.0 (12.03.2019)

- Dispatcher moved to [carapax](https://github.com/tg-rs/carapax).
- Added `Update::get_chat_id()`, `Update::get_chat_username()` and `Update::get_user()` methods.
- Added `Message::is_edited()` and `Message::get_chat_username()` methods.
- Added `Message.commands` property.
- Added `UpdatesStreamOptions`.
- Removed `Api::create()` and `Api::with_proxy()` in favor of `Api::new()`.
- Removed `Api::get_updates()`, use `tgbot::handle_updates()` instead.
- `WebhookService` is public now.
- Respect `retry_after` parameter on polling error.

## 0.2.0 (27.02.2019)

- Migrated from curl to hyper.
- Added dispatcher.
- Added webhooks support.

## 0.1.0 (23.12.2018)

- First release.
