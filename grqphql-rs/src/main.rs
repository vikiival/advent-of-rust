use dotenv::dotenv;
use dotenv::var;
use reqwest::Client;
use graphql_client::{GraphQLQuery, Response};
use regex::Regex;
use std::collections::HashSet;

const GRAPHQL_API: &str = "https://gql.rmrk.app/v1/graphql";
// let ipfs_regex: regex::Regex = Regex::new(r"^ipfs://ipfs/([a-z0-9]+)").unwrap();

#[derive(Debug)]
pub struct NFT {
    id: String,
    metadata: String,
    metadata_image: String,
    collection_id: String,
}


impl NFT {
    pub fn get_metadata(&self) -> String {
        self.metadata.clone()
    }

    pub fn get_metadata_image(&self) -> String {
        self.metadata_image.clone()
    }
}

type NFTs = Vec<NFT>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug",
)]
pub struct AllTokens {
    pub nfts: NFTs,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = var("PINATA_API_KEY").expect("PINATA_API_KEY not set");

    let data = fetch_nfts().await;
    let re = Regex::new(r"^[a-z0-9]+").unwrap();
    let nfts: Vec<NFT> = data.nfts.iter()
        .map(to_nft)
        .filter(|nft| re.is_match(&nft.collection_id))
        .collect();

    
    let only_unique_nft_images = to_set(&nfts.iter().map(NFT::get_metadata_image).collect());
    let only_unique_nft_metadata = to_set(&nfts.iter().map(NFT::get_metadata).collect());


    println!("{:#?}", nfts);
}


fn unpack(s: &Option<String>) -> String {
    match s {
        Some(s) => s.clone(),
        None => "".to_string(),
    }
}

async fn fetch_nfts() -> all_tokens::ResponseData {
    let client = Client::new();
    let request_body = AllTokens::build_query(all_tokens::Variables);

    let response: Response<all_tokens::ResponseData> = client
        .post(GRAPHQL_API)
        .json(&request_body)
        .send()
        .await
        .expect("request failed")
        .json()
        .await
        .expect("failed to parse json");

    return response.data.expect("data is null");
}

fn to_nft(nft: &all_tokens::AllTokensNfts) -> NFT {
    NFT {
        id: nft.id.clone(),
        metadata: unpack(&nft.metadata),
        metadata_image: unpack(&nft.metadata_image),
        collection_id: nft.collection_id.clone(),
    }
}

fn extract_ipfs_prefix(s: &str) -> String {
    let re = Regex::new(r"^ipfs://ipfs/([a-z0-9]+)").unwrap();
    let ipfs_prefix = re.captures(s).unwrap().get(1).map_or("", |m| m.as_str());
    ipfs_prefix.to_string()
}

fn to_set(s: &Vec<String>) -> HashSet<String> {
    s.iter().cloned().collect()
}