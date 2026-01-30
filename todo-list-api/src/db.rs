use anyhow::Result;
use std::path::Path;

use crate::{Auth, UserReg};

const USERS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
)
"#;
const REG_USER: &str = r#"
INSERT INTO users (name, email, password) VALUES (?, ?, ?)
"#;

const TASKS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
)
"#;

struct DB {
    pool: deadpool_sqlite::Pool,
}

impl DB {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = deadpool_sqlite::Config::new(path.as_ref());
        let pool = config.create_pool(deadpool_sqlite::Runtime::Tokio1)?;

        Ok(DB { pool })
    }

    pub async fn init_tables(&self) -> Result<()> {
        let conn = self.pool.get().await?;

        conn.interact(|conn| -> Result<()> {
            conn.execute(USERS_TABLE, [])?;
            conn.execute(TASKS_TABLE, [])?;
            Ok(())
        })
        .await
        .map_err(|e| anyhow::anyhow!("interact error: {}", e))??;

        Ok(())
    }
}
