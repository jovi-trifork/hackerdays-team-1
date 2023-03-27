use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::{
    collections::HashSet
};

use crate::model::{AppState, User, InternalUser};

pub async fn get_all_users(State(app_state): State<AppState>) -> impl IntoResponse {
    let user_map = app_state.internalUsers.read().unwrap();
    let user_list: Vec<User> = user_map
        .iter()
        .map(|(_, internal_user)| internal_user.get_model())
        .collect();
    println!("Size is: {}", user_list.len());

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
        let user_map = app_state.internalUsers.read().unwrap();
        for user_id in channel_users_ids.unwrap() {
            let user = user_map.get(user_id).unwrap();
            res.push(user.get_model().clone());
        }
    }
    (StatusCode::OK, Json(res))
}

pub async fn set_user(
    State(app_state): State<AppState>,
    Json(user): Json<User>
) -> impl IntoResponse {
    update_internal_user(&app_state, user.clone());

    (StatusCode::CREATED, Json(user))
}


pub fn update_internal_user(
    app_state: &AppState,
    model: User
) -> InternalUser {
    let mut users_map = app_state.internalUsers.write().unwrap();
    let user_opt = users_map.get_mut(&model.get_id());

    if user_opt.is_some() {
        let internal_user = user_opt.unwrap();
        internal_user.set_model(model.clone());
        println!("Found user");

        return internal_user.clone();
    } else {
        let internal_user = InternalUser::new(model.get_id(), model, HashSet::new(), HashSet::new());
        users_map.insert(internal_user.get_id(), internal_user.clone());
        println!("Size is: {}", users_map.len());

        return internal_user;
    }
}

pub fn add_blocked_user(
    app_state: &AppState,
    user_id: String,
    id_to_block: String
) -> InternalUser {
    let mut users_map = app_state.internalUsers.write().unwrap();
    let user_opt = users_map.get_mut(&user_id);

    if user_opt.is_some() {
        let internal_user = user_opt.unwrap();
        internal_user.add_blocked_user(id_to_block.clone());

        return internal_user.clone();
    } else {
        update_internal_user(app_state, User::new(user_id))
    }
}

pub fn add_owned_channel(
    app_state: &AppState,
    user_id: String,
    channel_id: String
) -> InternalUser {
    let mut users_map = app_state.internalUsers.write().unwrap();
    let user_opt = users_map.get_mut(&user_id);

    if user_opt.is_some() {
        let internal_user = user_opt.unwrap();
        internal_user.add_owned_channel(channel_id.clone());

        return internal_user.clone();
    } else {
        update_internal_user(app_state, User::new(user_id))
    }
}