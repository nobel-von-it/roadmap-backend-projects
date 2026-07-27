use super::*;
use tempfile::tempdir;
use tokio::fs as async_fs;

// ============================================================================
// 1. Tests for Cache Trait Implementation on NoteManager
// ============================================================================

#[tokio::test]
async fn test_cache_load_auto_creates_file_if_missing() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    let cache_file = root.join(".cache.json");
    assert!(!cache_file.exists(), "Cache file should not exist initially");

    manager.load_cache().await?;

    assert!(cache_file.exists(), "Cache file should be created after load_cache()");
    let content = async_fs::read_to_string(cache_file).await?;
    assert_eq!(content, "{}", "Newly created cache should contain empty JSON object");
    assert!(manager.cache().is_empty());
    Ok(())
}

#[tokio::test]
async fn test_cache_manual_create_and_save() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let manager = NoteManager::new(root.clone());

    manager.save_cache().await?;

    let cache_file = root.join(".cache.json");
    assert!(cache_file.exists());
    Ok(())
}

#[tokio::test]
async fn test_cache_load_existing_valid_data() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let cache_file = root.join(".cache.json");

    let test_id = Uuid::new_v4();
    let test_path = root.join("my_note.md");

    let mut initial_map = HashMap::new();
    initial_map.insert(test_id, test_path.clone());
    let serialized = serde_json::to_string(&initial_map)?;
    async_fs::write(&cache_file, serialized).await?;

    let mut manager = NoteManager::new(root);
    manager.load_cache().await?;

    assert_eq!(manager.cache().len(), 1);
    assert_eq!(manager.cache().get(&test_id), Some(&test_path));
    Ok(())
}

#[tokio::test]
async fn test_cache_load_corrupted_json_returns_error() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let cache_file = root.join(".cache.json");

    async_fs::write(&cache_file, "{ invalid_json_data: ").await?;

    let mut manager = NoteManager::new(root);
    let result = manager.load_cache().await;

    assert!(result.is_err(), "Loading corrupted JSON must return an Error");
    Ok(())
}

#[tokio::test]
async fn test_cache_clear_resets_map_and_file() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    manager
        .create_note(&root, "note_to_clear".to_string(), Some("content".to_string()))
        .await?;
    assert_eq!(manager.cache().len(), 1);

    manager.clear_cache().await?;

    assert!(manager.cache().is_empty(), "In-memory cache should be cleared");

    let cache_content = async_fs::read_to_string(root.join(".cache.json")).await?;
    assert_eq!(cache_content.trim(), "{}", "Cache file on disk should be reset to empty object");
    Ok(())
}

// ============================================================================
// 2. Tests for NoteOperation Trait Implementation (Create, Get, Edit, Move, Delete)
// ============================================================================

#[tokio::test]
async fn test_create_note_success() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    let note = manager
        .create_note(&root, "hello_world".to_string(), Some("# Hello World".to_string()))
        .await?;

    assert_eq!(note.title(), "hello_world");
    assert_eq!(note.content(), "# Hello World");
    assert_eq!(note.path(), root.join("hello_world.md"));
    assert!(note.path().exists(), "File must exist on disk");

    let disk_content = async_fs::read_to_string(note.path()).await?;
    assert_eq!(disk_content, "# Hello World");

    assert_eq!(manager.cache().len(), 1);
    assert_eq!(manager.cache().get(note.id()), Some(&note.path));
    Ok(())
}

#[tokio::test]
async fn test_create_note_with_none_content() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    let note = manager.create_note(&root, "empty_note".to_string(), None).await?;

    assert_eq!(note.content(), "");
    let disk_content = async_fs::read_to_string(note.path()).await?;
    assert_eq!(disk_content, "");
    Ok(())
}

#[tokio::test]
async fn test_create_note_in_nested_directory() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let nested_dir = root.join("folder").join("subfolder");
    let mut manager = NoteManager::new(root.clone());

    let note = manager
        .create_note(&nested_dir, "nested_note".to_string(), Some("deep content".to_string()))
        .await?;

    assert!(nested_dir.exists(), "Nested directory should be created automatically");
    assert!(note.path().exists());
    assert_eq!(note.path(), nested_dir.join("nested_note.md"));
    Ok(())
}

#[tokio::test]
async fn test_create_note_already_exists_error() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    manager
        .create_note(&root, "duplicate".to_string(), Some("first".to_string()))
        .await?;

    let result = manager
        .create_note(&root, "duplicate".to_string(), Some("second".to_string()))
        .await;

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("path already exists"));
    Ok(())
}

#[tokio::test]
async fn test_create_note_persists_to_cache_on_disk() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    
    let note_id = {
        let mut manager = NoteManager::new(root.clone());
        let note = manager
            .create_note(&root, "persisted".to_string(), Some("content".to_string()))
            .await?;
        *note.id()
    };

    let mut new_manager = NoteManager::new(root.clone());
    new_manager.load_cache().await?;

    assert_eq!(new_manager.cache().len(), 1);
    assert_eq!(new_manager.cache().get(&note_id), Some(&root.join("persisted.md")));

    let loaded_note = new_manager.get_note(&note_id).await?;
    assert_eq!(loaded_note.title(), "persisted");
    assert_eq!(loaded_note.content(), "content");
    Ok(())
}

#[tokio::test]
async fn test_get_note_success() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    let created = manager
        .create_note(&root, "fetch_me".to_string(), Some("some data".to_string()))
        .await?;

    let fetched = manager.get_note(created.id()).await?;
    assert_eq!(fetched.id(), created.id());
    assert_eq!(fetched.title(), "fetch_me");
    assert_eq!(fetched.content(), "some data");
    assert_eq!(fetched.path(), created.path());
    Ok(())
}

#[tokio::test]
async fn test_get_note_non_existent_uuid_fails() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let manager = NoteManager::new(root);

    let random_id = Uuid::new_v4();
    let result = manager.get_note(&random_id).await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("note not found"));
    Ok(())
}

#[tokio::test]
async fn test_get_note_missing_file_on_disk_fails() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    let note = manager
        .create_note(&root, "temp_file".to_string(), Some("text".to_string()))
        .await?;

    async_fs::remove_file(note.path()).await?;

    let result = manager.get_note(note.id()).await;
    assert!(result.is_err(), "Reading a deleted file from disk should fail");
    Ok(())
}

#[tokio::test]
async fn test_edit_note_success() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    let note = manager
        .create_note(&root, "editable".to_string(), Some("v1 content".to_string()))
        .await?;

    let edited = manager.edit_note(note.id(), "v2 content updated".to_string()).await?;

    assert_eq!(edited.content(), "v2 content updated");
    assert_eq!(edited.title(), "editable");
    assert_eq!(edited.id(), note.id());

    let disk_content = async_fs::read_to_string(note.path()).await?;
    assert_eq!(disk_content, "v2 content updated");
    Ok(())
}

#[tokio::test]
async fn test_edit_note_non_existent_id_fails() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root);

    let random_id = Uuid::new_v4();
    let result = manager.edit_note(&random_id, "new text".to_string()).await;

    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_move_note_rename_in_same_directory() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    let original = manager
        .create_note(&root, "old_title".to_string(), Some("content".to_string()))
        .await?;

    let moved = manager
        .move_note(original.id(), &root, Some("new_title".to_string()))
        .await?;

    assert_eq!(moved.title(), "new_title");
    assert_eq!(moved.path(), root.join("new_title.md"));
    assert!(!original.path().exists(), "Old file path should no longer exist");
    assert!(moved.path().exists(), "New file path must exist");

    assert_eq!(manager.cache().get(original.id()), Some(&moved.path));

    let fetched = manager.get_note(original.id()).await?;
    assert_eq!(fetched.title(), "new_title");
    Ok(())
}

#[tokio::test]
async fn test_move_note_relocate_to_new_directory() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let target_subfolder = root.join("archive");
    let mut manager = NoteManager::new(root.clone());

    let original = manager
        .create_note(&root, "stay_named".to_string(), Some("archived note".to_string()))
        .await?;

    let moved = manager
        .move_note(original.id(), &target_subfolder, None)
        .await?;

    assert_eq!(moved.title(), "stay_named");
    assert_eq!(moved.path(), target_subfolder.join("stay_named.md"));
    assert!(target_subfolder.exists(), "Target subdirectory should be auto-created");
    assert!(!original.path().exists());
    assert!(moved.path().exists());
    Ok(())
}

#[tokio::test]
async fn test_move_note_relocate_and_rename_simultaneously() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let target_subfolder = root.join("docs");
    let mut manager = NoteManager::new(root.clone());

    let original = manager
        .create_note(&root, "draft".to_string(), Some("draft body".to_string()))
        .await?;

    let moved = manager
        .move_note(original.id(), &target_subfolder, Some("published".to_string()))
        .await?;

    assert_eq!(moved.title(), "published");
    assert_eq!(moved.path(), target_subfolder.join("published.md"));
    assert!(!original.path().exists());
    assert!(moved.path().exists());
    Ok(())
}

#[tokio::test]
async fn test_move_note_target_exists_fails() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    let note1 = manager
        .create_note(&root, "note1".to_string(), Some("c1".to_string()))
        .await?;
    let _note2 = manager
        .create_note(&root, "note2".to_string(), Some("c2".to_string()))
        .await?;

    let result = manager
        .move_note(note1.id(), &root, Some("note2".to_string()))
        .await;

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("target path already exists"));
    Ok(())
}

#[tokio::test]
async fn test_delete_note_success() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    let note = manager
        .create_note(&root, "to_delete".to_string(), Some("bye".to_string()))
        .await?;

    let deleted = manager.delete_note(note.id()).await?;

    assert_eq!(deleted.id(), note.id());
    assert!(!note.path().exists(), "File should be deleted from disk");
    assert_eq!(manager.cache().get(note.id()), None, "ID should be removed from cache");

    let get_result = manager.get_note(note.id()).await;
    assert!(get_result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_delete_note_non_existent_uuid_fails() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root);

    let random_id = Uuid::new_v4();
    let result = manager.delete_note(&random_id).await;

    assert!(result.is_err());
    Ok(())
}

// ============================================================================
// 3. Tests for NoteFSManager & NoteFS Trait
// ============================================================================

#[tokio::test]
async fn test_notefs_create_and_get_content() -> Result<()> {
    let fs_mgr = NoteFSManager;
    let dir = tempdir()?;
    let dir_path = dir.path().to_path_buf();

    fs_mgr.create(dir_path.clone(), "test_notefs".to_string(), "hello fs".to_string()).await?;

    let file_path = dir_path.join("test_notefs.md");
    assert!(file_path.exists());

    let content_by_title = fs_mgr.get_content_by_title(&dir_path, "test_notefs").await?;
    assert_eq!(content_by_title, "hello fs");

    let content_by_path = fs_mgr.get_content_by_path(&file_path).await?;
    assert_eq!(content_by_path, "hello fs");
    Ok(())
}

#[tokio::test]
async fn test_notefs_scan_root_dir() -> Result<()> {
    let fs_mgr = NoteFSManager;
    let dir = tempdir()?;
    let dir_path = dir.path().to_path_buf();

    fs_mgr.create(dir_path.clone(), "note1".to_string(), "c1".to_string()).await?;
    fs_mgr.create(dir_path.clone(), "note2".to_string(), "c2".to_string()).await?;
    async_fs::write(dir_path.join("ignore.txt"), "not a markdown file").await?;

    let found = fs_mgr.scan_root_dir(dir_path.to_str().unwrap()).await;

    assert_eq!(found.len(), 2, "Should only scan .md files");
    let filenames: Vec<String> = found.iter().map(|p| p.file_name().unwrap().to_str().unwrap().to_string()).collect();
    assert!(filenames.contains(&"note1.md".to_string()));
    assert!(filenames.contains(&"note2.md".to_string()));
    assert!(!filenames.contains(&"ignore.txt".to_string()));
    Ok(())
}

// ============================================================================
// 4. Helper Function Tests
// ============================================================================

#[test]
fn test_get_file_name_helper() -> Result<()> {
    assert_eq!(get_file_name("/path/to/my_file.md")?, "my_file");
    assert_eq!(get_file_name("my_file.md")?, "my_file");
    assert_eq!(get_file_name(PathBuf::from("/a/b/c/doc.md"))?, "doc");
    assert!(get_file_name("/").is_err());
    Ok(())
}
