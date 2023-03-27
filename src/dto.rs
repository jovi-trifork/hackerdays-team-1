use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
struct Message {
    id: Uuid,
    timestamp: NaiveDateTime,
    message: Payload,
    from_user: Uuid,
}

#[derive(Serialize, Deserialize)]
struct Payload {
    html: String,
    text: String,
}