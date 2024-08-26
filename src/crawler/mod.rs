use std::include_str;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

const RAW_CRAWLER_DATA: &str = include_str!("crawler-user-agents.json");

#[derive(Deserialize, Debug)]
struct CrawlerData {
    pattern: String,
}

#[derive(Debug)]
struct Crawler {
    pattern: Regex,
}

lazy_static! {
    static ref CRAWLERS: Vec<Crawler> = serde_json::from_str::<Vec<CrawlerData>>(RAW_CRAWLER_DATA)
        .unwrap()
        .into_iter()
        .map(|crawler| Crawler {
            pattern: Regex::new(&crawler.pattern).unwrap(),
        })
        .collect();
}

pub fn is_crawler(user_agent: &str) -> bool {
    CRAWLERS
        .iter()
        .any(|crawler| crawler.pattern.is_match(user_agent))
}
