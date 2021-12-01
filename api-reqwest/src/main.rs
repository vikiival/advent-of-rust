extern crate dotenv;

use dotenv::dotenv;
use reqwest::Client;
use std::env::var;

const PINATA_BASE_API: &str = "https://api.pinata.cloud/";

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = Client::new();
    let url = String::from(PINATA_BASE_API) + "data/testAuthentication";
    let token = var("PINATA_API_KEY").expect("PINATA_API_KEY not set");

    let res = client
        .get(url)
        .header("Authorization", token)
        .send()
        .await;
    println!("{:?}", res);
}

