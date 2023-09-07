#![allow(unused)]

use std::net::SocketAddr;

use axum::{Router, routing::get, response::{Html, IntoResponse}, extract::{Query, Path}};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello: Router = Router::new()
    .route("/hello", get(handler_hello))
    .route("/hello2/:name", get(handler_hello2));

    // region -- Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap()
    // endregion -- Start Server
}

// region: -- Handler Hello

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,

}
// ex. `/hello?name=Trundle`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// ex. `hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse{
    print!("->> {:<12} - hander_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello2 <strong>{name}</strong>"))
}

// endregion: -- Handler Hello
