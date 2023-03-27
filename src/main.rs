mod model;
mod messages;
mod users;
mod channels;
mod systems;

use axum::{
    routing::{get, Router},
};
use messages::{get_messages, create_message};
use systems::{get_systems, create_system};
use model::*;
use users::get_users;
use channels::get_channels;
use std::{
    net::SocketAddr,
};

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
            get(get_users),
        )
        .route(
            "/api/v1/channels",
            get(get_channels),
        )
        .route(
            "/api/v1/systems",
            get(get_systems).post(create_system),
        )
        .with_state(state);
    //        Router::new().route("/", get(|| async { "Hello, world!" }));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let server = axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service());

    println!("Server running at: http://{}", addr);
    server.await.unwrap();
}


pub async fn get_index() -> &'static str {
    "Welcome!"
}
