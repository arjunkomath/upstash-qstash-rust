//! # Upstash QStash
//! Unofficial Rust client for [Upstash QStash](https://docs.upstash.com/qstash)
//! QStash is an HTTP based messaging and scheduling solution for the serverless and edge runtimes.
use reqwest::{header, Url};
use serde::Serialize;
use serde_json::{json, Value};
use std::str;

pub use message::MessageSettings;
pub use utils::{QStashError, Result};

mod message;
mod utils;

/// Url of the qstash api server.
/// will be the base url for requests via this Client library.
static BASE_URL: &'static str = "https://qstash.upstash.io/v1/";

/// QStash client
pub struct Client {
    http: reqwest::Client,
    api_base_url: Url,
}

impl Client {
    /// Create a new QStash client using your token.
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

        let api_base_url = Url::parse(BASE_URL)?;

        Ok(Self { http, api_base_url })
    }

    /// Get your current quota limits.
    ///
    /// # Example
    ///
    /// ```
    /// match qstash.get_quota().await {
    ///     Ok(result) => println!("Quota: {:?}", result),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub async fn get_quota(&self) -> utils::Result<Value> {
        let endpoint = self.api_base_url.join("quota")?;
        let response = self.http.get(endpoint).send().await?;
        let body = response.json().await?;
        Ok(body)
    }

    /// Get the complete message with the given id
    ///
    /// # Arguments
    ///
    /// * `message_id` - The id of the message to get.
    ///
    /// # Example
    ///
    /// ```
    /// match qstash.get_message("msg_5QRRvEnXf9J".to_owned()).await {
    ///     Ok(result) => println!("Result: {:?}", result),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub async fn get_message(&self, message_id: String) -> utils::Result<Value> {
        let endpoint = self
            .api_base_url
            .join(format!("messages/{}", message_id).as_str())?;
        let response = self.http.get(endpoint).send().await?;
        let body = response.json().await?;
        Ok(body)
    }

    /// Publish a message to a URL or Topic
    ///
    /// # Arguments
    ///
    /// * `url_or_topic` - The url of the endpoint to publish to.
    /// * `body` - The JSON message to publish.
    ///
    /// # Example
    ///
    /// ```
    /// use qstash::Client;
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), ()> {
    ///     let qstash = upstash_qstash::Client::new("your-token".to_owned()).expect("Init failed");
    ///     let body = serde_json::json!({
    ///         "key1": "value1",
    ///         "key2": "value2"
    ///     });
    ///     match qstash
    ///         .publish_json(
    ///             "url-or-token".to_owned(),
    ///             &body,
    ///         )
    ///         .await
    ///     {
    ///         Ok(result) => println!("Published {:?}", result),
    ///         Err(e) => println!("Error: {}", e),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn publish_json<'a, T, U>(
        &self,
        url_or_topic: String,
        body: &T,
        message_settings: U,
    ) -> utils::Result<Value>
    where
        T: Serialize,
        U: Into<Option<MessageSettings<'a>>>,
    {
        let endpoint = self
            .api_base_url
            .join(format!("publish/{}", url_or_topic).as_str())?;

        let message_settings = message_settings.into().unwrap_or(MessageSettings::new());

        let payload = json!(body);
        let response = self
            .http
            .post(endpoint)
            .headers(message_settings.as_headers())
            .json(&payload)
            .send()
            .await?;
        let body = response.json().await?;
        Ok(body)
    }
}
