
mod model;
mod routes;
mod server_sync;

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