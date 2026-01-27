use std::sync::{Arc, RwLock};

use axum::extract::State;
use axum::{Json, response::IntoResponse};
use serde_json::json;

use crate::api;
use crate::cache::{Cache, CacheService};
use crate::models::{CacheKey, FormCity};

pub async fn get_current_temperature<C: Cache>(
    State(cache): State<Arc<RwLock<CacheService<C>>>>,
    Json(form): Json<FormCity>,
) -> impl IntoResponse {
    let user_cache_key = CacheKey::new(form.city.clone(), 0, form.timestamp);
    log::info!("user_cache_key: {}", &user_cache_key.to_string());

    if let Ok(reader) = cache.read() {
        log::info!("reader len: {}", reader.len());
        if let Some(pt) = reader.get(&user_cache_key) {
            log::info!("cache hit: {} base temp {}C", &form.city, pt.temp);
            return Json(pt).into_response();
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

    let current_api_time = response_vc.get_current_api_time();
    let cache_key = CacheKey::new(form.city.clone(), current_api_time, form.timestamp);

    let pt = response_vc.get_prepared_temp();

    {
        if let Ok(mut writer) = cache.write() {
            writer.set(cache_key, pt.clone());
        }
    }

    Json(pt).into_response()
}
