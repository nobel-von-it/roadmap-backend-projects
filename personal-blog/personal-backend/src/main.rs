use anyhow::Result;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use rusqlite::Connection;
use serde::Serialize;
use tinytemplate::TinyTemplate;
use tokio::net::TcpListener;

static ARTICLE_FULL_TEPLATE: &'static str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Arcticle - { title }</title>
    </head>
    <body>
    <div id="container">
        <h1>{ title }</h1>
        <p>{ date }</p>
        <p>{ content }</p>
    </div>
    </body>
</html>
"#;

static ARTICLES_TEPLATE: &'static str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Articles</title>
    </head>
    <body>
        <div id="container">
            <h1>Articles</h1>
            <ul id="articles-list">
            {{ for article in articles }}
                <li class="article-mini">
                    <a href="/article/{ article.id }">{ article.title }</a>
                    <div>{ article.date }</div>
                </li>
            {{ endfor }}
            </ul>
        </div>
    </body>
</html>
"#;

#[derive(Debug, Serialize)]
struct Articles {
    articles: Vec<Article>,
}

impl Articles {
    fn from_db() -> Result<Self> {
        let conn = Connection::open("blog.sqlite")?;
        let mut stmt = conn.prepare("SELECT id, title, date FROM articles;")?;
        let rows = stmt.query_map([], |row| {
            Ok(Article {
                id: row.get(0)?,
                title: row.get(1)?,
                date: row.get(2)?,
                content: String::new(),
            })
        })?;

        Ok(Articles {
            articles: rows.flatten().collect(),
        })
    }
    fn get_html(&self) -> Result<Html<String>> {
        let mut tt = TinyTemplate::new();
        tt.add_template("articles", ARTICLES_TEPLATE)?;

        let rendered = tt.render("articles", &self)?;

        Ok(Html(rendered))
    }
}

#[derive(Debug, Serialize)]
struct Article {
    id: i32,
    title: String,
    date: String,
    content: String,
}

impl Article {
    fn from_db(id: i32) -> Result<Self> {
        let conn = Connection::open("blog.sqlite")?;
        let mut stmt =
            conn.prepare("SELECT id, title, date, content FROM articles WHERE id = ?1;")?;
        let rows = stmt.query_map(&[id.to_string().as_str()], |row| {
            Ok(Self {
                id: row.get(0)?,
                title: row.get(1)?,
                date: row.get(2)?,
                content: row.get(3)?,
            })
        })?;

        for row in rows {
            return Ok(row?);
        }

        Err(anyhow::anyhow!("article not found"))
    }
    fn get_html(&self) -> Result<Html<String>> {
        let mut tt = TinyTemplate::new();
        tt.add_template("article", ARTICLE_FULL_TEPLATE)?;

        let rendered = tt.render("article", &self)?;

        Ok(Html(rendered))
    }
}

async fn handle_articles() -> impl IntoResponse {
    let articles = Articles::from_db().unwrap();
    let html = articles.get_html().unwrap();
    html
}

async fn handle_article(url: axum::http::Uri) -> impl IntoResponse {
    let idx = url
        .path()
        .split("/")
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let article = Article::from_db(idx).unwrap();
    let html = article.get_html().unwrap();
    html
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/article/{id}", get(handle_article))
        .route("/home", get(handle_articles));
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
