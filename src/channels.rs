use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, extract::Query};
use uuid::Uuid;

use crate::model::{AppState, Channel};

pub async fn create_channel(
    Query(user_id): Query<String>,
    State(app_state): State<AppState>,
    Json(channel): Json<Channel>
) -> impl IntoResponse {
    let mut channel_map = app_state.channels.write().unwrap();
    channel_map.insert(channel.get_id(), channel.clone());

    (StatusCode::OK, Json(channel))
}

pub async fn get_channels(State(app_state): State<AppState>) -> impl IntoResponse {
    let channel_map = app_state.channels.read().unwrap();
    let channel_list: Vec<Channel> = channel_map.values().cloned().collect();

    (StatusCode::OK, Json(channel_list))
}
