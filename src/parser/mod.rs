pub mod example {

    use scraper::Selector;

    use crate::product::Product;

    pub fn process_single(html: scraper::Html) -> Option<Product> {
        let prod_selector = match Selector::parse("div.product") {
            Ok(selector) => selector,
            Err(_) => return None,
        };

        let prod_element = html.select(&prod_selector).next().unwrap();

        let imgsrc = prod_element
            .select(&Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);

        let title = prod_element
            .select(&Selector::parse("h1.product_title").unwrap())
            .next()
            .map(|h1| h1.text().collect::<String>());

        let price = prod_element
            .select(&Selector::parse("p.price>span.amount>bdi").unwrap())
            .next()
            .map(|bdi| bdi.text().collect::<String>());

        Some(Product {
            url: "".to_string(),
            imgsrc: imgsrc.unwrap_or("".to_string()),
            title: title.unwrap_or("".to_string()),
            price: price.unwrap_or("".to_string()),
        })
    }
}
