
pub mod parser {
    // pub mod amazon {
    //     fn process_single(html: scraper::Html) -> crate::product::Product {
    //
    //     }
    // }
    pub mod example {
        use std::error::Error;

        use scraper::Selector;

        use crate::product::Product;

        fn process_single(html: scraper::Html) -> Result<Product, Box<dyn Error>> {
            let prod_selector = Selector::parse("div.product")?;

            let prod_element = html.select(&prod_selector).next().unwrap();

            let imgsrc = prod_element
                .select(&Selector::parse("img").unwrap())
                .next()
                .and_then(|img| img.value().attr("href"))
                .map(str::to_owned);

            let title = prod_element
                .select(&Selector::parse("h1.product_title").unwrap())
                .next()
                .map(|h1| h1.text().collect::<String>());

            let price = prod_element
                .select(&Selector::parse("p.price>span.amount>bdi").unwrap())
                .next()
                .map(|bdi| bdi.text().collect::<String>());

            Ok(Product {
                url: "".to_string(),
                imgsrc: imgsrc.unwrap(),
                title: title.unwrap(),
                price: price.unwrap()
            })
        }
    }
}

