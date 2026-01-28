use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    Form, Json, Router,
    extract::State,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::{get, post},
};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

struct SqliteService {
    pool: deadpool_sqlite::Pool,
}

impl SqliteService {
    fn new() -> Result<Self> {
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("posts.db");
        log::info!("open database with path: {}", db_path.display());
        let config = deadpool_sqlite::Config::new(db_path);
        let pool = config.create_pool(deadpool_sqlite::Runtime::Tokio1)?;

        Ok(Self { pool })
    }

    async fn init_db(&self) -> Result<()> {
        let conn = self.pool.get().await?;
        conn.interact(|conn| -> Result<()> {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS posts (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    content TEXT NOT NULL,
                    category TEXT NOT NULL,
                    tags TEXT NOT NULL,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
            )",
                (),
            )?;

            Ok(())
        })
        .await
        .map_err(|e| anyhow::anyhow!("interact error: {}", e))??;

        Ok(())
    }

    async fn create_post(&self, new_post: BlogPost) -> Result<DBBlogPost> {
        if !validate_post(&new_post) {
            return Err(anyhow::anyhow!("new post has invalid data"));
        }

        let conn = self.pool.get().await?;

        let result = conn.interact(move |conn| -> Result<DBBlogPost> {
            let now = Local::now();
            let now_str = now.to_rfc3339();
            let tags_str = new_post.tags.join(", ");


            conn.execute(
                "INSERT INTO posts (title, content, category, tags, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&new_post.title, &new_post.content, &new_post.category, &tags_str, &now_str, &now_str)
            )?;

            let id = conn.last_insert_rowid() as u64;

            Ok(DBBlogPost{
                id,
                title: new_post.title,
                content: new_post.content,
                category: new_post.category,
                tags: new_post.tags,
                created_at: now,
                updated_at: now
            })
        }).await.map_err(|e| anyhow::anyhow!("interact error: {}", e))??;

        Ok(result)
    }

    async fn select_all_posts(&self) -> Result<Vec<DBBlogPost>> {
        let conn = self.pool.get().await?;

        let result = conn
            .interact(|conn| -> Result<Vec<RawDBBlogPost>> {
                let mut stmt = conn.prepare("SELECT * FROM posts")?;
                let posts_iter = stmt.query_map([], |row| {
                    let id = row.get(0)?;
                    let title = row.get(1)?;
                    let content = row.get(2)?;
                    let category = row.get(3)?;
                    let tags = row.get(4)?;
                    let created_at = row.get(5)?;
                    let updated_at = row.get(6)?;

                    Ok(RawDBBlogPost {
                        id,
                        title,
                        content,
                        category,
                        tags,
                        created_at,
                        updated_at,
                    })
                })?;

                posts_iter
                    .map(|post| post.map_err(|e| anyhow::anyhow!("post error: {}", e)))
                    .collect::<Result<Vec<RawDBBlogPost>>>()
            })
            .await
            .map_err(|e| anyhow::anyhow!("interact error: {}", e))??;

        result
            .into_iter()
            .map(DBBlogPost::any_from)
            .collect::<Result<Vec<DBBlogPost>>>()
    }

    async fn delete_post(&self, id: u64) -> Result<()> {
        let conn = self.pool.get().await?;

        conn.interact(move |conn| -> Result<()> {
            conn.execute("DELETE FROM posts WHERE id = ?1", [id])?;
            Ok(())
        })
        .await
        .map_err(|e| anyhow::anyhow!("interact error: {}", e))??;

        Ok(())
    }
}

fn validate_post(post: &BlogPost) -> bool {
    let not_empty_is_ascii = |s: &str| -> bool { !s.is_empty() && s.is_ascii() };

    not_empty_is_ascii(&post.title)
        && not_empty_is_ascii(&post.content)
        && not_empty_is_ascii(&post.category)
        && post.tags.iter().filter(|s| not_empty_is_ascii(s)).count() == post.tags.len()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawDBBlogPost {
    id: u64,
    title: String,
    content: String,
    category: String,
    tags: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DBBlogPost {
    id: u64,
    title: String,
    content: String,
    category: String,
    tags: Vec<String>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl DBBlogPost {
    fn any_from(post: RawDBBlogPost) -> Result<Self> {
        let tags = post.tags.split(", ").map(String::from).collect();
        let created_at = post.created_at.parse()?;
        let updated_at = post.updated_at.parse()?;

        Ok(Self {
            id: post.id,
            title: post.title,
            content: post.content,
            category: post.category,
            tags,
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BlogPost {
    title: String,
    content: String,
    category: String,
    tags: Vec<String>,
}

async fn new_post(
    State(db): State<Arc<SqliteService>>,
    Json(new_post): Json<BlogPost>,
) -> impl IntoResponse {
    // id autoincrement
    // time in business logic (chrono)
    // TODO: "DB insert "

    log::info!("user send new_post: {:#?}", &new_post);
    match db.create_post(new_post).await {
        Ok(created_post) => (StatusCode::CREATED, Json(created_post)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn get_post(uri: Uri) -> impl IntoResponse {
    let (_, id) = uri.path().split_once("/").unwrap();
    let id: u64 = id.parse().unwrap();

    // TODO: "DB select where id = {id}"
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn get_all_posts(State(db): State<Arc<SqliteService>>) -> impl IntoResponse {
    // TODO: "DB select *"

    match db.select_all_posts().await {
        Ok(posts) => (StatusCode::OK, Json(posts)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn put_post(uri: Uri, Form(put_post): Form<BlogPost>) -> impl IntoResponse {
    let (_, id) = uri.path().split_once("/").unwrap();
    let id: u64 = id.parse().unwrap();

    // TODO: "DB update where id = {id}"
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn delete_post(State(db): State<Arc<SqliteService>>, uri: Uri) -> impl IntoResponse {
    let id = uri.path().split("/").last().unwrap();
    log::info!("delete post with id {id}");
    let id: u64 = id.trim().parse().unwrap();

    // TODO: "DB delete where id = {id}"
    match db.delete_post(id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // dotenvy::dotenv()?;
    env_logger::init();

    let static_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");

    let db = SqliteService::new()?;
    db.init_db().await?;

    let db = Arc::new(db); // TODO: implement db service

    let router = Router::new()
        .route("/posts", post(new_post).get(get_all_posts))
        .route(
            "/posts/{id}",
            get(get_post).put(put_post).delete(delete_post),
        )
        .nest_service("/static", ServeDir::new(static_path))
        .with_state(db);

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, router).await?;

    Ok(())
}
