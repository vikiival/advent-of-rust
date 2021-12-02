use reqwest::Client;
use graphql_client::{GraphQLQuery, Response};
use regex::Regex;

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
     // this is the important line
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

    //  println!("{:#?}", response);
    let data = response.data.expect("data is null");
    let re = Regex::new(r"^[a-z0-9]+").unwrap();
    let nfts: Vec<NFT> = data.nfts.iter().map(|nft| NFT {
        id: nft.id.clone(),
        metadata: unpack(&nft.metadata),
        metadata_image: unpack(&nft.metadata_image),
        collection_id: nft.collection_id.clone(),
    })
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