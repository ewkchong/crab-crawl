#[derive(Debug)]
pub struct Product {
    pub url: String,
    pub imgsrc: String,
    pub title: String,
    pub price: String,
}

pub type ProductList = Vec<Product>;
