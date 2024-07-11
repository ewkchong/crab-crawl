mod crawler;
mod parser;
mod product;

use product::Product;
use serde::Deserialize;

use axum::{routing::get, Json, Router};
use http::StatusCode;

async fn parse_url(Json(payload): Json<ParseQuery>) -> Result<Json<Product>, StatusCode> {
    let document = match crawler::get_html(payload.url).await {
        Ok(html) => html,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    let product = parser::example::process_single(document).unwrap();

    Ok(Json(product))
}

#[derive(Deserialize)]
struct ParseQuery {
    url: String
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/parse_url", get(parse_url));
        
    let port = 3000;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    println!("Listening on localhost:{}...", port);
    axum::serve(listener, app).await.unwrap();
}
