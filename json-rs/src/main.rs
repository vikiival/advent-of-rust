use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::fs::File;
use std::collections::BTreeMap;

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

// struct KeyValue {
//     key: String,
//     value: String
// }

type KeyValue = (String, String);


fn main() {
    let meta = get_meta().iter().map(map_to_kv).collect::<Vec<KeyValue>>();
    

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

fn map_to_kv(meta: &Metadata) -> KeyValue {
    (meta.id.clone(), one_of(&meta.image, &meta.animation_url))
}
