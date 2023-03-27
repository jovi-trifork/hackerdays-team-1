mod model;
mod messages;
mod users;
mod channels;
use axum::{
    routing::{get, Router},
};
use messages::{get_messages, create_message};
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
        .with_state(state);
    //        Router::new().route("/", get(|| async { "Hello, world!" }));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}



