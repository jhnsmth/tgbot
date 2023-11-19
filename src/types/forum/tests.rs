use crate::{
    api::{assert_payload_eq, Payload},
    types::{
        tests::assert_json_eq,
        CloseForumTopic,
        CloseGeneralForumTopic,
        CreateForumTopic,
        DeleteForumTopic,
        EditForumTopic,
        EditGeneralForumTopic,
        ForumTopic,
        ForumTopicIconColor,
        GetForumTopicIconStickers,
        HideGeneralForumTopic,
        Integer,
        ReopenForumTopic,
        ReopenGeneralForumTopic,
        UnhideGeneralForumTopic,
        UnpinAllForumTopicMessages,
        UnpinAllGeneralForumTopicMessages,
    },
};

#[test]
fn forum_topic() {
    let expected_struct = ForumTopic::new(ForumTopicIconColor::Bittersweet, 1, "topic-name");
    assert_json_eq(
        expected_struct.clone(),
        serde_json::json!({
            "message_thread_id": 1,
            "name": "topic-name",
            "icon_color": 16478047,
        }),
    );
    assert_json_eq(
        expected_struct.with_icon_custom_emoji_id("emoji-id"),
        serde_json::json!({
            "message_thread_id": 1,
            "name": "topic-name",
            "icon_color": 16478047,
            "icon_custom_emoji_id": "emoji-id",
        }),
    );
}

#[test]
fn forum_topic_icon_color() {
    for (expected_struct, expected_value) in [
        (ForumTopicIconColor::BakerMillerPink, 16749490),
        (ForumTopicIconColor::Bittersweet, 16478047),
        (ForumTopicIconColor::BrightLavender, 13338331),
        (ForumTopicIconColor::Jasmine, 16766590),
        (ForumTopicIconColor::LightGreen, 9367192),
        (ForumTopicIconColor::VeryLightAzure, 7322096),
        (ForumTopicIconColor::Unknown(0), 0),
    ] {
        assert_eq!(Integer::from(expected_struct), expected_value);
        assert_eq!(ForumTopicIconColor::from(expected_value), expected_struct);
        assert_json_eq(expected_struct, serde_json::json!(expected_value));
    }
}

#[test]
fn close_forum_topic() {
    assert_payload_eq(
        Payload::json(
            "closeForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1
            }),
        ),
        CloseForumTopic::new(1, 1),
    );
}

#[test]
fn close_general_forum_topic() {
    assert_payload_eq(
        Payload::json(
            "closeGeneralForumTopic",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        CloseGeneralForumTopic::new(1),
    );
}

#[test]
fn create_forum_topic() {
    let method = CreateForumTopic::new(1, "topic-name");
    assert_payload_eq(
        Payload::json(
            "createForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "name": "topic-name"
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "createForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "name": "topic-name",
                "icon_color": 13338331,
                "icon_custom_emoji_id": "emoji-id"
            }),
        ),
        method
            .with_icon_color(ForumTopicIconColor::BrightLavender)
            .with_icon_custom_emoji_id("emoji-id"),
    );
}

#[test]
fn delete_forum_topic() {
    assert_payload_eq(
        Payload::json(
            "deleteForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1
            }),
        ),
        DeleteForumTopic::new(1, 1),
    );
}

#[test]
fn edit_forum_topic() {
    let method = EditForumTopic::new(1, 1);
    assert_payload_eq(
        Payload::json(
            "editForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1
            }),
        ),
        method.clone(),
    );
    assert_payload_eq(
        Payload::json(
            "editForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1,
                "name": "topic-name",
                "icon_custom_emoji_id": "emoji-id"
            }),
        ),
        method.with_icon_custom_emoji_id("emoji-id").with_name("topic-name"),
    );
}

#[test]
fn edit_general_forum_topic() {
    assert_payload_eq(
        Payload::json(
            "editGeneralForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "name": "new-name"
            }),
        ),
        EditGeneralForumTopic::new(1, "new-name"),
    );
}

#[test]
fn get_forum_topic_icon_stickers() {
    assert_payload_eq(Payload::empty("getForumTopicIconStickers"), GetForumTopicIconStickers);
}

#[test]
fn hide_general_forum_topic() {
    assert_payload_eq(
        Payload::json(
            "hideGeneralForumTopic",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        HideGeneralForumTopic::new(1),
    );
}

#[test]
fn reopen_forum_topic() {
    assert_payload_eq(
        Payload::json(
            "reopenForumTopic",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1
            }),
        ),
        ReopenForumTopic::new(1, 1),
    );
}

#[test]
fn reopen_general_forum_topic() {
    assert_payload_eq(
        Payload::json(
            "reopenGeneralForumTopic",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        ReopenGeneralForumTopic::new(1),
    );
}

#[test]
fn unhide_general_forum_topic() {
    assert_payload_eq(
        Payload::json(
            "unhideGeneralForumTopic",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        UnhideGeneralForumTopic::new(1),
    );
}

#[test]
fn unpin_all_forum_topic_messages() {
    assert_payload_eq(
        Payload::json(
            "unpinAllForumTopicMessages",
            serde_json::json!({
                "chat_id": 1,
                "message_thread_id": 1
            }),
        ),
        UnpinAllForumTopicMessages::new(1, 1),
    );
}

#[test]
fn unpin_all_general_forum_topic_messages() {
    assert_payload_eq(
        Payload::json(
            "unpinAllGeneralForumTopicMessages",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        UnpinAllGeneralForumTopicMessages::new(1),
    );
}
