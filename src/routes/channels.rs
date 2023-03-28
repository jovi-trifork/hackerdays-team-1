use std::collections::HashMap;
use std::sync::RwLockWriteGuard;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::model::{AppState, Channel, InternalUser};
use crate::routes::users::{add_owned_channel};

pub async fn create_channel(
    State(app_state): State<AppState>,
    Json(channel): Json<Channel>
) -> impl IntoResponse {
    println!("Create");
    let mut channel_map = app_state.channels.write().unwrap();
    channel_map.insert(channel.get_id(), channel.clone());
    
    let mut user_map = app_state.internal_users.write().unwrap();

    add_owned_channel(
        &mut user_map,
        channel.get_owner_id().clone(),
        channel.get_id().clone()
    );
    println!("Added channel");

    (StatusCode::CREATED, Json(channel)).into_response()
}

pub async fn get_channels(State(app_state): State<AppState>) -> impl IntoResponse {
    let channel_map = app_state.channels.read().unwrap();
    let channel_list: Vec<Channel> = channel_map.values().cloned().collect();

    (StatusCode::OK, Json(channel_list))
}
