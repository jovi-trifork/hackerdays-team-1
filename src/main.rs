mod dto;
mod todos;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, Router},
    Json,
};
use dto::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let message_db = ChannelMessages::default();

    let app = Router::new()
        .route(
            "/api/v1/channels/:channel_id/messages",
            get(channel_messages).post(create_message),
        )
        .with_state(message_db);
    //        Router::new().route("/", get(|| async { "Hello, world!" }));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn channel_messages(
    Path(id): Path<Uuid>,
    State(messages): State<ChannelMessages>,
) -> impl IntoResponse {
    let messages = messages
        .read()
        .unwrap()
        .get(&id);
    Json(messages)
}

async fn create_message(Json(message): Json<Message>, State(db): State<Db>) -> impl IntoResponse {
    let message = Message {
        id: Uuid::
        timestamp: Utc::now().to_rfc3339(),
        message,
        from_user: "test".to_string(),
    };

    let mut db = db.write().unwrap();
    db.insert(message.id, message);
}

type ChannelMessages = Arc<RwLock<HashMap<Uuid, Vec<Message>>>>;

#[derive(Serialize, Deserialize)]
struct Message {
    id: Uuid,
    timestamp: String,
    message: Payload,
    from_user: String,
}

#[derive(Serialize, Deserialize)]
struct Payload {
    html: String,
    text: String,
}
