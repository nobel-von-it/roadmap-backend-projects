use crate::models::{
    CacheKey,
    api::{self, PreparedTemp},
};
use anyhow::Result;
use redis::AsyncTypedCommands;

pub trait Cache<K, V> {
    async fn set(&self, key: K, value: V, ttl: i64, score: i64) -> Result<()>;
    async fn get(&self, key: &K, current: i64, score: i64) -> Result<Option<V>>;
}

#[derive(Debug, Clone)]
pub struct CacheService<K, V, C: Cache<K, V>> {
    service: C,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K, V, C: Cache<K, V>> CacheService<K, V, C> {
    pub fn new(service: C) -> Self {
        CacheService {
            service,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<K, V, C: Cache<K, V>> Cache<K, V> for CacheService<K, V, C> {
    async fn set(&self, key: K, value: V, ttl: i64, score: i64) -> Result<()> {
        self.service.set(key, value, ttl, score).await
    }
    async fn get(&self, key: &K, current: i64, score: i64) -> Result<Option<V>> {
        self.service.get(key, current, score).await
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

impl Cache<CacheKey, PreparedTemp> for RedisCache {
    async fn set(&self, key: CacheKey, value: PreparedTemp, ttl: i64, score: i64) -> Result<()> {
        let mut conn = self.pool.get().await?;

        conn.zadd(&key, serde_json::to_string(&value)?, score)
            .await?;
        conn.expire(&key, ttl).await?;

        Ok(())
    }
    async fn get(&self, key: &CacheKey, current: i64, score: i64) -> Result<Option<PreparedTemp>> {
        let mut conn = self.pool.get().await?;

        if let Some(res) = conn.zrevrange(key, 0, 0).await?.first() {
            return Ok(Some(serde_json::from_str(res)?));
        }

        Ok(None)
    }
}
// impl<K, V> Cache<K, V> for RedisCache
// where
//     K: redis::ToRedisArgs + Sync + Send,
//     V: serde::Serialize + serde::de::DeserializeOwned + Sync + Send,
// {
//     async fn set(&self, name: K, value: V, ttl: i64, score: i64) -> Result<()> {
//         let mut conn = self.pool.get().await?;
//         let encoded_value = serde_json::to_string(&value)?;
//
//         conn.zadd(&name, encoded_value, score).await?;
//         conn.expire(&name, ttl).await?;
//
//         Ok(())
//     }
//     async fn get(&self, key: &K, current: i64, score: i64) -> Result<Option<V>> {
//         let mut conn = self.pool.get().await?;
//
//         let results = conn.zrevrange(key, 0, 0).await?;
//
//         if let Some(json) = results.first() {
//             let value: V = serde_json::from_str(&json)?;
//         }
//
//         let json: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
//
//         match json {
//             Some(s) => Ok(Some(serde_json::from_str(&s)?)),
//             None => Ok(None),
//         }
//     }
// }
