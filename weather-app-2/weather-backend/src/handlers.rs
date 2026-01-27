use std::sync::Arc;

use axum::extract::State;
use axum::{Json, response::IntoResponse};
use serde_json::json;

use crate::api;
use crate::cache::{Cache, CacheService, RedisCache};
use crate::models::api::PreparedTemp;
use crate::models::vc::ResponseVC;
use crate::models::{CacheKey, FormCity};

#[axum::debug_handler]
pub async fn get_current_temperature(
    State(cache): State<Arc<CacheService<CacheKey, PreparedTemp, RedisCache>>>,
    Json(form): Json<FormCity>,
) -> impl IntoResponse {
    let user_cache_key = CacheKey::new(form.city.clone(), 0);
    log::info!("user_cache_key: {}", &user_cache_key.to_string());

    let aprx_score = form.timestamp - (form.timestamp % 3600);
    if aprx_score + 3600 > form.timestamp
        && let Some(pt) = cache
            .get(&user_cache_key, form.timestamp as i64, aprx_score as i64)
            .await
            .unwrap()
    {
        log::info!("cache hit: {} base temp {}C", &form.city, pt.temp);
        return Json(pt).into_response();
    }

    let response_vc: ResponseVC = match api::fetch_weather_api(&form.city).await {
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
    let cache_key = CacheKey::new(form.city.clone(), current_api_time);

    let pt = response_vc.get_prepared_temp();

    let ttl = 3600 - (form.timestamp % 3600);
    if let Err(set_error) = cache
        .set(cache_key, pt.clone(), ttl as i64, current_api_time as i64)
        .await
    {
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": set_error.to_string()})),
        )
            .into_response();
    };

    Json(pt).into_response()
}
