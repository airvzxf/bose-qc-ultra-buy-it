use crate::AppError;
use reqwest::blocking::{Client, Response};
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, HOST, TE,
    UPGRADE_INSECURE_REQUESTS, USER_AGENT,
};
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn extract_html_content(url: &str) -> Result<String, AppError> {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"));
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("es-MX,en-US;q=0.7,en;q=0.3"),
    );
    headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(HOST, HeaderValue::from_static("www.liverpool.com.mx"));
    headers.insert(TE, HeaderValue::from_static("trailers"));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));

    let client: Client = Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .build()?;

    let response: Response = client.get(url).send()?;

    if response.status().is_success() {
        Ok(response.text()?)
    } else {
        Err(AppError::HttpStatusCode(response.status().to_string()))
    }
}

pub fn store_response_to_file(content: &String, file_path: &Path) -> Result<(), AppError> {
    let mut file: File = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}
