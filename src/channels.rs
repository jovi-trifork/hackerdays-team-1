use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::model::{AppState, Channel};

pub async fn get_channels(State(app_state): State<AppState>) -> impl IntoResponse {
    let channel_map = app_state.channels.read().unwrap();
    let channel_list: Vec<Channel> = channel_map.values().flatten().cloned().collect();

    (StatusCode::OK, Json(channel_list))
}
