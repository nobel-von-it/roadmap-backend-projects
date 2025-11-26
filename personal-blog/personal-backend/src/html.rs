use std::{collections::HashMap, fs};

use anyhow::{Result, anyhow};
use axum::response::Html;
use tinytemplate::TinyTemplate;

use crate::models::{Article, Articles};

#[derive(Debug, Clone)]
pub struct HtmlManager {
    preloads: HashMap<String, String>,
}

impl HtmlManager {
    pub fn load() -> Result<Self> {
        let preloads = fs::read_dir("html")?
            .flatten()
            .filter_map(|e| {
                let name = match e.file_name().to_str() {
                    Some(name) => name.to_string(),
                    None => return None,
                };
                let content = match fs::read_to_string(e.path()) {
                    Ok(content) => content,
                    Err(_) => return None,
                };
                Some((name, content))
            })
            .collect::<HashMap<_, _>>();

        if preloads.is_empty() {
            return Err(anyhow::anyhow!("no html files found"));
        }
        if preloads.len() != 5 {
            return Err(anyhow::anyhow!("not all html files found"));
        }

        Ok(HtmlManager { preloads })
    }

    pub fn article_html(&self, article: &Article) -> Result<Html<String>> {
        if let Some(html) = self.preloads.get("article.html") {
            let mut tt = TinyTemplate::new();
            tt.add_template("article", html)?;
            return Ok(Html(tt.render("article", article)?));
        }
        Err(anyhow!("article.html not found"))
    }
    pub fn new_article_html(&self) -> Result<Html<String>> {
        if let Some(html) = self.preloads.get("new_article.html") {
            return Ok(Html(html.to_string()));
        }
        Err(anyhow!("new_article.html not found"))
    }
    pub fn edit_article_html(&self, article: &Article) -> Result<Html<String>> {
        if let Some(html) = self.preloads.get("edit_article.html") {
            let mut tt = TinyTemplate::new();
            tt.add_template("edit_article", html)?;
            return Ok(Html(tt.render("edit_article", article)?));
        }
        Err(anyhow!("edit_article.html not found"))
    }

    pub fn home_html(&self, articles: &Articles) -> Result<Html<String>> {
        if let Some(html) = self.preloads.get("home.html") {
            let mut tt = TinyTemplate::new();
            tt.add_template("home_articles", html)?;
            return Ok(Html(tt.render("home_articles", articles)?));
        }
        Err(anyhow!("home.html not found"))
    }
    pub fn admin_html(&self, articles: &Articles) -> Result<Html<String>> {
        if let Some(html) = self.preloads.get("admin.html") {
            let mut tt = TinyTemplate::new();
            tt.add_template("admin_articles", html)?;
            return Ok(Html(tt.render("admin_articles", articles)?));
        }
        Err(anyhow!("admin.html not found"))
    }
}
