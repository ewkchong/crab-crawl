use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    sync::{Arc, Mutex},
};

struct URLJob {
    url: String,
    level: u8,
}

type SharedURLQueue = Arc<Mutex<VecDeque<URLJob>>>;

type SharedURLSet = Arc<Mutex<HashSet<String>>>;

pub async fn get_html(url: String) -> Result<scraper::Html, Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?;

    let html = scraper::Html::parse_document(&response);
    Ok(html)
}

fn get_hostname(url: &String) -> String {
    "".into()
}

pub fn crawl(seeds: Vec<String>, maxDepth: u8) -> Result<(), String> {
    let visited: SharedURLSet = Arc::new(Mutex::new(HashSet::new()));

    let seed_vec: VecDeque<URLJob> = seeds
        .into_iter()
        .map(|seed| URLJob {
            url: seed,
            level: 0,
        })
        .collect();

    if seed_vec.is_empty() {
        return Err("Seed set cannot be empty".to_string());
    }

    let hostname = get_hostname(&seed_vec.front().unwrap().url);

    let queue: SharedURLQueue = Arc::new(Mutex::new(seed_vec));

    loop {
        let front_opt = match queue.lock() {
            Ok(ref mut q) => q.pop_front(),
            Err(_) => break,
        };

        if front_opt.is_none() {
            break;
        }
        let front = front_opt.unwrap();

        let queue = queue.clone();
        let visited = visited.clone();

        tokio::spawn(async move {
            process(&front, queue, visited).await;
        });
    }

    Ok(())
}

async fn process(uj: &URLJob, queue: SharedURLQueue, visited: SharedURLSet) {
}
