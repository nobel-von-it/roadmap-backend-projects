use anyhow::Result;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LTResponse {
    matches: Vec<LTMatch>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LTMatch {
    message: String,
    #[serde(rename = "shortMessage")]
    short_message: String,
    relpacements: Vec<LTReplacement>,
    offset: usize,
    length: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LTReplacement {
    value: String,
}

pub async fn check_grammar(text: &str) -> Result<LTResponse> {
    // TODO: use a docker container to run the grammar check locally
    let req = reqwest::Client::new()
        .post("http://localhost:8010/v2/check")
        .form(&[("language", "ru-RU"), ("text", text)]);
    match req.send().await {
        Ok(res) => {
            let status = res.status();
            tracing::info!(status = ?status, "Grammar check request sent successfully");
            match res.text().await {
                Ok(text) => {
                    tracing::info!(text = ?format!("{}", &text[0..100]), "Grammar check response received successfully");
                    let text: LTResponse = serde_json::from_str(&text)?;
                    Ok(text)
                }
                Err(err) => {
                    tracing::error!(error = %err, "Failed to read response body");
                    Err(anyhow::anyhow!("Internal Server Error"))
                }
            }
        }
        Err(err) => {
            tracing::error!(error = %err, "Failed to send grammar check request");
            Err(anyhow::anyhow!("Internal Server Error"))
        }
    }
}
