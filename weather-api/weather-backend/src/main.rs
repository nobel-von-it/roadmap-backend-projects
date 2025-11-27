mod models;

use std::{env, fs, io::Write};

use anyhow::Result;
use serde_json::Value;

static API_BASE_URL: &'static str =
    "https://weather.visualcrossing.com/VisualCrossingWebServices/rest/services/timeline";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let api_key = env::var("WEATHER_KEY")?;

    let city = "krasnoyarsk";
    let response = reqwest::get(format!(
        "{}/{}?unitGroup=metric&&key={}&contentType=json",
        API_BASE_URL, city, api_key
    ))
    .await?;

    let value: Value = serde_json::from_str(&response.text().await?)?;

    if let Value::Object(obj) = value {
        for (k, _) in obj.iter() {
            println!("{}", k);
        }

        let address = obj.get("address").unwrap();
        println!("{}", address);

        let current_conditions = obj.get("currentConditions").unwrap();
        println!("{:#?}", current_conditions);
    }

    Ok(())
}
