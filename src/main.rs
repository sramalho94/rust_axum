#![allow(unused)]

use std::net::SocketAddr;

use axum::{Router, routing::get, response::Html};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(|| async {Html("Hello <strong>World!!!</strong>")}),);

    // region: --- Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    println!("->> LISTENING on {addr}\n");
    // endregion: --- Start Server
}
