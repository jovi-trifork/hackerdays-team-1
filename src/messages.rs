use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::model::{AppState, Message};

pub async fn get_messages(
    Path(id): Path<String>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let message_map = app_state.messages.read().unwrap();
    let ch_messages = message_map.get(&id);
    if ch_messages.is_some() {
        Json(ch_messages.unwrap().clone())
    } else {
        Json(Vec::<Message>::new())
    }
}

pub async fn create_message(
    Path(channel_id): Path<String>,
    State(app_state): State<AppState>,
    Json(message): Json<Message>,
) -> impl IntoResponse {
    let mut message_map = app_state.messages.write().unwrap();
    message_map
        .entry(channel_id)
        .or_insert(Vec::new())
        .push(message.clone());
    (StatusCode::CREATED, Json(message))
}
