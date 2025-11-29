use std::sync::{Arc, RwLock};

use axum::extract::State;
use axum::response::Html;
use axum::{Json, response::IntoResponse};
use serde_json::json;

use crate::api;
use crate::cache::{Cache, CacheService, RuntimeCache};
use crate::models::{CacheKey, FormCity};

pub async fn get_homepage() -> impl IntoResponse {
    Html(include_str!("../../index.html")).into_response()
}

pub async fn get_current_temperature(
    State(cache): State<Arc<RwLock<CacheService<RuntimeCache>>>>,
    Json(form): Json<FormCity>,
) -> impl IntoResponse {
    let cache_key = CacheKey {
        city: form.city.clone(),
        timestamp: form.timestamp,
    };

    if let Ok(reader) = cache.read() {
        println!("reader len: {}", reader.len());
        if let Some(pt) = reader.get_aprx(&cache_key) {
            println!("something really found");
            println!("{:#?}", pt);
            return pt.into_response();
        }
    }

    let response_vc = match api::fetch_weather_api(&form.city).await {
        Ok(response) => response,
        Err(e) => {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response();
        }
    };

    let fts = form.timestamp;
    let rts = response_vc.get_current_timestamp();
    println!("{} - {} = {}", fts, rts, fts.checked_sub(rts).unwrap_or(0));

    let pt = Json(response_vc.get_prepared_temp());

    {
        if let Ok(mut writer) = cache.write() {
            writer.set(cache_key, pt.clone());
        }
    }

    pt.into_response()
}
