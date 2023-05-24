use reqwest::{header, Url};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str;

mod utils;

/// Url of the qstash api server.
/// will be the base url for requests via this Client library.
static BASE_URL: &'static str = "https://qstash.upstash.io/v1";
// static BASE_URL: &'static str = "http://httpbin.org/post"; // For testing

pub struct Client {
    pub http: reqwest::Client,
    url: Url,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub message_id: String,
}

impl Client {
    /// Create a new qstash client.
    /// The token is the api key of your qstash account.
    /// You can get it from the qstash dashboard.
    pub fn new(token: String) -> utils::Result<Self> {
        let auth = format!("Bearer {}", token);

        let mut value = header::HeaderValue::from_str(auth.as_str())?;
        value.set_sensitive(true);

        let mut headers = header::HeaderMap::new();
        headers.append(header::AUTHORIZATION, value);

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        let url = Url::parse(BASE_URL)?;

        Ok(Self { http, url })
    }

    /// Publish a message to qstash.
    ///
    /// # Arguments
    ///
    /// * `url` - The url of the endpoint to publish to.
    /// * `body` - The JSON message to publish.
    pub async fn publish_json<T: Serialize>(
        &self,
        url: String,
        body: &T,
    ) -> utils::Result<ApiResponse> {
        let url = self.url.join(format!("/publish/{}", url).as_str())?;

        let payload = json!(body);

        let response = self.http.post(url).json(&payload).send().await?;

        if response.status().is_success() {
            let body = response.json().await?;
            Ok(body)
        } else {
            Err(response.error_for_status().unwrap_err().into())
        }
    }
}
