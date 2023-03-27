use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

type ChannelMessages = Arc<RwLock<HashMap<String, Vec<Message>>>>;
type Channels = Arc<RwLock<HashMap<String, Vec<Channel>>>>;
type Users = Arc<RwLock<HashMap<String, Vec<User>>>>;
type ChannelUsers = Arc<RwLock<HashMap<String, Vec<String>>>>;


#[derive(Clone, Default)]
pub struct AppState {
    pub messages: ChannelMessages,
    pub channels: Channels,
    pub users: Users,
    pub channel_users: ChannelUsers,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Payload {
    html: String,
    text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Channel {
    id: Uuid,
    name: String,
    icon: String,
    description: String,
    visibiliy: bool,
    size: i32,
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
