mod channels;
mod messages;
mod model;
mod systems;
mod users;
use axum::routing::{get, Router};
use channels::{get_channels, create_channel};
use messages::{create_message, get_messages};
use model::*;
use std::net::SocketAddr;
use systems::{create_system, get_systems};
use users::{get_channel_users, get_users};

#[tokio::main]
async fn main() {
    let state = AppState::default();

    let app = Router::new()
        .route(
            "/api/v1/channels/:channel_id/messages",
            get(get_messages).post(create_message),
        )
        .route(
            "/api/v1/users",
            get(get_users),
        )
        .route(
            "/api/v1/channels",
            get(get_channels).post(create_channel),
        )
        .with_state(state);

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}
