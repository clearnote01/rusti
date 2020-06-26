use anyhow::Result;
use reqwest::header;

pub fn get_authenticated_client() -> Result<reqwest::Client> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_static(crate::constants::TOKEN),
    );

    let client = reqwest::Client::builder()
        .user_agent("some random name")
        .default_headers(headers)
        .build()?;

    Ok(client)
}
