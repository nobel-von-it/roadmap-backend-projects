use anyhow::{Result, anyhow};
use axum::http::HeaderMap;
use base64::Engine;

pub fn parse_idx<S: AsRef<str>>(s: S) -> Result<i32> {
    let idx = s
        .as_ref()
        .split("/")
        .last()
        .ok_or(anyhow!("Invalid URL"))?
        .parse()?;
    Ok(idx)
}
pub fn parse_basic_auth(headers: &HeaderMap) -> Option<(String, String)> {
    let value = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    if !value.starts_with("Basic ") {
        return None;
    }

    let encoded = &value[6..];
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .ok()?;
    let s = String::from_utf8(decoded).ok()?;

    let mut parts = s.splitn(2, ':');
    Some((parts.next()?.to_string(), parts.next()?.to_string()))
}
