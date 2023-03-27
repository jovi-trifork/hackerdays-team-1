use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

<<<<<<< HEAD
type Uuid = String;
type ChannelId = String;
type UserId = String;
type ChannelMessages = Arc<RwLock<HashMap<ChannelId, Vec<Message>>>>;
type Channels = Arc<RwLock<HashMap<ChannelId, Channel>>>;
type Users = Arc<RwLock<HashMap<UserId, User>>>;
type Systems = Arc<RwLock<HashMap<String, Vec<System>>>>;
type ChannelUsers = Arc<RwLock<HashMap<ChannelId, Vec<UserId>>>>;
=======
type ChannelMessages = Arc<RwLock<HashMap<String, Vec<Message>>>>;
type UserChannels = Arc<RwLock<HashMap<String, Vec<Channel>>>>;
type Channels = Arc<RwLock<HashMap<String, Channel>>>;
type Users = Arc<RwLock<HashMap<String, User>>>;
type ChannelUsers = Arc<RwLock<HashMap<String, Vec<String>>>>;
type Systems = Arc<RwLock<HashMap<String, Vec<System>>>>;
>>>>>>> 465373efddd940d2fab84034ace25158743c4382

#[derive(Clone, Default)]
pub struct AppState {
    pub messages: ChannelMessages,
    pub channels: Channels,
    pub users: Users,
<<<<<<< HEAD
    pub systems: Systems,
    pub channel_users: ChannelUsers,
=======
    pub user_channels: UserChannels,
    pub channel_users: ChannelUsers,
    pub systems: Systems,
>>>>>>> 465373efddd940d2fab84034ace25158743c4382
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

#[derive(Serialize, Deserialize)]
pub struct System {
    id: Uuid,
    address: String,
    last_sync: NaiveDateTime,
    status: String,
}
