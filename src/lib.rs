//! # Upstash QStash
//! Unofficial Rust client for [Upstash QStash](https://docs.upstash.com/qstash)
//! QStash is an HTTP based messaging and scheduling solution for the serverless and edge runtimes.
use reqwest::{header, Url};
use serde::Serialize;
use serde_json::{json, Value};
use std::str;

mod utils;

/// Url of the qstash api server.
/// will be the base url for requests via this Client library.
static BASE_URL: &'static str = "https://qstash.upstash.io/v1/";

#[derive(Debug)]
pub struct MessageSettings<'a> {
    pub delay: Option<&'a str>,
    pub retries: Option<u32>,
    pub cron: Option<&'a str>,
    pub callback: Option<&'a str>,
    pub dedup_id: Option<&'a str>,
}

impl<'a> MessageSettings<'a> {
    pub fn new() -> Self {
        Self {
            delay: None,
            retries: None,
            cron: None,
            callback: None,
            dedup_id: None,
        }
    }

    pub fn delay(mut self, delay: &'a str) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = Some(retries);
        self
    }

    pub fn cron(mut self, cron: &'a str) -> Self {
        self.cron = Some(cron);
        self
    }

    pub fn callback_url(mut self, callback_url: &'a str) -> Self {
        self.callback = Some(callback_url);
        self
    }

    pub fn dedup_id(mut self, dedup_id: &'a str) -> Self {
        self.dedup_id = Some(dedup_id);
        self
    }

    fn as_headers(self) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();

        if let Some(delay) = self.delay {
            headers.insert("Upstash-Delay", delay.to_string().parse().unwrap());
        }
        if let Some(retries) = self.retries {
            headers.insert("Upstash-Retries", retries.to_string().parse().unwrap());
        }
        if let Some(cron) = self.cron {
            headers.insert("Upstash-Cron", cron.parse().unwrap());
        }
        if let Some(callback) = self.callback {
            headers.insert("Upstash-Callback", callback.parse().unwrap());
        }
        if let Some(dedup_id) = self.dedup_id {
            headers.insert("Upstash-Deduplication-Id", dedup_id.parse().unwrap());
        }

        headers
    }
}

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
    ///     let qstash_client = upstash_qstash::Client::new("your-token".to_owned()).expect("Init failed");
    ///     let body = serde_json::json!({
    ///         "key1": "value1",
    ///         "key2": "value2"
    ///     });
    ///     match qstash_client
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
