use crate::{
    tests::{assert_json_eq, assert_request_eq, ExpectedRequest},
    types::{
        BanChatMember,
        Chat,
        ChatInviteLink,
        ChatMember,
        ChatMemberAdministrator,
        ChatMemberCreator,
        ChatMemberKicked,
        ChatMemberRestricted,
        ChatMemberUpdated,
        ChatPermissions,
        GetChatAdministrators,
        GetChatMember,
        GetChatMemberCount,
        GroupChat,
        PromoteChatMember,
        RestrictChatMember,
        SetChatAdministratorCustomTitle,
        UnbanChatMember,
        User,
    },
};

#[test]
fn chat_member_admin() {
    let expected_struct = ChatMember::Administrator(ChatMemberAdministrator {
        user: User {
            id: 1,
            is_bot: false,
            first_name: String::from("John"),
            last_name: Some(String::from("Doe")),
            username: Some(String::from("john_doe")),
            language_code: Some(String::from("RU")),
        },
        custom_title: Some(String::from("Alpha")),
        is_anonymous: false,
        can_be_edited: true,
        can_change_info: false,
        can_post_messages: Some(true),
        can_edit_messages: Some(false),
        can_delete_messages: true,
        can_invite_users: false,
        can_restrict_members: true,
        can_pin_messages: Some(false),
        can_promote_members: true,
        can_manage_voice_chats: false,
        can_manage_chat: true,
    });
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "administrator",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John",
                "last_name": "Doe",
                "username": "john_doe",
                "language_code": "RU"
            },
            "custom_title": "Alpha",
            "is_anonymous": false,
            "can_be_edited": true,
            "can_change_info": false,
            "can_post_messages": true,
            "can_edit_messages": false,
            "can_delete_messages": true,
            "can_invite_users": false,
            "can_restrict_members": true,
            "can_pin_messages": false,
            "can_promote_members": true,
            "can_manage_voice_chats": false,
            "can_manage_chat": true
        }),
    );
    let expected_struct = ChatMember::Administrator(ChatMemberAdministrator {
        user: User {
            id: 1,
            is_bot: false,
            first_name: String::from("John"),
            last_name: None,
            username: None,
            language_code: None,
        },
        custom_title: None,
        is_anonymous: false,
        can_be_edited: true,
        can_change_info: false,
        can_post_messages: None,
        can_edit_messages: None,
        can_delete_messages: true,
        can_invite_users: false,
        can_restrict_members: true,
        can_pin_messages: None,
        can_promote_members: true,
        can_manage_voice_chats: false,
        can_manage_chat: true,
    });
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "administrator",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John",
            },
            "is_anonymous": false,
            "can_be_edited": true,
            "can_change_info": false,
            "can_delete_messages": true,
            "can_invite_users": false,
            "can_restrict_members": true,
            "can_promote_members": true,
            "can_manage_voice_chats": false,
            "can_manage_chat": true
        }),
    );
}

#[test]
fn chat_member_creator() {
    let expected_struct = ChatMember::Creator(ChatMemberCreator {
        user: User {
            id: 1,
            is_bot: false,
            first_name: String::from("John"),
            last_name: None,
            username: None,
            language_code: None,
        },
        is_anonymous: false,
        custom_title: Some(String::from("Alpha")),
    });

    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "creator",
            "is_anonymous": false,
            "custom_title": "Alpha",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John"
            }
        }),
    );
    let expected_struct = ChatMember::Creator(ChatMemberCreator {
        user: User {
            id: 1,
            is_bot: false,
            first_name: String::from("John"),
            last_name: None,
            username: None,
            language_code: None,
        },
        is_anonymous: false,
        custom_title: None,
    });
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "creator",
            "is_anonymous": false,
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John"
            }
        }),
    );
}

#[test]
fn chat_member_kicked() {
    let expected_struct = ChatMember::Kicked(ChatMemberKicked {
        user: User {
            id: 1,
            is_bot: false,
            first_name: String::from("John"),
            last_name: Some(String::from("Doe")),
            username: Some(String::from("john_doe")),
            language_code: Some(String::from("RU")),
        },
        until_date: 0,
    });
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(!expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "kicked",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John",
                "last_name": "Doe",
                "username": "john_doe",
                "language_code": "RU"
            },
            "until_date": 0
        }),
    );
}

#[test]
fn chat_member_left() {
    let expected_struct = ChatMember::Left(User {
        id: 1,
        is_bot: true,
        first_name: String::from("John"),
        last_name: None,
        username: None,
        language_code: None,
    });
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(!expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "left",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "John"
            }
        }),
    );
}

#[test]
fn chat_member() {
    let expected_struct = ChatMember::Member(User {
        id: 1,
        is_bot: false,
        first_name: String::from("John"),
        last_name: None,
        username: None,
        language_code: None,
    });
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "member",
            "user": {
                "id": 1,
                "is_bot": false,
                "first_name": "John"
            }
        }),
    );
}

#[test]
fn chat_member_restricted() {
    let expected_struct = ChatMember::Restricted(ChatMemberRestricted {
        user: User {
            id: 1,
            is_bot: true,
            first_name: String::from("John"),
            last_name: None,
            username: None,
            language_code: None,
        },
        until_date: 0,
        can_change_info: true,
        can_invite_users: false,
        can_send_polls: true,
        can_pin_messages: Some(false),
        can_send_messages: true,
        can_send_media_messages: false,
        can_send_other_messages: true,
        can_add_web_page_previews: false,
        is_member: true,
    });
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "restricted",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "John"
            },
            "until_date": 0,
            "can_change_info": true,
            "can_invite_users": false,
            "can_send_polls": true,
            "can_pin_messages": false,
            "can_send_messages": true,
            "can_send_media_messages": false,
            "can_send_other_messages": true,
            "can_add_web_page_previews": false,
            "is_member": true
        }),
    );
    let expected_struct = ChatMember::Restricted(ChatMemberRestricted {
        user: User {
            id: 1,
            is_bot: true,
            first_name: String::from("John"),
            last_name: None,
            username: None,
            language_code: None,
        },
        until_date: 0,
        can_change_info: true,
        can_invite_users: false,
        can_send_polls: true,
        can_pin_messages: None,
        can_send_messages: true,
        can_send_media_messages: false,
        can_send_other_messages: true,
        can_add_web_page_previews: false,
        is_member: false,
    });
    assert_eq!(expected_struct.get_user().id, 1);
    assert!(!expected_struct.is_member());
    assert_json_eq(
        expected_struct,
        serde_json::json!({
            "status": "restricted",
            "user": {
                "id": 1,
                "is_bot": true,
                "first_name": "John"
            },
            "until_date": 0,
            "can_change_info": true,
            "can_invite_users": false,
            "can_send_polls": true,
            "can_send_messages": true,
            "can_send_media_messages": false,
            "can_send_other_messages": true,
            "can_add_web_page_previews": false,
            "is_member": false
        }),
    );
}

#[test]
fn chat_member_updated() {
    assert_json_eq(
        ChatMemberUpdated {
            chat: Chat::Group(GroupChat {
                id: 1,
                title: String::from("group-title"),
                photo: None,
                invite_link: None,
                pinned_message: None,
                permissions: None,
                has_protected_content: None,
                message_auto_delete_time: None,
            }),
            from: User {
                id: 1,
                is_bot: true,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
            },
            date: 0,
            old_chat_member: ChatMember::Member(User {
                id: 2,
                is_bot: false,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
            }),
            new_chat_member: ChatMember::Kicked(ChatMemberKicked {
                user: User {
                    id: 2,
                    is_bot: true,
                    first_name: String::from("John"),
                    last_name: None,
                    username: None,
                    language_code: None,
                },
                until_date: 0,
            }),
            invite_link: Some(ChatInviteLink {
                invite_link: String::from("https://t.me/joinchat/o8oIBrbCI3U2OGJi"),
                creator: User {
                    id: 1,
                    is_bot: false,
                    first_name: String::from("John"),
                    last_name: None,
                    username: None,
                    language_code: None,
                },
                creates_join_request: false,
                is_primary: true,
                is_revoked: false,
                name: None,
                expire_date: None,
                member_limit: None,
                pending_join_request_count: None,
            }),
        },
        serde_json::json!({
            "chat": {
                "id": 1,
                "type": "group",
                "title": "group-title"
            },
            "from": {
                "id": 1,
                "is_bot": true,
                "first_name": "John"
            },
            "date": 0,
            "old_chat_member": {
                "status": "member",
                "user": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "John"
                }
            },
            "new_chat_member": {
                "status": "kicked",
                "user": {
                    "id": 2,
                    "is_bot": true,
                    "first_name": "John",
                },
                "until_date": 0
            },
            "invite_link": {
                "invite_link": "https://t.me/joinchat/o8oIBrbCI3U2OGJi",
                "creator": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "John"
                },
                "creates_join_request": false,
                "is_primary": true,
                "is_revoked": false
            }
        }),
    );
    assert_json_eq(
        ChatMemberUpdated {
            chat: Chat::Group(GroupChat {
                id: 1,
                title: String::from("group-title"),
                photo: None,
                invite_link: None,
                pinned_message: None,
                permissions: None,
                has_protected_content: None,
                message_auto_delete_time: None,
            }),
            from: User {
                id: 1,
                is_bot: true,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
            },
            date: 0,
            old_chat_member: ChatMember::Member(User {
                id: 2,
                is_bot: false,
                first_name: String::from("John"),
                last_name: None,
                username: None,
                language_code: None,
            }),
            new_chat_member: ChatMember::Kicked(ChatMemberKicked {
                user: User {
                    id: 2,
                    is_bot: true,
                    first_name: String::from("John"),
                    last_name: None,
                    username: None,
                    language_code: None,
                },
                until_date: 0,
            }),
            invite_link: None,
        },
        serde_json::json!({
            "chat": {
                "id": 1,
                "type": "group",
                "title": "group-title"
            },
            "from": {
                "id": 1,
                "is_bot": true,
                "first_name": "John"
            },
            "date": 0,
            "old_chat_member": {
                "status": "member",
                "user": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "John"
                }
            },
            "new_chat_member": {
                "status": "kicked",
                "user": {
                    "id": 2,
                    "is_bot": true,
                    "first_name": "John",
                },
                "until_date": 0
            }
        }),
    );
}

#[test]
fn ban_chat_member() {
    let method = BanChatMember::new(1, 2);
    assert_request_eq(
        ExpectedRequest::post_json(
            "banChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        method.clone(),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "banChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 3,
                "revoke_messages": true,
            }),
        ),
        method.until_date(3).revoke_messages(true),
    );
}

#[test]
fn get_chat_administrators() {
    assert_request_eq(
        ExpectedRequest::post_json(
            "getChatAdministrators",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        GetChatAdministrators::new(1),
    );
}

#[test]
fn get_chat_member() {
    assert_request_eq(
        ExpectedRequest::post_json(
            "getChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        GetChatMember::new(1, 2),
    );
}

#[test]
fn get_chat_members_count() {
    assert_request_eq(
        ExpectedRequest::post_json(
            "getChatMemberCount",
            serde_json::json!({
                "chat_id": 1
            }),
        ),
        GetChatMemberCount::new(1),
    );
}

#[test]
fn promote_chat_member() {
    assert_request_eq(
        ExpectedRequest::post_json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "is_anonymous": true,
                "can_change_info": true,
                "can_delete_messages": true,
                "can_edit_messages": true,
                "can_invite_users": true,
                "can_manage_chat": true,
                "can_manage_voice_chats": true,
                "can_pin_messages": true,
                "can_post_messages": true,
                "can_promote_members": true,
                "can_restrict_members": true
            }),
        ),
        PromoteChatMember::new(1, 2).promote_all(),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "is_anonymous": false,
                "can_change_info": false,
                "can_delete_messages": false,
                "can_edit_messages": false,
                "can_invite_users": false,
                "can_manage_chat": false,
                "can_manage_voice_chats": false,
                "can_pin_messages": false,
                "can_post_messages": false,
                "can_promote_members": false,
                "can_restrict_members": false
            }),
        ),
        PromoteChatMember::new(1, 2).demote_all(),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "is_anonymous": false,
                "can_change_info": true,
                "can_delete_messages": false,
                "can_edit_messages": true,
                "can_invite_users": true,
                "can_manage_chat": false,
                "can_manage_voice_chats": true,
                "can_pin_messages": true,
                "can_post_messages": false,
                "can_promote_members": false,
                "can_restrict_members": false
            }),
        ),
        PromoteChatMember::new(1, 2)
            .is_anonymous(false)
            .can_change_info(true)
            .can_edit_messages(true)
            .can_delete_messages(false)
            .can_invite_users(true)
            .can_manage_chat(false)
            .can_manage_voice_chats(true)
            .can_pin_messages(true)
            .can_post_messages(false)
            .can_promote_members(false)
            .can_restrict_members(false),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "promoteChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        PromoteChatMember::new(1, 2),
    );
}

#[test]
fn restrict_chat_member() {
    let method = RestrictChatMember::new(1, 2);
    assert_request_eq(
        ExpectedRequest::post_json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "permissions": {}
            }),
        ),
        method.clone(),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 100,
                "permissions": {}
            }),
        ),
        method.until_date(100),
    );
    let method = RestrictChatMember::new(1, 2).allow_all();
    assert_request_eq(
        ExpectedRequest::post_json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "permissions": {
                    "can_send_messages": true,
                    "can_send_media_messages": true,
                    "can_send_polls": true,
                    "can_send_other_messages": true,
                    "can_add_web_page_previews": true,
                    "can_change_info": true,
                    "can_invite_users": true,
                    "can_pin_messages": true,
                }
            }),
        ),
        method.clone(),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 100,
                "permissions": {
                    "can_send_messages": true,
                    "can_send_media_messages": true,
                    "can_send_polls": true,
                    "can_send_other_messages": true,
                    "can_add_web_page_previews": true,
                    "can_change_info": true,
                    "can_invite_users": true,
                    "can_pin_messages": true,
                }
            }),
        ),
        method.until_date(100),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "until_date": 100,
                "permissions": {
                    "can_send_messages": true,
                    "can_send_media_messages": false,
                    "can_send_other_messages": true,
                    "can_add_web_page_previews": false
                }
            }),
        ),
        RestrictChatMember::new(1, 2)
            .with_permissions(
                ChatPermissions::default()
                    .with_send_messages(true)
                    .with_send_media_messages(false)
                    .with_send_other_messages(true)
                    .with_add_web_page_previews(false),
            )
            .until_date(100),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "restrictChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "permissions": {}
            }),
        ),
        RestrictChatMember::new(1, 2),
    );
}

#[test]
fn set_chat_administrator_custom_title() {
    assert_request_eq(
        ExpectedRequest::post_json(
            "setChatAdministratorCustomTitle",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 1,
                "custom_title": "Alpha"
            }),
        ),
        SetChatAdministratorCustomTitle::new(1, 1, "Alpha"),
    );
}

#[test]
fn unban_chat_member() {
    let method = UnbanChatMember::new(1, 2);
    assert_request_eq(
        ExpectedRequest::post_json(
            "unbanChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2
            }),
        ),
        method.clone(),
    );
    assert_request_eq(
        ExpectedRequest::post_json(
            "unbanChatMember",
            serde_json::json!({
                "chat_id": 1,
                "user_id": 2,
                "only_if_banned": true
            }),
        ),
        method.only_if_banned(true),
    );
}
