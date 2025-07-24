pub use crate::types::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ChatResponse {
    ActiveUser {
        user: User,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ChatError {
        #[serde(skip_serializing_if = "Option::is_none")]
        user_: Option<User>,
        chat_error: ChatError,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ChatCmdError {
        #[serde(skip_serializing_if = "Option::is_none")]
        user_: Option<User>,
        chat_error: ChatError,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ChatRunning {
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ChatStarted {
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ChatStopped {
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    Chats {
        // user: User,
        chats: Vec<Chat>,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ContactConnected {
        contact: Contact,
        user: User,
        #[serde(skip_serializing_if = "Option::is_none")]
        user_custom_profile: Option<Profile>,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    GroupMembers {
        user: User,
        group: Group,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    NewChatItems {
        user: User,
        chat_items: Vec<AChatItem>,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ReceivedGroupInvitation {
        user: User,
        group_info: GroupInfo,
        contact: Contact,
        member_role: GroupMemberRole,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    UserContactLinkCreated {
        user: User,
        conn_link_contact: ConnLinkContact,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    UsersList {
        users: Vec<UserInfo>,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    #[serde(untagged)]
    ContactLink {
        user: User,
        contact_link: ContactLink,
        // content of this should be "userContactLink", but idk how to check it here
        #[serde(rename = "type")]
        _type: String,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        unknown_fields: HashMap<String, JsonValue>,
    },
    #[serde(untagged)]
    ContactRequest { contact_request: UserContactRequest },
    #[serde(untagged)]
    Unknown(JsonValue),
}
