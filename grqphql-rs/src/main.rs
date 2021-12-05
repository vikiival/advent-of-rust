use reqwest::Client;
use graphql_client::{GraphQLQuery, Response};
use regex::Regex;
use std::collections::HashSet;

const GRAPHQL_API: &str = "https://gql.rmrk.app/v1/graphql";

#[derive(Debug)]
pub struct NFT {
    id: String,
    metadata: String,
    metadata_image: String,
    collection_id: String,
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
    let data = fetch_nfts().await;
    let re = Regex::new(r"^[a-z0-9]+").unwrap();
    let nfts: Vec<NFT> = data.nfts.iter()
        .map(to_nft)
        .filter(|nft| re.is_match(&nft.collection_id))
        .collect();

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

// fn to_set(s: &Vec<String>) -> HashSet<String> {
//     s.iter().cloned().collect()
// }