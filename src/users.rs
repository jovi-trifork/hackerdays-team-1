use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::model::{AppState, User};

pub async fn get_users(State(app_state): State<AppState>) -> impl IntoResponse {
    let user_map = app_state.users.read().unwrap();
    let user_list: Vec<User> = user_map.values().flatten().cloned().collect();

    (StatusCode::OK, Json(user_list))
}
