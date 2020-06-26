use anyhow::Result;
use reqwest::header;
use std::env;

const ACCESS_TOKEN_KEY: &str = "MY_GITHUB_ACCESS_TOKEN";

pub fn get_authenticated_client() -> Result<reqwest::Client> {
    let mut headers = header::HeaderMap::new();
    let token: String = env::var(&ACCESS_TOKEN_KEY).unwrap();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&token)?,
    );

    let client = reqwest::Client::builder()
        .user_agent("some random name")
        .default_headers(headers)
        .build()?;

    Ok(client)
}
