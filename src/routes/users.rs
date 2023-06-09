use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::{
    collections::{HashSet, HashMap}, sync::{Arc, RwLock, RwLockWriteGuard}
};

use crate::model::{AppState, User, InternalUser};

pub async fn get_all_users(State(app_state): State<AppState>) -> impl IntoResponse {
    let user_map = app_state.internal_users.read().unwrap();
    let user_list: Vec<User> = user_map.values()
        .map(InternalUser::get_model)
        .collect();

    (StatusCode::OK, Json(user_list))
}

pub async fn get_channel_users(
    Path(channel_id): Path<String>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let channel_users_map = app_state.channel_users.read().unwrap();
    let channel_users_ids = channel_users_map.get(&channel_id);
    let res = match channel_users_ids {
        Some(ids) => {
            let user_map = app_state.internal_users.read().unwrap();
            ids.iter()
                .map(|id|user_map.get(id).unwrap().get_model())
                .collect()
        },
        None => Vec::new()
    };
    (StatusCode::OK, Json(res))
}

pub async fn set_user(
    State(app_state): State<AppState>,
    Json(user): Json<User>
) -> impl IntoResponse {
    let mut user_map = app_state.internal_users.write().unwrap();
    let mut internal_user = get_or_create_user(&user_map, user.get_id());
    internal_user.set_model(user.clone());
    user_map.insert(user.get_id().to_owned(), internal_user);

    (StatusCode::CREATED, Json(user))
}

pub async fn get_internal_users(State(app_state): State<AppState>) -> impl IntoResponse {
    let users_map = app_state.internal_users.read().unwrap();
    let user_list: Vec<InternalUser> = users_map.values().cloned().collect();

    (StatusCode::OK, Json(user_list))
}

pub async fn set_internal_user(
    Path(user_id): Path<String>,
    State(app_state): State<AppState>,
    Json(user): Json<InternalUser>
) -> impl IntoResponse {
    let mut users_map = app_state.internal_users.write().unwrap();
    users_map.insert(user_id, user.clone());

    (StatusCode::OK, Json(user))
}

pub fn get_or_create_user(
    users_map: &RwLockWriteGuard<HashMap<String, InternalUser>>,
    user_id: &str,
) -> InternalUser {
    let user_opt = users_map.get(user_id);

    if let Some(user) = user_opt {
        return user.clone();
    }

    return InternalUser::new(
        user_id.to_owned(),
        User::new(user_id.to_owned()),
        HashSet::new(),
        HashSet::new(),
    );
}
