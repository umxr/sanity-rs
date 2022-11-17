#![crate_name = "sanity_rs"]

extern crate reqwest;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

pub struct Config {
    project_id: String,
    dataset: String,
    token: String,
}

fn construct_headers(config: &Config) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&config.token).unwrap(),
    );
    headers.insert(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers
}

fn construct_project_url(config: &Config) -> String {
    format!(
        "https://{}.sanity.io/v1/data/query/{}?query=*",
        config.project_id, config.dataset
    )
}

impl Config {
    pub fn new(project_id: String, dataset: String, token: String) -> Config {
        Config {
            project_id,
            dataset,
            token,
        }
    }
    pub async fn query(&self, query: String) -> Result<reqwest::Response, reqwest::Error> {
        let client = Client::new();
        let headers = construct_headers(&self);
        let url = construct_project_url(&self).replace("*", &query);
        client.get(url).headers(headers).send().await
    }
}
