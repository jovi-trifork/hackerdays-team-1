use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::model::{AppState, User};

pub async fn get_users(State(app_state): State<AppState>) -> impl IntoResponse {
    let user_map = app_state.users.read().unwrap();
    let user_list: Vec<User> = user_map.values().cloned().collect();

    (StatusCode::OK, Json(user_list))
}

pub async fn get_channel_users(
    Path(channel_id): Path<String>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let channel_users_map = app_state.channel_users.read().unwrap();
    let channel_users_ids = channel_users_map
        .get(&channel_id);
    let mut res: Vec<User> = vec![];
    if channel_users_ids.is_some() {
        let user_map = app_state.users.read().unwrap();
        for user_id in channel_users_ids.unwrap() {
            let user = user_map.get(user_id).unwrap();
            res.push(user.clone());
        }
    }
    (StatusCode::OK, Json(res))
}
