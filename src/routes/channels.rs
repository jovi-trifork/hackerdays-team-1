use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::model::{AppState, Channel, InternalChannel};
use crate::routes::users::{get_or_create_user};

pub async fn create_internal_channel(
    State(app_state): State<AppState>,
    Json(channel): Json<InternalChannel>
) -> impl IntoResponse {
    println!("Create");
    let mut channel_map = app_state.internal_channels.write().unwrap();
    channel_map.insert(channel.get_id(), channel.clone());
    
    let user_id = channel.get_owner_id().clone();
    let mut user_map = app_state.internal_users.write().unwrap();
    let mut user = get_or_create_user(
        &mut user_map,
        user_id.clone()
    );
    user.add_owned_channel(channel.get_id().clone());
    user_map.insert(user_id.clone(), user);

    (StatusCode::CREATED, Json(channel))
}

pub async fn get_internal_channels(State(app_state): State<AppState>) -> impl IntoResponse {

}

pub async fn get_channels(State(app_state): State<AppState>) -> impl IntoResponse {
    let channel_map = app_state.internal_channels.read().unwrap();
    let channel_list: Vec<Channel> = channel_map.values()
        .map(InternalChannel::get_model)
        .collect();

    (StatusCode::OK, Json(channel_list))
}