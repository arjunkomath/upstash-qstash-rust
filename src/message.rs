use reqwest::header;

/// Optional parameters and configuration for messages
#[derive(Debug)]
pub struct MessageSettings<'a> {
    pub delay: Option<&'a str>,
    pub retries: Option<u32>,
    pub cron: Option<&'a str>,
    pub callback: Option<&'a str>,
    pub dedup_id: Option<&'a str>,
    pub custom_headers: Option<header::HeaderMap>,
}

impl<'a> MessageSettings<'a> {
    pub fn new() -> Self {
        Self {
            delay: None,
            retries: None,
            cron: None,
            callback: None,
            dedup_id: None,
            custom_headers: None,
        }
    }

    /// Delay a message by a certain amount of time relative to the time the message was published.
    ///
    /// The format for the duration is (number)(unit). Here are some examples:
    ///
    /// 10s = 10 seconds
    /// 1m = 1 minute
    /// 30m = half an hour
    /// 2h = 2 hours
    /// 7d = 7 days
    pub fn delay(mut self, delay: &'a str) -> Self {
        self.delay = Some(delay);
        self
    }

    /// Set the number of retries for the message.
    /// The maximum number of retries depends on your current plan.
    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = Some(retries);
        self
    }

    /// Set the cron expression for the message.
    ///
    /// In addition to sending a message once, you can create a schedule, and QStash will publish the message in the given period.
    /// Schedules can be configured using cron expressions. crontab.guru is a great tool for understanding and creating cron expressions.
    /// QStash uses UTC as timezone when evaluating cron expressions.
    pub fn cron(mut self, cron: &'a str) -> Self {
        self.cron = Some(cron);
        self
    }

    /// A callback allows you to call a long running function without having to wait for its response.
    /// Instead of waiting for the request to finish, you can add a callback url to your published message
    /// and when the request finishes, QStash will call your callback URL with the response.
    pub fn callback_url(mut self, callback_url: &'a str) -> Self {
        self.callback = Some(callback_url);
        self
    }

    /// Messages can be deduplicated to prevent duplicate messages from being sent.
    /// When a duplicate message is detected, it is accepted by QStash but not enqueued.
    /// This can be useful when the connection between your service and QStash fails, and you never receive the acknowledgement.
    /// You can simply retry publishing and can be sure that the message will enqueued only once.
    pub fn dedup_id(mut self, dedup_id: &'a str) -> Self {
        self.dedup_id = Some(dedup_id);
        self
    }

    /// In addition to sending the message itself, you can also forward HTTP headers.
    pub fn custom_headers(mut self, custom_headers: header::HeaderMap) -> Self {
        self.custom_headers = Some(custom_headers);
        self
    }

    pub fn as_headers(self) -> header::HeaderMap {
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
        if let Some(custom_headers) = self.custom_headers {
            headers.extend(custom_headers);
        }

        headers
    }
}
