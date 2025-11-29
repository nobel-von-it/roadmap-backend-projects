use std::collections::HashMap;

use axum::Json;

use crate::models::{CacheKey, api};

const HOUR: u64 = 60 * 60;

pub trait Cache {
    fn should_refresh(&self, key: &CacheKey, ts: u64) -> bool;
    fn set(&mut self, key: CacheKey, value: Json<api::PreparedTemp>);
    fn get(&self, key: &CacheKey) -> Option<Json<api::PreparedTemp>>;
    fn get_aprx(&self, key_aprx: &CacheKey) -> Option<Json<api::PreparedTemp>>;
    fn del(&mut self, key: &CacheKey);
    fn len(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct CacheService<C: Clone> {
    service: C,
}

impl<C: Clone> CacheService<C> {
    pub fn new(service: C) -> CacheService<C> {
        CacheService { service }
    }
}

impl Cache for CacheService<RuntimeCache> {
    fn should_refresh(&self, key: &CacheKey, ts: u64) -> bool {
        self.service.should_refresh(key, ts)
    }
    fn set(&mut self, key: CacheKey, value: Json<api::PreparedTemp>) {
        self.service.set(key, value);
    }
    fn get(&self, key: &CacheKey) -> Option<Json<api::PreparedTemp>> {
        self.service.get(key)
    }
    fn get_aprx(&self, key_aprx: &CacheKey) -> Option<Json<api::PreparedTemp>> {
        self.service.get_aprx(key_aprx)
    }
    fn del(&mut self, key: &CacheKey) {
        self.service.del(key);
    }
    fn len(&self) -> usize {
        self.service.len()
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeCache {
    responses: HashMap<CacheKey, Json<api::PreparedTemp>>,
}

impl RuntimeCache {
    pub fn new() -> RuntimeCache {
        RuntimeCache {
            responses: HashMap::new(),
        }
    }
}

impl Cache for RuntimeCache {
    fn should_refresh(&self, key: &CacheKey, ts: u64) -> bool {
        self.responses.contains_key(key)
            && ts
                .checked_sub(key.timestamp)
                .map(|sub| sub < HOUR * 2)
                .unwrap_or(false)
    }
    fn set(&mut self, key: CacheKey, value: Json<api::PreparedTemp>) {
        self.responses.insert(key, value);
    }
    fn get(&self, key: &CacheKey) -> Option<Json<api::PreparedTemp>> {
        self.responses.get(key).cloned()
    }
    fn get_aprx(&self, key_aprx: &CacheKey) -> Option<Json<api::PreparedTemp>> {
        if let Some(value) = self.get(key_aprx) {
            return Some(value);
        }
        self.responses
            .iter()
            .find(|(k, _)| {
                k.city == key_aprx.city
                    && k.timestamp.checked_sub(key_aprx.timestamp).unwrap_or(0) < HOUR * 2
            })
            .map(|(_, v)| v.clone())
    }
    fn del(&mut self, key: &CacheKey) {
        self.responses.remove(key);
    }
    fn len(&self) -> usize {
        self.responses.len()
    }
}
