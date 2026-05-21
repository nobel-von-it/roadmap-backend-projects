mod jwt {
    use std::time::Duration;

    use anyhow::Result;
    use chrono::Utc;
    use serde::Serialize;

    pub struct Jwt {
        token: String,
    }

    impl Jwt {
        pub fn create(id: i32) -> Result<Self> {
            let secret = std::env::var("SECRET").expect("SECRET not found");
            let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());

            #[derive(Serialize)]
            struct TmpClaims {
                user_id: i32,
                exp: usize,
            }

            let claims = TmpClaims {
                user_id: id,
                exp: (Utc::now() + Duration::from_hours(24)).timestamp() as usize,
            };
            let token = jsonwebtoken::encode(&Default::default(), &claims, &encoding_key)?;

            Ok(Jwt { token })
        }
    }

    impl ToString for Jwt {
        fn to_string(&self) -> String {
            self.token.clone()
        }
    }
}

mod database {
    use anyhow::Result;

    use crate::{UserJwtResponse, UserLog, UserReg, jwt};

    #[derive(Clone)]
    pub struct Db {
        pub pool: deadpool_sqlite::Pool,
    }

    impl Db {
        pub fn new<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
            let path = path.as_ref();

            if !path.exists() {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::File::create(path)
                    .map_err(|e| anyhow::anyhow!("Failed to create database file: {}", e))?;
            }

            let config = deadpool_sqlite::Config::new(path);
            let pool = config.create_pool(deadpool_sqlite::Runtime::Tokio1)?;
            Ok(Db { pool })
        }

        #[allow(dead_code)]
        pub async fn clear(&self) -> Result<()> {
            let conn = self.pool.get().await?;
            let _ = conn
                .interact(|conn| -> Result<()> {
                    let _ = conn.execute("DELETE FROM users", [])?;
                    let _ = conn.execute("DELETE FROM tasks", [])?;
                    Ok(())
                })
                .await
                .map_err(|e| anyhow::anyhow!("interact error: {}", e))?;

            Ok(())
        }

        pub async fn init_tables(&self) -> Result<()> {
            let conn = self.pool.get().await?;

            let _ = conn
                .interact(|conn| -> Result<()> {
                    let _ = conn.execute(
                        "CREATE TABLE IF NOT EXISTS users (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    email TEXT NOT NULL UNIQUE,
                    password TEXT NOT NULL
                )",
                        [],
                    )?;
                    let _ = conn.execute(
                        "CREATE TABLE IF NOT EXISTS tasks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    description TEXT NOT NULL,
                    user_id INTEGER NOT NULL,
                    FOREIGN KEY (user_id) REFERENCES users(id)
                )",
                        [],
                    )?;
                    Ok(())
                })
                .await
                .map_err(|e| anyhow::anyhow!("interact error: {}", e))?;

            Ok(())
        }

        pub async fn register_user(&self, user_reg: UserReg) -> Result<UserJwtResponse> {
            let conn = self.pool.get().await?;

            let id = conn
                .interact(move |conn| -> Result<i32> {
                    let _ = conn.execute(
                        "INSERT INTO users (name, email, password) VALUES (?, ?, ?)",
                        [&user_reg.name, &user_reg.email, &user_reg.password],
                    )?;

                    let mut statement = conn.prepare("SELECT id FROM users WHERE email = ?")?;
                    let mut rows = statement.query(&[&user_reg.email])?;
                    let row = rows.next()?;
                    let id = row
                        .ok_or(anyhow::anyhow!("Failed to get user ID"))?
                        .get("id")?;
                    Ok(id)
                })
                .await
                .map_err(|e| anyhow::anyhow!("interact error: {}", e))??;

            let token = jwt::Jwt::create(id)?;

            Ok(UserJwtResponse {
                token: token.to_string(),
            })
        }

        pub async fn login_user(&self, user_log: UserLog) -> Result<UserJwtResponse> {
            let conn = self.pool.get().await?;

            let id = conn
                .interact(move |conn| -> Result<i32> {
                    let mut statement = conn.prepare("SELECT id FROM users WHERE email = ?")?;
                    let mut rows = statement.query(&[&user_log.email])?;
                    let row = rows.next()?;
                    let id = row
                        .ok_or(anyhow::anyhow!("Failed to get user ID"))?
                        .get("id")?;
                    Ok(id)
                })
                .await
                .map_err(|e| anyhow::anyhow!("interact error: {}", e))??;

            let token = jwt::Jwt::create(id)?;

            Ok(UserJwtResponse {
                token: token.to_string(),
            })
        }
    }
}
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use database::Db;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
struct UserReg {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct UserJwtResponse {
    token: String,
}

#[derive(Deserialize)]
struct UserLog {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct UserLogResponse {
    token: String,
}

#[derive(Deserialize)]
struct ToDoItem {
    title: String,
    description: String,
}

#[derive(Serialize)]
struct ToDoItemResponse {
    id: i32,
    title: String,
    description: String,
}

struct ToDoListResponse {
    data: Vec<ToDoItemResponse>,
    page: i32,
    limit: i32,
    total: i32,
}

async fn register(State(db): State<Db>, Json(user_reg): Json<UserReg>) -> impl IntoResponse {
    let result = db.register_user(user_reg).await;
    match result {
        Ok(user_reg_response) => (StatusCode::OK, Json(user_reg_response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

async fn login(State(db): State<Db>, Json(user_log): Json<UserLog>) -> impl IntoResponse {
    let result = db.login_user(user_log).await;
    match result {
        Ok(user_log_response) => (StatusCode::OK, Json(user_log_response)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load environment variables");

    let db = Db::new("todo.db").expect("Failed to create database");
    db.init_tables().await.expect("Failed to initialize tables");

    let router = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("[IP_ADDRESS]")
        .await
        .expect("Failed to bind to port");
    axum::serve(listener, router)
        .await
        .expect("Failed to start server");
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    use super::*;

    static INIT: Once = Once::new();

    fn init() {
        INIT.call_once(|| {
            dotenvy::dotenv().expect("Failed to load environment variables");
        });
    }

    #[tokio::test]
    async fn test_register_user() {
        init();

        let db = Db::new("test.db").expect("Failed to create database");
        db.init_tables().await.expect("Failed to initialize tables");

        let user_reg = UserReg {
            name: "test".to_string(),
            email: "test".to_string(),
            password: "test".to_string(),
        };
        let result = db.register_user(user_reg.clone()).await;
        println!("{:?}", result);
        assert!(result.is_ok());

        db.clear().await.expect("Failed to clear database");

        let result = db.register_user(user_reg).await;
        println!("{:?}", result);
        assert!(result.is_ok());

        db.clear().await.expect("Failed to clear database");
    }

    #[tokio::test]
    async fn test_login_user() {
        init();

        let db = Db::new("test.db").expect("Failed to create database");
        db.init_tables().await.expect("Failed to initialize tables");

        let user_reg = UserReg {
            name: "test".to_string(),
            email: "test".to_string(),
            password: "test".to_string(),
        };
        let user_log = UserLog {
            email: "test".to_string(),
            password: "test".to_string(),
        };
        let _ = db.register_user(user_reg).await;
        let result = db.login_user(user_log).await;
        println!("{:?}", result);
        assert!(result.is_ok());

        db.clear().await.expect("Failed to clear database");
    }
}
