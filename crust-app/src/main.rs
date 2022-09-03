use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    let app = Router::new().route("/", get(handler));

    // Address that server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    // `std::include_str` macro can be used to include an utf-8 file as `&'static str` in compile
    // time. This method is relative to current `main.rs` file.
    Html(include_str!("../templates/index.html"))
}