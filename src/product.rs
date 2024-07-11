use serde::Serialize;

#[derive(Serialize)]
pub struct Product {
    pub url: String,
    pub imgsrc: String,
    pub title: String,
    pub price: String,
}
