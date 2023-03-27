use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use std::{
    collections::HashMap,
    collections::HashSet,
    sync::{Arc, RwLock},
};

type Uuid = String;
type ChannelId = String;
type UserId = String;
type ChannelMessages = Arc<RwLock<HashMap<ChannelId, Vec<Message>>>>;
type Channels = Arc<RwLock<HashMap<ChannelId, Channel>>>;
type Users = Arc<RwLock<HashMap<UserId, User>>>;
type Systems = Arc<RwLock<HashMap<String, Vec<System>>>>;
type ChannelUsers = Arc<RwLock<HashMap<ChannelId, Vec<UserId>>>>;
type UserChannels = Arc<RwLock<HashMap<String, Vec<Channel>>>>;

#[derive(Clone, Default)]
pub struct AppState {
    pub messages: ChannelMessages,
    pub channels: Channels,
    pub users: Users,
    pub user_channels: UserChannels,
    pub channel_users: ChannelUsers,
    pub systems: Systems,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Payload {
    html: String,
    text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Channel {
    id: String,
    name: Option<String>,
    icon: Option<String>,
    description: Option<String>,
    visibility: bool,
    size: i32,
    owner_id: String
}

impl Channel {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_owner_id(&self) -> String {
        self.owner_id.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    id: Uuid,
    timestamp: String,
    message: Payload,
    from_user: Uuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    id: String,
    name: String,
    status: String,
    from_system: Uuid,
    avatar: String,
    owned_channels: HashSet<String>
}

impl User {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn add_owned_channel(&mut self, channel_id: String) {
        self.owned_channels.insert(channel_id);
    }
}

#[derive(Serialize, Deserialize)]
pub struct System {
    id: Uuid,
    address: String,
    last_sync: NaiveDateTime,
    status: String,
}
