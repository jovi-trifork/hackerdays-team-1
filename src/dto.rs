use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
struct Payload {
    html: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Channel {
    id: Uuid,
    name: String,
    icon: String,
    description: String,
    visibiliy: bool,
    size: i32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    id: Uuid,
    timestamp: NaiveDateTime,
    message: Payload,
    from_user: Uuid,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    name: String,
    status: String,
    from_system: Uuid,
    avatar: String,
}


#[derive(Serialize, Deserialize)]
struct System {
    id: Uuid,
    address: String,
    last_sync: NaiveDateTime,
    status: String,
}
