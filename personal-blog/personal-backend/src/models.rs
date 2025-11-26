use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Articles {
    articles: Vec<Article>,
}

impl Articles {
    pub fn from_db() -> Result<Self> {
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
}

#[derive(Debug, Serialize)]
pub struct Article {
    id: i32,
    title: String,
    date: String,
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct FormArticle {
    title: String,
    date: String,
    content: String,
}

impl Article {
    pub fn from_db(id: i32) -> Result<Self> {
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
    pub fn insert(form_article: FormArticle) -> Result<Self> {
        let conn = Connection::open("blog.sqlite")?;
        let mut stmt =
            conn.prepare("INSERT INTO articles (title, date, content) VALUES (?1, ?2, ?3);")?;
        stmt.execute(&[
            &form_article.title,
            &form_article.date,
            &form_article.content,
        ])?;

        let mut stmt =
            conn.prepare("SELECT id, title, date, content FROM articles WHERE title = ?1;")?;
        let rows = stmt.query_map(&[&form_article.title], |row| {
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
    pub fn update(&self) -> Result<()> {
        let conn = Connection::open("blog.sqlite")?;
        let mut stmt =
            conn.prepare("UPDATE articles SET title = ?1, date = ?2, content = ?3 WHERE id = ?4;")?;
        stmt.execute(&[&self.title, &self.date, &self.content, &self.id.to_string()])?;

        Ok(())
    }
    pub fn delete(&self) -> Result<()> {
        let conn = Connection::open("blog.sqlite")?;
        let mut stmt = conn.prepare("DELETE FROM articles WHERE id = ?1;")?;
        stmt.execute(&[&self.id.to_string()])?;

        Ok(())
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
