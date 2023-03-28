
mod model;
mod routes;

use axum::routing::{get, Router};
use model::AppState;
use std::net::SocketAddr;
use routes::channels::{get_channels, create_internal_channel};
use routes::messages::{create_message, get_messages};
use routes::systems::{get_systems, create_system};
use routes::users::{get_channel_users, get_all_users, set_user, get_internal_users, set_internal_user};

#[tokio::main]
async fn main() {
    let state = AppState::default();

    let app = Router::new()
        .route("/", get(get_index))
        .route(
            "/api/v1/channels/:channel_id/messages",
            get(get_messages).post(create_message),
        )
        .route(
            "/api/v1/users",
            get(get_all_users).post(set_user),
        )
        .route(
            "/internal/users",
            get(get_internal_users).post(set_internal_user)
        )
        .route(
            "/api/v1/channels",
            get(get_channels),
        )
        .route(
            "/internal/channels",
            get(get_channels).post(create_internal_channel),
        )
        .route(
            "/api/v1/channels/{channel_id}/users", 
            get(get_channel_users),
        )  
        .route(
            "/api/v1/systems", 
            get(get_systems).post(create_system), 
        )   
        .with_state(state.clone());

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let server = axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service());

    println!("Server running at: http://{}", addr);
    server_sync::start_server_sync(state);

    server.await.unwrap();
}


pub async fn get_index() -> &'static str {
    "Welcome!"
}


mod server_sync {
    use crate::model;
    use model::*;

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
                let state = get_state(system).await;
                println!("{state:?}")

                // TODO: Merge

            }


    }

    async fn get_state(system: &System) -> Option<ServerSyncAppState> {
        let address = system.get_address();
        let url_base = format!("{address}/api/v1");

        // Get channels
        let url_get_channels = format!("{url_base}/channels", );
        let channels: GetChannelsResponse = send_get_request(&url_get_channels).await?;
        println!("Received channels: {channels:#?}");

        // Get messages in each channel
        for channel in channels {
            let channel_id = channel.get_id();
            let url_get_messages = format!("{url_base}/channels/{channel_id}/messages", );
            let messages: GetMessagesResponse = send_get_request(&url_get_messages).await?;

            println!("Messages in channel {channel_id}: {messages:#?}");
        }
        
        
        // Get users
        let url_get_users = format!("{url_base}/users", );
        let users: GetUsersResponse = send_get_request(&url_get_users).await?;
        println!("Received users: {users:#?}");

        /*
        let url_get_systems = format!("http://{address}/api/v1/systems");
        */
        
        None
    }
    
    async fn send_get_request<T: for<'de> serde::Deserialize<'de>>(url: &str) -> Option<T> {    
        reqwest::get(url)
                    .await.ok()?
                    .json::<T>()
                    .await.ok()
    }
}