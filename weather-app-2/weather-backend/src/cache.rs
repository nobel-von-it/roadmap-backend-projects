use std::{collections::HashMap, time::Duration};

use crate::models::{CacheKey, api};
use anyhow::Result;
use axum::Json;
use redis::TypedCommands;

const HOUR: u64 = 60 * 60;

pub trait Cache {
    fn set(&mut self, key: CacheKey, value: api::PreparedTemp);
    fn get(&self, key: &CacheKey) -> Option<api::PreparedTemp>;
    fn del(&mut self, key: &CacheKey);
    fn len(&self) -> usize;
}

#[derive(Debug)]
pub struct CacheService<C: Cache> {
    service: C,
}

impl<C: Cache> CacheService<C> {
    pub fn new(service: C) -> CacheService<C> {
        CacheService { service }
    }
}

impl<C: Cache> Cache for CacheService<C> {
    fn set(&mut self, key: CacheKey, value: api::PreparedTemp) {
        self.service.set(key, value);
    }
    fn get(&self, key: &CacheKey) -> Option<api::PreparedTemp> {
        self.service.get(key)
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
    responses: HashMap<CacheKey, api::PreparedTemp>,
}

impl RuntimeCache {
    pub fn new() -> RuntimeCache {
        RuntimeCache {
            responses: HashMap::new(),
        }
    }
    fn get_last_by_city(&self, city_name: &str) -> Option<(CacheKey, api::PreparedTemp)> {
        self.responses
            .iter()
            .filter(|(k, _)| k.city_name == city_name)
            .reduce(|(k1, v1), (k2, v2)| {
                if k1.api_timestamp > k2.api_timestamp {
                    (k1, v1)
                } else {
                    (k2, v2)
                }
            })
            .map(|(k, v)| (k.clone(), v.clone()))
    }
}

impl Cache for RuntimeCache {
    fn set(&mut self, key: CacheKey, value: api::PreparedTemp) {
        self.responses.insert(key, value);
    }
    fn get(&self, user_key: &CacheKey) -> Option<api::PreparedTemp> {
        if let Some((k, v)) = self.get_last_by_city(user_key.city_name.as_str())
            && k.api_timestamp + HOUR < user_key.user_timestamp
        {
            return Some(v);
        }
        None
    }
    fn del(&mut self, key: &CacheKey) {
        self.responses.remove(key);
    }
    fn len(&self) -> usize {
        self.responses.len()
    }
}

pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    pub fn new(url: &str) -> Result<RedisCache> {
        let client = redis::Client::open(url)?;
        Ok(RedisCache { client })
    }
    pub fn get_all_keys(&self) -> Vec<String> {
        let mut conn = self.client.get_connection().unwrap();
        conn.keys("*").expect("KEYS failed")
    }
}

impl Cache for RedisCache {
    fn set(&mut self, key: CacheKey, value: api::PreparedTemp) {
        let mut conn = self.client.get_connection().unwrap();

        let encoded = serde_json::to_string(&value).expect("Serialization failed");

        conn.zadd(&key.city_name, encoded, key.api_timestamp)
            .expect("ZADD failed");

        let expire_time = HOUR - key.user_timestamp % HOUR;
        log::info!(
            "key with name {} will expire in {} seconds",
            key.city_name,
            expire_time
        );
        conn.expire(&key.city_name, expire_time as i64)
            .expect("EXPIRE failed");
    }
    fn get(&self, key: &CacheKey) -> Option<api::PreparedTemp> {
        let mut conn = self.client.get_connection().unwrap();
        conn.zrevrange(&key.city_name, 0, key.user_timestamp as isize)
            .expect("ZREVRANGE failed")
            .last()
            .map(|v| serde_json::from_str(v).expect("Deserialization failed"))
    }
    fn del(&mut self, key: &CacheKey) {
        let mut conn = self.client.get_connection().unwrap();
        conn.zrembyscore(key.to_string(), key.api_timestamp, key.api_timestamp)
            .expect("ZREMBYSCORE failed");
    }
    fn len(&self) -> usize {
        let mut conn = self.client.get_connection().unwrap();
        conn.keys("*").unwrap().len()
    }
}
