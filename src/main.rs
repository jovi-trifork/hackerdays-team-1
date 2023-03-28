
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
    use model::AppState;

    pub fn start_server_sync(app_state: AppState ) {
        use std::thread;
        use std::time::Duration;
    
        // Start a thread to every poll all systems.
        tokio::task::spawn(async move {
            loop {
                server_sync(&app_state).await;
                thread::sleep(Duration::from_secs(10));
            }
        });
    }
    
    async fn server_sync(app_state: &AppState)  {
        let systems = app_state.systems.read().unwrap().clone();
        let addresses = systems.values().map(|system| system.get_address()).collect::<Vec<&str>>();
        println!("Polling systems {addresses:?}");
    
            for system in systems.values() {
                // Get address
                let channels = get_channels(system).await;
                
                // Get channels
                println!("Received channels: {channels:?}")
                
                /*
                let url_get_messages_for_channel = format!("http://{address}/api/v1/channels/{channel_id}/messages");
                let url_get_users = format!("http://{address}/api/v1/users");
                let url_get_systems = format!("http://{address}/api/v1/systems");
                */
        
                
        
                // Get messages
        
                // Merge
            }
    }
    
    async fn get_channels(system: &model::System) -> Option<model::GetChannelsResponse> {
        let url_get_channels = format!("{}/api/v1/channels", system.get_address());
    
    println!("Sending request to {url_get_channels}");
        let resp = reqwest::get(url_get_channels)
                    .await.ok()?
                    .json::<model::GetChannelsResponse>()
                    .await.ok()?;
        println!("{:#?}", resp);
        return Some(resp)
    }
}