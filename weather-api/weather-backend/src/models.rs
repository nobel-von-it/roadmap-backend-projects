use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentConditions {
    #[serde(rename = "temp")]
    temperature: f32,
    #[serde(rename = "feelslike")]
    feels_like: f32,

    #[serde(rename = "conditions")]
    description: String,
    datetime: String,
}
