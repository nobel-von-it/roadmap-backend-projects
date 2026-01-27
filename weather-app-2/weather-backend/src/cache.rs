use crate::models::{CacheKey, api::PreparedTemp};
use anyhow::Result;
use redis::AsyncTypedCommands;

pub trait Cache {
    async fn set(&self, key: CacheKey, value: PreparedTemp, ttl: i64, score: i64) -> Result<()>;
    async fn get(&self, key: &CacheKey) -> Result<Option<PreparedTemp>>;
}

#[derive(Debug, Clone)]
pub struct CacheService<C: Cache> {
    service: C,
}

impl<C: Cache> CacheService<C> {
    pub fn new(service: C) -> Self {
        CacheService { service }
    }
}

impl<C: Cache> Cache for CacheService<C> {
    async fn set(&self, key: CacheKey, value: PreparedTemp, ttl: i64, score: i64) -> Result<()> {
        self.service.set(key, value, ttl, score).await
    }
    async fn get(&self, key: &CacheKey) -> Result<Option<PreparedTemp>> {
        self.service.get(key).await
    }
}
#[derive(Debug, Clone)]
pub struct RedisCache {
    pool: deadpool_redis::Pool,
}

impl RedisCache {
    pub fn new(url: &str) -> Result<RedisCache> {
        let config = deadpool_redis::Config::from_url(url);
        let pool = config.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;
        Ok(RedisCache { pool })
    }
    pub async fn get_all_keys(&self) -> Result<Vec<String>> {
        let mut conn = self.pool.get().await?;
        Ok(conn.keys("*").await?)
    }
}

impl Cache for RedisCache {
    async fn set(&self, key: CacheKey, value: PreparedTemp, ttl: i64, score: i64) -> Result<()> {
        let mut conn = self.pool.get().await?;

        conn.zadd(&key, serde_json::to_string(&value)?, score)
            .await?;
        conn.expire(&key, ttl).await?;

        Ok(())
    }
    async fn get(&self, key: &CacheKey) -> Result<Option<PreparedTemp>> {
        let mut conn = self.pool.get().await?;

        if let Some(res) = conn.zrevrange(key, 0, 0).await?.first() {
            return Ok(Some(serde_json::from_str(res)?));
        }

        Ok(None)
    }
}
