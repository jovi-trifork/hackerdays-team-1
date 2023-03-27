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
            get(get_messages).post(create_message),
        )
        .route(
            "/api/v1/users",
            get(get_users),
        )
        .with_state(state);
    //        Router::new().route("/", get(|| async { "Hello, world!" }));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_messages(
    Path(id): Path<String>,
    State(appState): State<AppState>,
) -> impl IntoResponse {
    let message_map = appState.messages.read().unwrap();
    let ch_messages = message_map.get(&id);
    if ch_messages.is_some() {
        Json(ch_messages.unwrap().clone())
    } else {
        Json(Vec::<Message>::new())
    }
}

async fn create_message(
    Path(channel_id): Path<String>,
    State(appState): State<AppState>,
    Json(message): Json<Message>,
) -> impl IntoResponse {
    let mut message_map = appState.messages.write().unwrap();
    message_map.entry(channel_id).or_insert(Vec::new()).push(message.clone());
    (StatusCode::CREATED, Json(message))
}

async fn get_users(State(appState): State<AppState>) -> impl IntoResponse {
    let user_map = appState.users.read().unwrap();
    let user_list: Vec<User> = user_map.values().flatten().cloned().collect();

    (StatusCode::OK, Json(user_list))
}



