use reqwest::Client as HttpClient;

pub struct ApiClient {
    base_url: String,
    pub token: Option<String>,
    http_client: HttpClient,
}

impl ApiClient {
    pub fn new(base_url: String, token: Option<String>) -> Self {
        let base_url = base_url.trim_end_matches('/').to_string();

        Self {
            base_url,
            token,
            http_client: HttpClient::new(),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn http_client(&self) -> &HttpClient {
        &self.http_client
    }

    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token;
    }
}
