use anyhow::{Result, anyhow};
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio::fs;
use uuid::Uuid;

pub trait Cache {
    async fn load_cache(&mut self) -> Result<()>;
    async fn create_cache(&mut self) -> Result<()>;
    async fn save_cache(&self) -> Result<()>;
    async fn clear_cache(&mut self) -> Result<()>;
}

pub trait NoteOperation {
    async fn create_note(
        &mut self,
        dir: &Path,
        title: String,
        content: Option<String>,
    ) -> Result<Note>;
    async fn delete_note(&mut self, id: &Uuid) -> Result<Note>;
    async fn move_note(&mut self, id: &Uuid, dir: &Path, title: Option<String>) -> Result<Note>;
    async fn edit_note(&mut self, id: &Uuid, content: String) -> Result<Note>;
    async fn get_note(&self, id: &Uuid) -> Result<Note>;
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub path: PathBuf,
    pub title: String,
    pub content: String,
}

impl Note {
    pub fn new(id: Uuid, path: PathBuf, title: String, content: String) -> Self {
        Self {
            id,
            path,
            title,
            content,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Debug, Clone, Default)]
pub struct NoteManager {
    root: PathBuf,
    cache: HashMap<Uuid, PathBuf>,
}

impl NoteManager {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            cache: HashMap::new(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn cache(&self) -> &HashMap<Uuid, PathBuf> {
        &self.cache
    }
}

impl Cache for NoteManager {
    async fn load_cache(&mut self) -> Result<()> {
        let path = self.root.join(".cache.json");
        if !path.exists() {
            tracing::info!("cache file not found, creating");
            self.create_cache().await?;
            return Ok(());
        }
        let content = fs::read_to_string(path).await?;
        let cache: HashMap<Uuid, PathBuf> = serde_json::from_str(&content)?;
        self.cache = cache;
        Ok(())
    }

    async fn create_cache(&mut self) -> Result<()> {
        let path = self.root.join(".cache.json");
        if !self.root.exists() {
            fs::create_dir_all(&self.root).await?;
        }
        fs::write(path, "{}").await?;
        Ok(())
    }

    async fn save_cache(&self) -> Result<()> {
        let path = self.root.join(".cache.json");
        if !self.root.exists() {
            fs::create_dir_all(&self.root).await?;
        }
        let content = serde_json::to_string_pretty(&self.cache)?;
        fs::write(path, content).await?;
        Ok(())
    }

    async fn clear_cache(&mut self) -> Result<()> {
        self.cache.clear();
        self.save_cache().await?;
        Ok(())
    }
}

impl NoteOperation for NoteManager {
    async fn create_note(
        &mut self,
        dir: &Path,
        title: String,
        content: Option<String>,
    ) -> Result<Note> {
        let id = Uuid::new_v4();
        let path = dir.join(format!("{}.md", title));
        if path.exists() {
            return Err(anyhow!("path already exists"));
        }
        let content = content.unwrap_or_default();
        fs::create_dir_all(dir).await?;
        fs::write(&path, &content).await?;
        self.cache.insert(id, path.clone());
        self.save_cache().await?;
        Ok(Note {
            id,
            path,
            title,
            content,
        })
    }

    async fn delete_note(&mut self, id: &Uuid) -> Result<Note> {
        let note = self.get_note(id).await?;
        fs::remove_file(&note.path).await?;
        self.cache.remove(id);
        self.save_cache().await?;
        Ok(note)
    }

    async fn move_note(&mut self, id: &Uuid, dir: &Path, title: Option<String>) -> Result<Note> {
        let note = self.get_note(id).await?;
        let new_title = title.unwrap_or(note.title.clone());
        let new_path = dir.join(format!("{}.md", new_title));

        if !dir.exists() {
            fs::create_dir_all(dir).await?;
        }

        if new_path.exists() && new_path != note.path {
            return Err(anyhow!("target path already exists"));
        }

        fs::rename(&note.path, &new_path).await?;
        self.cache.insert(*id, new_path.clone());
        self.save_cache().await?;

        Ok(Note {
            id: note.id,
            path: new_path,
            title: new_title,
            content: note.content,
        })
    }

    async fn edit_note(&mut self, id: &Uuid, content: String) -> Result<Note> {
        let note = self.get_note(id).await?;
        fs::write(note.path.clone(), &content).await?;
        Ok(Note {
            id: note.id,
            path: note.path,
            title: note.title,
            content,
        })
    }

    async fn get_note(&self, id: &Uuid) -> Result<Note> {
        let path = self.cache.get(id).ok_or(anyhow!("note not found"))?;
        let title = get_file_name(path)?;
        let content = fs::read_to_string(path).await?;
        Ok(Note {
            id: *id,
            path: path.clone(),
            title,
            content,
        })
    }
}

pub trait NoteFS {
    async fn create(&self, path_to_dir: PathBuf, title: String, content: String) -> Result<()>;
    async fn get_content_by_title(&self, path_to_dir: &PathBuf, title: &str) -> Result<String>;
    async fn get_content_by_path(&self, path_to_file: &PathBuf) -> Result<String> {
        let path_to_dir = path_to_file.parent().ok_or(anyhow!("no parent"))?;
        let title = get_file_name(path_to_file)?;
        self.get_content_by_title(&path_to_dir.to_path_buf(), &title)
            .await
    }
    async fn scan_root_dir(&self, root_path: &str) -> Vec<PathBuf> {
        let root_path = PathBuf::from(root_path);
        walkdir::WalkDir::new(root_path)
            .follow_links(true)
            .into_iter()
            .flatten()
            .filter(|entry| {
                entry.path().is_file() && entry.path().extension() == Some(OsStr::new("md"))
            })
            .map(|entry| entry.path().to_path_buf())
            .collect()
    }
}

impl NoteFS for NoteManager {
    async fn create(&self, path_to_dir: PathBuf, title: String, content: String) -> Result<()> {
        let file = path_to_dir.join(format!("{}.md", title));
        if file.exists() {
            return Err(anyhow!("path already exists"));
        }
        fs::create_dir_all(&path_to_dir).await?;
        fs::write(file, content).await?;
        Ok(())
    }

    async fn get_content_by_title(&self, path_to_dir: &PathBuf, title: &str) -> Result<String> {
        let file = path_to_dir.join(format!("{}.md", title));
        Ok(fs::read_to_string(file).await?)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NoteFSManager;

impl NoteFS for NoteFSManager {
    async fn create(&self, path_to_dir: PathBuf, title: String, content: String) -> Result<()> {
        let file = path_to_dir.join(format!("{}.md", title));
        if file.exists() {
            return Err(anyhow!("path already exists"));
        }
        fs::create_dir_all(&path_to_dir).await?;
        fs::write(file, content).await?;
        Ok(())
    }

    async fn get_content_by_title(&self, path_to_dir: &PathBuf, title: &str) -> Result<String> {
        let file = path_to_dir.join(format!("{}.md", title));
        Ok(fs::read_to_string(file).await?)
    }
}

pub fn get_file_name<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    Ok(path
        .file_name()
        .ok_or(anyhow!("no file name"))?
        .to_str()
        .ok_or(anyhow!("no file name"))?
        .to_string()
        .trim_end_matches(".md")
        .to_string())
}

#[cfg(test)]
mod tests;
