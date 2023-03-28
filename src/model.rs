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
type InternalChannels = Arc<RwLock<HashMap<ChannelId, InternalChannel>>>;
type InternalUsers = Arc<RwLock<HashMap<UserId, InternalUser>>>;
type Systems = Arc<RwLock<HashMap<Uuid, System>>>;
type ChannelUsers = Arc<RwLock<HashMap<ChannelId, Vec<UserId>>>>;
type UserChannels = Arc<RwLock<HashMap<String, Vec<Channel>>>>;

#[derive(Clone, Default)]
pub struct AppState {
    pub messages: ChannelMessages,
    pub internal_channels: InternalChannels,
    pub internal_users: InternalUsers,
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
    size: i32
}

impl Channel {
    pub fn new (id: String) -> Channel {
        Channel {
            id,
            name: "".to_string(),
            icon: "".to_string(),
            description: "".to_string(),
            visibility: true,
            size: 0
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn inc_size(&mut self) {
        self.size += 1;
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct InternalChannel {
    id: String,
    model: Channel,
    owner_id: String
}

impl InternalChannel {
    pub fn new (id: String, model: Channel, owner_id: String) -> InternalChannel {
        InternalChannel {
            id,
            model,
            owner_id
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_model(&self) -> Channel {
        self.model.clone()
    }

    pub fn set_model(&mut self, model: Channel) {
        self.model = model;
    }

    pub fn get_owner_id(&self) -> String {
        self.owner_id.clone()
    }

    pub fn inc_size(&mut self) {
        self.model.inc_size();
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
    from_system: String,
    avatar: String,
}

impl User {
    pub fn new (id: String) -> User {
        User {
            id,
            name: "".to_string(),
            status: "".to_string(),
            from_system: "own system".to_string(),
            avatar: "".to_string()
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InternalUser {
    id: String,
    model: User,
    owned_channels: HashSet<String>,
    blocked_users: HashSet<String>
}

impl InternalUser {
    pub fn new(id: String, model: User, owned_channels: HashSet<String>, blocked_users: HashSet<String>) -> InternalUser {
        InternalUser {
            id,
            model,
            owned_channels,
            blocked_users
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_model(&self) -> User {
        self.model.clone()
    }

    pub fn set_model(&mut self, model: User) {
        self.model = model;
    }

    pub fn add_owned_channel(&mut self, channel_id: String) {
        self.owned_channels.insert(channel_id);
    }

    pub fn add_blocked_user(&mut self, user_id: String) {
        self.blocked_users.insert(user_id);
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