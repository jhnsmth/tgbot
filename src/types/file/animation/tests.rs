use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{tests::assert_json_eq, Animation, ForceReply, InputFile, ParseMode, PhotoSize, SendAnimation, TextEntity},
};

#[test]
fn animation() {
    assert_json_eq(
        Animation {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            width: 200,
            height: 200,
            duration: 243,
            thumbnail: Some(PhotoSize {
                file_id: String::from("thumb-file-id"),
                file_unique_id: String::from("thumb-file-unique-id"),
                width: 24,
                height: 24,
                file_size: Some(1024),
            }),
            file_name: Some(String::from("File Name")),
            mime_type: Some(String::from("image/gif")),
            file_size: Some(20480),
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "width": 200,
            "height": 200,
            "duration": 243,
            "thumbnail": {
                "file_id": "thumb-file-id",
                "file_unique_id": "thumb-file-unique-id",
                "width": 24,
                "height": 24,
                "file_size": 1024
            },
            "file_name": "File Name",
            "mime_type": "image/gif",
            "file_size": 20480
        }),
    );
    assert_json_eq(
        Animation {
            file_id: String::from("file-id"),
            file_unique_id: String::from("file-unique-id"),
            width: 200,
            height: 200,
            duration: 30,
            thumbnail: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        },
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "width": 200,
            "height": 200,
            "duration": 30,
        }),
    );
}

#[test]
fn send_animation() {
    assert_payload_eq(
        Payload::form(
            "sendAnimation",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("animation", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendAnimation::new(1, InputFile::file_id("file-id")),
    );
    assert_payload_eq(
        Payload::form(
            "sendAnimation",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("animation", InputFile::file_id("file-id").into()),
                ("duration", 100.into()),
                ("width", 200.into()),
                ("height", 300.into()),
                ("thumbnail", InputFile::file_id("thumb-file-id").into()),
                ("caption", "Caption".into()),
                ("parse_mode", ParseMode::Markdown.into()),
                ("disable_notification", true.into()),
                ("protect_content", true.into()),
                ("reply_to_message_id", 1.into()),
                ("allow_sending_without_reply", true.into()),
                (
                    "reply_markup",
                    serde_json::to_string(&ForceReply::new(true)).unwrap().into(),
                ),
                ("message_thread_id", 1.into()),
                ("has_spoiler", true.into()),
            ]),
        ),
        SendAnimation::new(1, InputFile::file_id("file-id"))
            .duration(100)
            .width(200)
            .height(300)
            .thumbnail(InputFile::file_id("thumb-file-id"))
            .caption("Caption")
            .parse_mode(ParseMode::Markdown)
            .disable_notification(true)
            .protect_content(true)
            .reply_to_message_id(1)
            .allow_sending_without_reply(true)
            .reply_markup(ForceReply::new(true))
            .unwrap()
            .message_thread_id(1)
            .has_spoiler(true),
    );
}

#[test]
fn send_animation_caption_entities_vs_parse_mode() {
    assert_payload_eq(
        Payload::form(
            "sendAnimation",
            Form::from([
                ("chat_id", 1.into()),
                ("animation", InputFile::file_id("file-id").into()),
                ("caption_entities", r#"[{"offset":0,"length":10,"type":"bold"}]"#.into()),
            ]),
        ),
        SendAnimation::new(1, InputFile::file_id("file-id"))
            .parse_mode(ParseMode::Markdown)
            .caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap(),
    );
    assert_payload_eq(
        Payload::form(
            "sendAnimation",
            Form::from([
                ("chat_id", 1.into()),
                ("animation", InputFile::file_id("file-id").into()),
                ("parse_mode", "Markdown".into()),
            ]),
        ),
        SendAnimation::new(1, InputFile::file_id("file-id"))
            .caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap()
            .parse_mode(ParseMode::Markdown),
    );
}
