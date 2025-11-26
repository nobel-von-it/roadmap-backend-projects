use anyhow::Result;
use axum::{
    Form, Router,
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode, header::WWW_AUTHENTICATE},
    response::{Html, IntoResponse},
    routing::get,
};
use axum_auth::AuthBasic;
use axum_extra::TypedHeader;
use base64::Engine;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
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

static ARTICLE_EDIT_TEMPLATE: &'static str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Edit Arcticle - { title }</title>
    </head>
    <body>
    <div id="container">
        <h1>Edit Arcticle - { title }</h1>
        <form action="/edit/{ id }" method="POST">
            <label for="title">Title</label>
            <input type="text" name="title" id="title" value="{ title }">
            <label for="date">Date</label>
            <input type="text" name="date" id="date" value="{ date }">
            <label for="content">Content</label>
            <textarea name="content" id="content">{ content }</textarea>
            <button type="submit">Update</button>
        </form>
    </div>
    </body>
</html>
"#;

static ARTICLES_HOME_TEMPLATE: &'static str = r#"
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

static ARTICLES_ADMIN_TEMPLATE: &'static str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Personal Admin Posts</title>
    </head>
    <body>
        <div id="container">
            <h1>Articles</h1>
            <ul id="articles-list">
            {{ for article in articles }}
                <li class="article-mini">
                    <a href="/article/{ article.id }">{ article.title }</a>
                    <div>{ article.date }</div>
                    <a href="/edit/{ article.id }">edit</a>
                    <a href="/delete/{ article.id }">delete</a>
                </li>
            {{ endfor }}
            </ul>
        </div>
    </body>
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
    fn get_home_html(&self) -> Result<Html<String>> {
        let mut tt = TinyTemplate::new();
        tt.add_template("articles", ARTICLES_HOME_TEMPLATE)?;

        let rendered = tt.render("articles", &self)?;

        Ok(Html(rendered))
    }
    fn get_admin_html(&self) -> Result<Html<String>> {
        let mut tt = TinyTemplate::new();
        tt.add_template("articles", ARTICLES_ADMIN_TEMPLATE)?;

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

#[derive(Debug, Deserialize)]
struct FormArticle {
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
    fn update(&self) -> Result<()> {
        let conn = Connection::open("blog.sqlite")?;
        let mut stmt =
            conn.prepare("UPDATE articles SET title = ?1, date = ?2, content = ?3 WHERE id = ?4;")?;
        stmt.execute(&[&self.title, &self.date, &self.content, &self.id.to_string()])?;

        Ok(())
    }
    fn get_html(&self) -> Result<Html<String>> {
        let mut tt = TinyTemplate::new();
        tt.add_template("article", ARTICLE_FULL_TEPLATE)?;

        let rendered = tt.render("article", &self)?;

        Ok(Html(rendered))
    }
    fn get_edit_html(&self) -> Result<Html<String>> {
        let mut tt = TinyTemplate::new();
        tt.add_template("article", ARTICLE_EDIT_TEMPLATE)?;

        let rendered = tt.render("article", &self)?;

        Ok(Html(rendered))
    }
}

impl From<(i32, FormArticle)> for Article {
    fn from(value: (i32, FormArticle)) -> Self {
        let (id, form) = value;
        Self {
            id,
            title: form.title,
            date: form.date,
            content: form.content,
        }
    }
}

async fn handle_articles() -> impl IntoResponse {
    let articles = Articles::from_db().unwrap();
    let html = articles.get_home_html().unwrap();
    html
}

async fn handle_article(url: axum::http::Uri) -> impl IntoResponse {
    // TODO: Handle errors
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

async fn handle_protected_admin_panel(req: Request<Body>) -> impl IntoResponse {
    if let Some((user, pass)) = parse_basic_auth(req.headers()).await {
        if user == "admin" && pass == "1234" {
            let articles = Articles::from_db().unwrap();
            let html = articles.get_admin_html().unwrap();
            return html.into_response();
        }
    }

    (
        StatusCode::UNAUTHORIZED,
        [(
            axum::http::header::WWW_AUTHENTICATE,
            r#"Basic realm="MyRealm", charset="UTF-8""#,
        )],
        "Unauthorized",
    )
        .into_response()
}

async fn parse_basic_auth(headers: &HeaderMap) -> Option<(String, String)> {
    let value = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    if !value.starts_with("Basic ") {
        return None;
    }

    let encoded = &value[6..];
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .ok()?;
    let s = String::from_utf8(decoded).ok()?;

    let mut parts = s.splitn(2, ':');
    Some((parts.next()?.to_string(), parts.next()?.to_string()))
}

async fn handle_get_edit_article(url: axum::http::Uri) -> impl IntoResponse {
    // TODO: Handle errors
    let idx = url
        .path()
        .split("/")
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let article = Article::from_db(idx).unwrap();
    let html = article.get_edit_html().unwrap();
    html
}

async fn handle_post_edit_article(
    url: axum::http::Uri,
    Form(updated_article): Form<FormArticle>,
) -> impl IntoResponse {
    // TODO: Handle errors
    let idx = url
        .path()
        .split("/")
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let article = Article::from((idx, updated_article));
    article.update().unwrap();

    (StatusCode::TEMPORARY_REDIRECT, [("Location", "/admin")]).into_response()
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/article/{id}", get(handle_article))
        .route(
            "/edit/{id}",
            get(handle_get_edit_article).post(handle_post_edit_article),
        )
        .route("/home", get(handle_articles))
        .route("/admin", get(handle_protected_admin_panel));
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
