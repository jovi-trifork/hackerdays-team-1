use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let db = Db::default();

    let app = Router::new()
        .route("/api/v1/channels/", get(channel_messages))
        .route();
    //        Router::new().route("/", get(|| async { "Hello, world!" }));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn channel_messages() -> &'static str {
    "Hello, world!"
}

struct Message {
    id: String,
    timestamp: String,
    message: Payload,
    from_user: String,
}

struct Payload {
    html: String,
    text: String,
}