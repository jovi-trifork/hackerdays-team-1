use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::model::{AppState, Channel};

pub async fn create_channel(
    State(app_state): State<AppState>,
    Json(channel): Json<Channel>
) -> impl IntoResponse {
    let mut channel_map = app_state.channels.write().unwrap();
    channel_map.insert(channel.get_id(), channel.clone());

    /*let mut users_map = app_state.internalUsers.write().unwrap();
    let user_id = channel.get_owner_id();
    let user_opt = users_map.get_mut(&user_id);

    if user_opt.is_some() {
        let user = user_opt.unwrap();
        user.add_owned_channel(channel.get_id())
    } else {
        print!("No user: {:?}", channel.get_owner_id());
        (StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }*/
    (StatusCode::CREATED, Json(channel)).into_response()
}

pub async fn get_channels(State(app_state): State<AppState>) -> impl IntoResponse {
    let channel_map = app_state.channels.read().unwrap();
    let channel_list: Vec<Channel> = channel_map.values().cloned().collect();

    (StatusCode::OK, Json(channel_list))
}
