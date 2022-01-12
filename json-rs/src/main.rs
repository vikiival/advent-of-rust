use tokio::fs::copy;
use std::io::Cursor;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::fs::File;
use reqwest::{Client, multipart::Form};

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    id: String,
    image: Option<String>,
    animation_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    meta: Vec<Metadata>
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    data: Data
}

struct KeyValue {
    key: String,
    value: String
}

// type KeyValue = (String, String);

const PINATA_BASE_API: &str = "https://kodadot.mypinata.cloud/";
const IPFS_PREFIX: &str = "ipfs://";
const FULL_IPFS_PREFIX: &str = "https://ipfs.io/ipfs/";

fn main() {
    let meta = get_meta()
        .iter()
        .map(map_to_kv)
        .filter(only_with_value)
        .collect::<Vec<KeyValue>>();

    
    
}

fn get_meta() -> Vec<Metadata> {
    let path = Path::new("meta.json");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let meta: Response = from_reader(file).expect("error while reading or parsing");
    return meta.data.meta;
}

fn one_of(first: &Option<String>, second: &Option<String>) -> String {
    match first {
        Some(string) => string.to_string(),
        None => second.clone().unwrap_or_default()
    }
}


fn only_with_value(kv: &KeyValue) -> bool {
    kv.value != ""
}

fn map_to_kv(meta: &Metadata) -> KeyValue {
    KeyValue {
        key: meta.id.clone().replace(FULL_IPFS_PREFIX, ""),
        value: unwrap_and_replace(&meta.image)
    }
    // (meta.id.clone(), one_of(&meta.image, &meta.animation_url))
}

fn unwrap_and_replace(option: &Option<String>) -> String {
    match option {
        Some(string) => string.to_string().replace(IPFS_PREFIX, PINATA_BASE_API),
        None => "".to_string()
    }
}


async fn fetch_from_ipfs(metadata: Vec<KeyValue>) -> () {
    let client = Client::new();
    for meta in metadata {
        let response = client.get(&meta.value).send().await.unwrap();
        let bytes = response.bytes().await.unwrap();
        let mut cursor = Cursor::new(bytes);
        let path = Path::new(&meta.key);


    }
}