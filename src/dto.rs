use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct Payload {
    html: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Channel {
    id: Uuid,
    name: String,
    icon: String,
    description: String,
    visibiliy: bool,
    size: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    id: Uuid,
    timestamp: NaiveDateTime,
    message: Payload,
    from_user: Uuid,
}

#[derive(Serialize, Deserialize)]
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
