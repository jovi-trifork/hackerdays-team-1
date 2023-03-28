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
type ChannelMessagesSync = Arc<RwLock<ChannelMessages>>;
type InternalChannelsSync = Arc<RwLock<InternalChannels>>;
type InternalUsersSync = Arc<RwLock<InternalUsers>>;
type SystemsSync = Arc<RwLock<Systems>>;
type ChannelUsersSync = Arc<RwLock<ChannelUsers>>;
type UserChannelsSync = Arc<RwLock<UserChannels>>;

type ChannelMessages = HashMap<ChannelId, Vec<Message>>;
type InternalChannels = HashMap<ChannelId, InternalChannel>;
type InternalUsers = HashMap<UserId, InternalUser>;
type Systems = HashMap<String, System>;
type ChannelUsers = HashMap<ChannelId, Vec<UserId>>;
type UserChannels = HashMap<String, Vec<Channel>>;

pub type AppState = Arc<AppStateInternal>;

#[derive(Clone, Default)]
pub struct AppStateInternal {
    pub messages: ChannelMessagesSync,
    pub internal_channels: InternalChannelsSync,
    pub internal_users: InternalUsersSync,
    pub user_channels: UserChannelsSync,
    pub channel_users: ChannelUsersSync,
    pub systems: SystemsSync,
}

#[derive(Default, Debug)]
pub struct ServerSyncAppState {
    pub messages: ChannelMessages,
    pub internal_channels: InternalChannels,
    pub internal_users: InternalUsers,
    pub systems: Systems,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Payload {
    html: String,
    text: String,
}


pub type GetChannelsResponse = Vec<Channel>;
#[derive(Serialize, Deserialize, Clone, Debug)]
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

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn inc_size(&mut self) {
        self.size += 1;
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
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

pub type GetMessagesResponse = Vec<Message>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    id: String,
    timestamp: String,
    message: Payload,
    from_user: String,
}

impl Message {
    pub fn get_owner_id(&self) -> String {
        self.from_user.clone()
    }
}

pub type GetUsersResponse = Vec<User>;

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct System {
    id: String,
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

    pub fn get_address(&self) -> &str {
        &self.address
    }
}