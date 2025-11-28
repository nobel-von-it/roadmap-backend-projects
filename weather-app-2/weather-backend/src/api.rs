use crate::models::vc::ResponseVC;
use anyhow::Result;

const WEATHER_BASE_URL: &str =
    "https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline";

pub async fn fetch_weather_api<S: AsRef<str>>(city: S) -> Result<ResponseVC> {
    let api_key = std::env::var("WEATHER_API_KEY").expect("WEATHER_API_KEY must be set");
    let url = format!(
        "{}/{}?unitGroup=metric&key={}&contentType=json",
        WEATHER_BASE_URL,
        city.as_ref(),
        api_key
    );
    let response = reqwest::get(url).await?;
    let json = serde_json::from_str::<ResponseVC>(response.text().await?.as_str())?;

    Ok(json)
}
