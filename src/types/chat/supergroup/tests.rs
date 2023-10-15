use crate::types::{
    tests::assert_json_eq,
    Chat,
    ChatLocation,
    ChatPermissions,
    ChatPhoto,
    Location,
    Message,
    MessageData,
    MessageSender,
    SupergroupChat,
    Text,
    User,
};

#[test]
fn supergroup() {
    let expected_struct = Chat::Supergroup(SupergroupChat {
        id: 1,
        title: String::from("Supergroup Chat"),
        username: Some(String::from("supergroup_chat")),
        photo: Some(ChatPhoto {
            small_file_id: String::from("small-file-id"),
            small_file_unique_id: String::from("small-file-unique-id"),
            big_file_id: String::from("big-file-id"),
            big_file_unique_id: String::from("big-file-unique-id"),
        }),
        description: Some(String::from("Description")),
        invite_link: Some(String::from("example.com/join/supergroup")),
        pinned_message: Some(Box::new(Message {
            id: 1,
            date: 0,
            edit_date: None,
            sender: MessageSender::User(User {
                id: 1,
                is_bot: false,
                first_name: String::from("User"),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
            }),
            chat: Chat::Supergroup(SupergroupChat {
                id: 1,
                title: String::from("Supergroup Chat"),
                username: Some(String::from("supergroup_chat")),
                photo: None,
                description: None,
                invite_link: None,
                pinned_message: None,
                sticker_set_name: None,
                can_set_sticker_set: None,
                permissions: None,
                slow_mode_delay: None,
                message_auto_delete_time: None,
                linked_chat_id: None,
                location: None,
                has_protected_content: None,
                join_to_send_messages: None,
                join_by_request: None,
                is_forum: None,
            }),
            author_signature: None,
            has_protected_content: false,
            forward: None,
            is_automatic_forward: false,
            is_topic_message: None,
            message_thread_id: None,
            reply_to: None,
            via_bot: None,
            media_group_id: None,
            reply_markup: None,
            data: MessageData::Text(Text {
                data: String::from("message-text"),
                entities: None,
            }),
        })),
        sticker_set_name: Some(String::from("Sticker Set")),
        can_set_sticker_set: Some(true),
        permissions: Some(ChatPermissions {
            can_send_messages: Some(true),
            can_send_media_messages: Some(true),
            can_send_polls: Some(true),
            can_send_other_messages: Some(true),
            can_add_web_page_previews: Some(true),
            can_change_info: Some(true),
            can_invite_users: Some(true),
            can_pin_messages: Some(true),
        }),
        slow_mode_delay: Some(10),
        message_auto_delete_time: Some(86400),
        linked_chat_id: Some(2),
        location: Some(ChatLocation {
            location: Location {
                longitude: 0.0,
                latitude: 1.0,
                horizontal_accuracy: None,
                live_period: None,
                heading: None,
                proximity_alert_radius: None,
            },
            address: String::from("Address"),
        }),
        has_protected_content: Some(true),
        join_to_send_messages: Some(true),
        join_by_request: Some(true),
        is_forum: Some(true),
    });
    assert_eq!(expected_struct.get_id(), 1);
    assert_eq!(expected_struct.get_username().unwrap(), "supergroup_chat");
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "supergroup",
            "title": "Supergroup Chat",
            "username": "supergroup_chat",
            "photo": {
                "small_file_id": "small-file-id",
                "small_file_unique_id": "small-file-unique-id",
                "big_file_id": "big-file-id",
                "big_file_unique_id": "big-file-unique-id",
            },
            "description": "Description",
            "invite_link": "example.com/join/supergroup",
            "sticker_set_name": "Sticker Set",
            "can_set_sticker_set": true,
            "slow_mode_delay": 10,
            "permissions": {
                "can_send_messages": true,
                "can_send_media_messages": true,
                "can_send_polls": true,
                "can_send_other_messages": true,
                "can_add_web_page_previews": true,
                "can_change_info": true,
                "can_invite_users": true,
                "can_pin_messages": true,
            },
            "pinned_message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "supergroup",
                    "title": "Supergroup Chat",
                    "username": "supergroup_chat"
                },
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "User"
                },
                "text": "message-text",
                "has_protected_content": false,
                "is_automatic_forward": false,
            },
            "linked_chat_id": 2,
            "location": {
                "location": {
                    "longitude": 0.0,
                    "latitude": 1.0
                },
                "address": "Address"
            },
            "has_protected_content": true,
            "message_auto_delete_time": 86400,
            "join_to_send_messages": true,
            "join_by_request": true,
            "is_forum": true
        }),
    );

    let expected_struct = Chat::Supergroup(SupergroupChat {
        id: 1,
        title: String::from("Supergroup Chat"),
        username: None,
        photo: None,
        description: None,
        invite_link: None,
        pinned_message: None,
        sticker_set_name: None,
        can_set_sticker_set: None,
        permissions: None,
        slow_mode_delay: None,
        message_auto_delete_time: None,
        linked_chat_id: None,
        location: None,
        has_protected_content: None,
        join_to_send_messages: None,
        join_by_request: None,
        is_forum: None,
    });
    assert_eq!(expected_struct.get_id(), 1);
    assert!(expected_struct.get_username().is_none());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "id": 1,
            "type": "supergroup",
            "title": "Supergroup Chat"
        }),
    );
}
