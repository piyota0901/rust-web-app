use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html

    axum::Server::bind(&addr) // 127.0.0.1:3000にバインド
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}
