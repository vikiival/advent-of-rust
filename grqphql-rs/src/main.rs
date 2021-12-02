use reqwest::Client;
use graphql_client::{GraphQLQuery, Response};

const GRAPHQL_API: &str = "https://gql.rmrk.app/v1/graphql";

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug",
)]
pub struct AllTokens;

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

    
    //  let response_body: Response<union_query::ResponseData> = res.json().await?;
    //  println!("{:#?}", response_body);
}
