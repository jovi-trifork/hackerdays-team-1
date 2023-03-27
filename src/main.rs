mod dto;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, Router},
    Json, http::StatusCode,
};
use chrono::Utc;
use dto::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

type ChannelMessages = Arc<RwLock<HashMap<String, Vec<Message>>>>;
type Channels = Arc<RwLock<HashMap<String, Vec<Channel>>>>;
type Users = Arc<RwLock<HashMap<String, Vec<User>>>>;

#[derive(Clone, Default)]
struct AppState {
    messages: ChannelMessages,
    channels: Channels,
    users: Users
}

#[tokio::main]
async fn main() {
    let state = AppState::default();

    let app = Router::new()
        .route(
            "/api/v1/channels/:channel_id/messages",
            get(get_channel_messages),
        )
        .with_state(state.messages);
    //        Router::new().route("/", get(|| async { "Hello, world!" }));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_channel_messages(
    Path(id): Path<String>,
    State(messages): State<ChannelMessages>,
) -> impl IntoResponse {
    let x = messages.read();
    let y = x.unwrap();
    let ch_messages = y.get(&id);
    if ch_messages.is_some() {
        Json(ch_messages.unwrap().clone())
    } else {
        Json(Vec::<Message>::new())
    }
}

// async fn create_message(
//     Path(channel_id): Path<Uuid>,
//     Json(message): Json<Message>,
//     State(db): State<ChannelMessages>,
// ) -> () {
//     db.write().
// }

