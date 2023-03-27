use std::collections::hash_map::Entry;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::model::{AppState, Channel, Message};

pub async fn get_messages(
    Path(channel_id): Path<String>,
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    let message_map = app_state.messages.read().unwrap();
    let ch_messages = message_map.get(&channel_id);
    if ch_messages.is_some() {
        Json(ch_messages.unwrap().clone())
    } else {
        Json(Vec::<Message>::new())
    }
}

pub async fn create_message(
    Path(channel_id): Path<String>,
    State(app_state): State<AppState>,
    Json(message): Json<Message>,
) -> impl IntoResponse {
    let mut message_map = app_state.messages.write().unwrap();
    let entry = message_map.entry(channel_id.clone());
    // is entry vacant?
    match entry {
        Entry::Vacant(v) => {
            app_state
                .channels
                .write()
                .unwrap()
                .insert(channel_id.clone(), Channel::new(channel_id.clone(), message.get_owner_id()));
            message_map.insert(channel_id.clone(), Vec::<Message>::new());
        }
        Entry::Occupied(_) => {}
    }

    message_map
        .entry(channel_id.clone())
        .or_insert(Vec::new())
        .push(message.clone());
    app_state.channels.write().unwrap().get_mut(&channel_id).unwrap().inc_size();
    
    (StatusCode::CREATED, Json(message))
}
