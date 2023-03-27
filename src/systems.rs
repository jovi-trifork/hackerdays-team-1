use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::model::{AppState, Message};

pub async fn get_systems(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let message_map = app_state.systems.read().unwrap();

    panic!("get_systems")
}

pub async fn create_system(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let message_map = app_state.systems.read().unwrap();

    panic!("create_system")
}