mod handlers;
use std::borrow::Borrow;

use crate::handlers::{json_handler, template_handler, text_handler};

use axum::{handler::get, Router};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(template_handler))
        .route("/api/whoami", get(json_handler))
        .route("/text", get(text_handler));
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(addr.borrow())
        .serve(router.into_make_service())
        .await
        .expect("Server failed to start");
}
