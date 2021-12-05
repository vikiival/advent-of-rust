use reqwest::Client;
use reqwest::header::HeaderMap;

static BASE_URL: &'static str = "https://api.pinata.cloud";

api_url(path: &str) -> String {
  format!("{}{}", BASE_URL, path)
}

pub struct PinataApi {
  client: Client,
}

impl PinataApi {
  pub fn new<S: Into<String>>(pinata_master_key: S) -> PinataApi {
    let master_key = pinata_master_key.into();

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", "Bearer " + master_key.clone());
    
    let client = ClientBuilder::new()
      .default_headers(default_headers)
      .build()?;

    PinataApi {
      client: Client::new(),
    }
  }

  pub async fn test_authentication(&self) -> Result<(), Err> {
    let response = self.client.get(&api_url("/data/testAuthentication"))
      .send()
      .await?;

    self.parse_ok_result(response).await
  }

  pub async fn pin_by_hash(&self, hash: PinByHash) -> Result<PinByHashResult, Err> {
    let response = self.client.post(&api_url("/pinning/pinByHash"))
      .json(&hash)
      .send()
      .await?;

    self.parse_ok_result(response).await
  }


  async fn parse_ok_result(&self, response: Response) -> Result<(), Err> {
    if response.status().is_success() {
      Ok(())
    } else {
      let error = response.json::<PinataApiError>().await?;
      Err(error.message())
    }
  }
}