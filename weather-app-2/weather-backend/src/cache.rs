use std::collections::HashMap;

use axum::Json;

use crate::models::{CacheKey, api};

const HOUR: u64 = 60 * 60;

#[derive(Debug, Clone)]
pub struct CacheService {
    responses: HashMap<CacheKey, Json<api::PreparedTemp>>,
}

impl CacheService {
    pub fn new() -> CacheService {
        CacheService {
            responses: HashMap::new(),
        }
    }
    pub fn should_refresh(&self, key: &CacheKey, ts: u64) -> bool {
        self.responses.contains_key(key)
            && ts
                .checked_sub(key.timestamp)
                .map(|sub| sub < HOUR * 2)
                .unwrap_or(false)
    }
    pub fn set(&mut self, key: CacheKey, value: Json<api::PreparedTemp>) {
        self.responses.insert(key, value);
    }
    pub fn get(&self, key: &CacheKey) -> Option<Json<api::PreparedTemp>> {
        self.responses.get(key).cloned()
    }
    pub fn get_aprx(&self, key_aprx: &CacheKey) -> Option<Json<api::PreparedTemp>> {
        if let Some(value) = self.get(key_aprx) {
            return Some(value);
        }
        self.responses
            .iter()
            .find(|(k, _)| {
                // println!(
                //     "{} == {} && {} > {} (diff {})",
                //     k.city,
                //     key_aprx.city,
                //     k.timestamp,
                //     key_aprx.timestamp,
                //     k.timestamp.abs_diff(key_aprx.timestamp)
                // );
                k.city == key_aprx.city
                    && k.timestamp.checked_sub(key_aprx.timestamp).unwrap_or(0) < HOUR * 2
            })
            .map(|(_, v)| v.clone())
    }
    pub fn del(&mut self, key: &CacheKey) {
        self.responses.remove(key);
    }
    pub fn len(&self) -> usize {
        self.responses.len()
    }
}
