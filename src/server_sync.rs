use crate::model;
use model::*;
use std::{collections::HashMap, hash::Hash};

pub fn start_server_sync(state: AppState ) {
    use std::thread;
    use std::time::Duration;

    // Start a thread to every poll all systems.
    tokio::task::spawn(async move {
        loop {
            server_sync(&state).await;
            thread::sleep(Duration::from_secs(10));
        }
    });
}

async fn server_sync(state: &AppState)  {
    let systems = state.systems.read().unwrap().clone();
    let addresses = systems.values().map(|system| system.get_address()).collect::<Vec<&str>>();
    println!("Polling systems {addresses:?}");

        for system in systems.values() {
            let system_state = get_state(system).await.unwrap();
            println!("{system_state:?}");

            state.merge(system_state)
        }
}

async fn get_state(system: &System) -> Option<ServerSyncAppState> {
    let address = system.get_address();
    let url_base = format!("{address}/api/v1");

    // Get channels
    let url_get_channels = format!("{url_base}/channels", );
    let channels: GetChannelsResponse = send_get_request(&url_get_channels).await?;

    let mut internal_channels = InternalChannels::new();
    for channel in channels {
        let internal_channel: InternalChannel = channel.into();
        internal_channels.insert(internal_channel.get_id().to_owned(), internal_channel);
    }

    // Get messages in each channel
    let mut messages = InternalChannelMessages::new();
    for internal_channel in internal_channels.values() {
        let channel_id = internal_channel.get_id();
        let url_get_messages = format!("{url_base}/channels/{channel_id}/messages", );
        let channel_messages: GetMessagesResponse = send_get_request(&url_get_messages).await?;

        messages.insert(channel_id.to_owned(), channel_messages);
    }
    
    // Get users
    let url_get_users = format!("{url_base}/users", );
    let users: GetUsersResponse = send_get_request(&url_get_users).await?;

    let mut internal_users = InternalUsers::new();
    for user in users {
        let internal_user: InternalUser = user.into();
        internal_users.insert(internal_user.get_id().to_owned(), internal_user);
    }

    // Return
    let state = ServerSyncAppState{
        messages,
        internal_channels,
        internal_users,
    };

    return Some(state);
}

async fn send_get_request<T: for<'de> serde::Deserialize<'de>>(url: &str) -> Option<T> {    
    reqwest::get(url)
                .await.ok()?
                .json::<T>()
                .await.ok()
}