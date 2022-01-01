use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, Integer},
};
use serde::Serialize;

/// Promote or demote a user in a supergroup or a channel
///
/// The bot must be an administrator in the chat
/// for this to work and must have the appropriate admin rights
/// Pass False for all boolean parameters to demote a user
#[derive(Clone, Debug, Serialize)]
pub struct PromoteChatMember {
    chat_id: ChatId,
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_delete_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_manage_voice_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_promote_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_restrict_members: Option<bool>,
}

impl PromoteChatMember {
    /// Creates a new PromoteChatMember with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        PromoteChatMember {
            chat_id: chat_id.into(),
            user_id,
            is_anonymous: None,
            can_change_info: None,
            can_delete_messages: None,
            can_edit_messages: None,
            can_invite_users: None,
            can_manage_chat: None,
            can_manage_voice_chats: None,
            can_pin_messages: None,
            can_post_messages: None,
            can_promote_members: None,
            can_restrict_members: None,
        }
    }

    /// Promote all privileges
    pub fn promote_all(mut self) -> Self {
        self.is_anonymous = Some(true);
        self.can_change_info = Some(true);
        self.can_delete_messages = Some(true);
        self.can_edit_messages = Some(true);
        self.can_invite_users = Some(true);
        self.can_manage_chat = Some(true);
        self.can_manage_voice_chats = Some(true);
        self.can_pin_messages = Some(true);
        self.can_post_messages = Some(true);
        self.can_promote_members = Some(true);
        self.can_restrict_members = Some(true);
        self
    }

    /// Demote all privileges
    pub fn demote_all(mut self) -> Self {
        self.is_anonymous = Some(false);
        self.can_change_info = Some(false);
        self.can_delete_messages = Some(false);
        self.can_edit_messages = Some(false);
        self.can_invite_users = Some(false);
        self.can_manage_chat = Some(false);
        self.can_manage_voice_chats = Some(false);
        self.can_pin_messages = Some(false);
        self.can_post_messages = Some(false);
        self.can_promote_members = Some(false);
        self.can_restrict_members = Some(false);
        self
    }

    /// Administrator's presence in the chat is hidden if true
    #[allow(clippy::wrong_self_convention)]
    pub fn is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }

    /// Administrator can change chat title, photo and other settings
    pub fn can_change_info(mut self, can_change_info: bool) -> Self {
        self.can_change_info = Some(can_change_info);
        self
    }

    /// Administrator can delete messages of other users
    pub fn can_delete_messages(mut self, can_delete_messages: bool) -> Self {
        self.can_delete_messages = Some(can_delete_messages);
        self
    }

    /// Administrator can edit messages of other users and can pin messages, channels only
    pub fn can_edit_messages(mut self, can_edit_messages: bool) -> Self {
        self.can_edit_messages = Some(can_edit_messages);
        self
    }

    /// Administrator can invite new users to the chat
    pub fn can_invite_users(mut self, can_invite_users: bool) -> Self {
        self.can_invite_users = Some(can_invite_users);
        self
    }

    /// Administrator can access the chat event log, chat statistics,
    /// message statistics in channels, see channel members,
    /// see anonymous administrators in supergroups and ignore slow mode
    ///
    /// Implied by any other administrator privilege
    pub fn can_manage_chat(mut self, can_manage_chat: bool) -> Self {
        self.can_manage_chat = Some(can_manage_chat);
        self
    }

    /// Administrator can manage voice chats, supergroups only
    pub fn can_manage_voice_chats(mut self, can_manage_voice_chats: bool) -> Self {
        self.can_manage_voice_chats = Some(can_manage_voice_chats);
        self
    }

    /// Administrator can pin messages, supergroups only
    pub fn can_pin_messages(mut self, can_pin_messages: bool) -> Self {
        self.can_pin_messages = Some(can_pin_messages);
        self
    }

    /// Administrator can create channel posts, channels only
    pub fn can_post_messages(mut self, can_post_messages: bool) -> Self {
        self.can_post_messages = Some(can_post_messages);
        self
    }

    /// Administrator can add new administrators with a subset of his own privileges or demote administrators
    /// that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    pub fn can_promote_members(mut self, can_promote_members: bool) -> Self {
        self.can_promote_members = Some(can_promote_members);
        self
    }

    /// Administrator can restrict, ban or unban chat members
    pub fn can_restrict_members(mut self, can_restrict_members: bool) -> Self {
        self.can_restrict_members = Some(can_restrict_members);
        self
    }
}

impl Method for PromoteChatMember {
    type Response = bool;

    fn into_request(self) -> Request {
        Request::json("promoteChatMember", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn promote_chat_member_promote_all() {
        let request = PromoteChatMember::new(1, 2).promote_all().into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/promoteChatMember"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
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
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }

    #[test]
    fn promote_chat_member_demote_all() {
        let request = PromoteChatMember::new(1, 2).demote_all().into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/promoteChatMember"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
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
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }

    #[test]
    fn promote_chat_member_custom() {
        let request = PromoteChatMember::new(1, 2)
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
            .can_restrict_members(false)
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/promoteChatMember"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
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
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }

    #[test]
    fn promote_chat_member_empty() {
        let request = PromoteChatMember::new(1, 2).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/promoteChatMember"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "chat_id": 1,
                    "user_id": 2
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }
}
