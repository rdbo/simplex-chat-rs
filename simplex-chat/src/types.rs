use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocalProfile {
    pub profile_id: u64,
    pub display_name: String,
    pub full_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_link: Option<String>,
    pub local_alias: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: u64,
    pub agent_user_id: String,
    pub user_contact_id: u64,
    pub local_display_name: String,
    pub profile: LocalProfile,
    pub active_user: bool,
    // view_pwd_hash: String, // Declared in the typescript API, but not sent by server
    pub show_ntfs: bool,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user: User,
    pub unread_count: u64,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub contact_id: u64,
    pub local_display_name: String,
    // profile: Profile,
    pub active_conn: Option<Connection>,
    pub contact_status: ContactStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_group: Option<u64>,
    // created_at: Date,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ContactStatus {
    Active,
    Deleted,
    DeletedByUser,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[serde(tag = "type")]
pub enum ChatInfo {
    Direct {
        contact: Contact,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    Group {
        group_info: GroupInfo,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ContactRequest {
        contact_request: UserContactRequest,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    #[serde(untagged)]
    Unknown(JsonValue),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub group_info: GroupInfo,
    pub members: Vec<GroupMember>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupInfo {
    pub group_id: u64,
    pub local_display_name: String,
    pub group_profile: GroupProfile,
    pub membership: GroupMember,
    // pub created_at: Date, // TODO: Pick date type
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupProfile {
    pub display_name: String,
    pub full_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    pub group_member_id: u64,
    pub member_id: String,
    pub member_role: GroupMemberRole,
    pub local_display_name: String,
    pub member_profile: Profile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_contact_id: Option<u64>,
    pub active_conn: Option<Connection>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum GroupMemberRole {
    // role used for unknown profiles in group
    // (e.g. forwarded messages from member no longer in the group)
    Author,
    Observer,
    Member,
    Admin,
    Owner,
    #[serde(untagged)]
    Unknown(JsonValue),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    display_name: String,
    full_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_link: Option<String>,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AChatItem {
    pub chat_info: ChatInfo,
    pub chat_item: ChatItem,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatItem {
    pub chat_dir: Direction,
    pub meta: Meta,
    // pub content: CIContent,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub formatted_text: Option<Vec<FormattedText>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub quoted_item: Option<CIQuote>
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[serde(tag = "type")]
pub enum Direction {
    DirectSnd,
    DirectRcv,
    GroupSnd,
    GroupRcv {
        group_member: GroupMember,
    },
    #[serde(untagged)]
    Unknown(JsonValue),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub item_id: u64,
    pub item_ts: DateTime<Utc>,
    pub item_text: String,
    // item_status: CIStatus,
    // created_at: Date,
    // pub item_deleted: bool,
    pub item_edited: bool,
    pub editable: bool,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub chat_info: ChatInfo,
    // chat_items: Vec<ChatItem>,
    // chat_stats: ChatStats,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactLink {
    pub address_settings: AddressSettings,
    pub conn_link_contact: ConnLinkContact,
    pub short_link_data_set: bool,
    pub user_contact_link_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressSettings {
    pub auto_accept: AutoAccept,
    pub business_address: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnLinkContact {
    pub conn_full_link: String,
    pub conn_short_link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoAccept {
    pub accept_incognito: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(
    tag = "type",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum MsgContent {
    Text {
        text: String,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    Link {
        text: String,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    Image {
        image: String, // Base64 string
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    File {
        text: String,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    Unknown(JsonValue),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[serde(tag = "type")]
pub enum ChatError {
    Error {
        error_type: ChatErrorType,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ErrorAgent {
        agent_error: JsonValue,
        #[serde(flatten)]
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        _unknown_fields: HashMap<String, JsonValue>,
    },
    ErrorStore {
        store_error: JsonValue,
    },
    #[serde(untagged)]
    Unknown(JsonValue),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ChatErrorType {
    NoActiveUser,
    ActiveUserExists,
    #[serde(untagged)]
    Unknown(JsonValue),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub conn_id: u64,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub _unknown_fields: HashMap<String, JsonValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserContactRequest {
    pub contact_request_id: u64,
    pub local_display_name: String,
    pub profile: Profile,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComposedMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quoted_item_id: Option<u64>,
    pub msg_content: MsgContent,
    pub mentions: Mentions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mentions {}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ChatType {
    Direct,
    Group,
    ContactRequest,
}

impl ToString for ChatType {
    fn to_string(&self) -> String {
        match self {
            Self::Direct => "@",
            Self::Group => "#",
            Self::ContactRequest => "<@",
        }
        .to_owned()
    }
}
