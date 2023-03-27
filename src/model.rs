use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
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
type Systems = Arc<RwLock<HashMap<Uuid, System>>>;
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
    visibility: bool,
    size: i32,
    owner_id: String
}

impl Channel {
    pub fn new (id: String, owner_id: String) -> Channel {
        Channel {
            id,
            name: "".to_string(),
            icon: "".to_string(),
            description: "".to_string(),
            visibility: true,
            size: 0,
            owner_id
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_owner_id(&self) -> String {
        self.owner_id.clone()
    }

    pub fn inc_size(&mut self) {
        self.size += 1;
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    id: Uuid,
    timestamp: String,
    message: Payload,
    from_user: Uuid,
}

impl Message {
    pub fn get_owner_id(&self) -> String {
        self.from_user.clone()
    }
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

    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }
}