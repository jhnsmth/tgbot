use crate::{
    api::{assert_payload_eq, Form, FormValue, Payload},
    types::{tests::assert_json_eq, ForceReply, InputFile, ParseMode, SendVoice, TextEntity, Voice},
};

#[test]
fn voice() {
    assert_json_eq(
        Voice::new(500, "file-id", "file-unique-id")
            .with_mime_type("audio/ogg")
            .with_file_size(40960),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "duration": 500,
            "mime_type": "audio/ogg",
            "file_size": 40960
        }),
    );
    assert_json_eq(
        Voice::new(500, "file-id", "file-unique-id"),
        serde_json::json!({
            "file_id": "file-id",
            "file_unique_id": "file-unique-id",
            "duration": 500,
        }),
    );
}

#[test]
fn send_voice() {
    assert_payload_eq(
        Payload::form(
            "sendVoice",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("voice", InputFile::file_id("file-id").into()),
            ]),
        ),
        SendVoice::new(1, InputFile::file_id("file-id")),
    );
    assert_payload_eq(
        Payload::form(
            "sendVoice",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("voice", InputFile::file_id("file-id").into()),
                ("caption", "Caption".into()),
                ("parse_mode", ParseMode::Markdown.into()),
                ("duration", 100.into()),
                ("disable_notification", true.into()),
                ("protect_content", true.into()),
                ("reply_to_message_id", 1.into()),
                ("allow_sending_without_reply", true.into()),
                (
                    "reply_markup",
                    serde_json::to_string(&ForceReply::new(true)).unwrap().into(),
                ),
                ("message_thread_id", 1.into()),
            ]),
        ),
        SendVoice::new(1, InputFile::file_id("file-id"))
            .with_allow_sending_without_reply(true)
            .with_caption("Caption")
            .with_disable_notification(true)
            .with_duration(100)
            .with_message_thread_id(1)
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_protect_content(true)
            .with_reply_markup(ForceReply::new(true))
            .unwrap()
            .with_reply_to_message_id(1),
    );
}

#[test]
fn send_voice_entities_vs_parse_mode() {
    assert_payload_eq(
        Payload::form(
            "sendVoice",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("voice", InputFile::file_id("file-id").into()),
                ("parse_mode", "Markdown".into()),
            ]),
        ),
        SendVoice::new(1, InputFile::file_id("file-id"))
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap()
            .with_caption_parse_mode(ParseMode::Markdown),
    );
    assert_payload_eq(
        Payload::form(
            "sendVoice",
            Form::from([
                ("chat_id", FormValue::from(1)),
                ("voice", InputFile::file_id("file-id").into()),
                ("caption_entities", r#"[{"offset":0,"length":10,"type":"bold"}]"#.into()),
            ]),
        ),
        SendVoice::new(1, InputFile::file_id("file-id"))
            .with_caption_parse_mode(ParseMode::Markdown)
            .with_caption_entities(vec![TextEntity::bold(0..10)])
            .unwrap(),
    );
}
