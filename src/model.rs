use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

type Uuid = String;
type ChannelId = String;
type UserId = String;
type ChannelMessages = Arc<RwLock<HashMap<ChannelId, Vec<Message>>>>;
type Channels = Arc<RwLock<HashMap<ChannelId, Channel>>>;
type Users = Arc<RwLock<HashMap<UserId, User>>>;
type Systems = Arc<RwLock<HashMap<String, System>>>;
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
    name: String,
    icon: String,
    description: String,
    visibiliy: bool,
    size: i32,
}

impl Channel {
    pub fn get_id(&self) -> String {
        self.id.clone()
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
    id: Uuid,
    name: String,
    status: String,
    from_system: Uuid,
    avatar: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct System {
    id: Uuid,
    address: String,
    last_sync: DateTime<Utc>,
    status: String,
}

impl System {
    pub fn new(address: String) -> Self {
        System {
            id: uuid::Uuid::new_v4().to_string(),
            address,
            last_sync: chrono::Utc::now(),
            status: "".to_string(),
        }
    }
}