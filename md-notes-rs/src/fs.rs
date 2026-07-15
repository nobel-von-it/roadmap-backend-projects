use anyhow::{Result, anyhow};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio::fs;

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

#[derive(Clone, Copy)]
pub struct NoteFSManager;
impl NoteFS for NoteFSManager {
    async fn create(&self, path_to_dir: PathBuf, title: String, content: String) -> Result<()> {
        let path = path_to_dir.canonicalize()?;
        let file = path.join(format!("{}.md", title));
        if file.exists() {
            return Err(anyhow!("path already exists"));
        }
        fs::create_dir_all(&path).await?;
        fs::write(file, content).await?;
        Ok(())
    }
    async fn get_content_by_title(&self, path_to_dir: &PathBuf, title: &str) -> Result<String> {
        let path = path_to_dir.canonicalize()?;
        let file = path.join(format!("{}.md", title));
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
mod tests {
    use tempfile::{tempdir, tempfile};

    use super::*;

    #[tokio::test]
    async fn test_create() -> Result<()> {
        let fs = NoteFSManager;
        let path = tempdir()?;
        fs.create(
            path.path().to_path_buf(),
            "test".to_string(),
            "test content".to_string(),
        )
        .await?;
        assert!(path.path().join("test.md").exists());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_content_by_title() -> Result<()> {
        let fs = NoteFSManager;
        let path = tempdir()?;
        fs.create(
            path.path().to_path_buf(),
            "test".to_string(),
            "test content".to_string(),
        )
        .await?;
        let content = fs
            .get_content_by_title(&path.path().to_path_buf(), "test")
            .await?;
        assert_eq!(content, "test content");
        Ok(())
    }

    #[tokio::test]
    async fn test_get_content_by_path() -> Result<()> {
        let fs = NoteFSManager;
        let path = tempdir()?;
        fs.create(
            path.path().to_path_buf(),
            "test".to_string(),
            "test content".to_string(),
        )
        .await?;
        let content = fs.get_content_by_path(&path.path().join("test.md")).await?;
        assert_eq!(content, "test content");
        Ok(())
    }

    #[tokio::test]
    async fn test_scan_root_dir() -> Result<()> {
        let fs = NoteFSManager;
        let path = tempdir()?;
        fs.create(
            path.path().to_path_buf(),
            "test".to_string(),
            "test content".to_string(),
        )
        .await?;
        let notes = fs.scan_root_dir(path.path().to_str().unwrap()).await;
        println!("{:?}", notes);
        let title = get_file_name(path.path().join("test.md"))?;
        println!("{}", title);
        Ok(())
    }
}
