mod parser;
mod product;

use axum::{routing::get, Json, Router};
use http::StatusCode;

use serde::Deserialize;

async fn parse(Json(payload): Json<ParseQuery>) -> StatusCode {
    println!("{}, {}", payload.url, payload.typ);

    StatusCode::OK
}

#[derive(Deserialize)]
struct ParseQuery {
    url: String,
    typ: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/parse", get(parse));
        

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
