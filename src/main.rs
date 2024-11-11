#![allow(unused)]
use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;

use serde::Deserialize;

#[derive(Deserialize)]
struct HelloParams {
    name: Option<String>,
}
async fn hello_world(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.unwrap_or("World".to_string());
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Hello, World!</title>
            </head>
            <body>
                <h1>Hello, {name}!</h1>
            </body>
        </html>"#
    );
    return Html(html);
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
