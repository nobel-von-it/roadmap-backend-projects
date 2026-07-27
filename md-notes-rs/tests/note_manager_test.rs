use anyhow::Result;
use md_notes_rs::fs::{Cache, NoteManager, NoteOperation};
use tempfile::tempdir;
use tokio::fs as async_fs;

#[tokio::test]
async fn test_full_note_lifecycle_integration() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    // 1. Create initial note
    let note = manager
        .create_note(
            &root,
            "integration_note".to_string(),
            Some("Initial content for integration test".to_string()),
        )
        .await?;

    let note_id = *note.id();

    // 2. Read note by ID
    let fetched_1 = manager.get_note(&note_id).await?;
    assert_eq!(fetched_1.title(), "integration_note");
    assert_eq!(fetched_1.content(), "Initial content for integration test");

    // 3. Edit note content
    let edited = manager
        .edit_note(&note_id, "Updated content version 2".to_string())
        .await?;
    assert_eq!(edited.content(), "Updated content version 2");

    // 4. Move and rename note to subfolder
    let subfolder = root.join("project_notes");
    let moved = manager
        .move_note(
            &note_id,
            &subfolder,
            Some("renamed_integration_note".to_string()),
        )
        .await?;
    assert_eq!(moved.title(), "renamed_integration_note");
    assert_eq!(moved.path(), subfolder.join("renamed_integration_note.md"));

    // 5. Test persistence across manager restarts
    let mut manager_restart = NoteManager::new(root.clone());
    manager_restart.load_cache().await?;

    let fetched_restarted = manager_restart.get_note(&note_id).await?;
    assert_eq!(fetched_restarted.title(), "renamed_integration_note");
    assert_eq!(fetched_restarted.content(), "Updated content version 2");

    // 6. Delete note
    let deleted = manager_restart.delete_note(&note_id).await?;
    assert_eq!(deleted.id(), &note_id);
    assert!(!moved.path().exists(), "File must be removed from disk");

    // 7. Ensure cannot fetch deleted note
    assert!(manager_restart.get_note(&note_id).await.is_err());
    Ok(())
}

#[tokio::test]
async fn test_multiple_notes_concurrent_cache_integrity() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().to_path_buf();
    let mut manager = NoteManager::new(root.clone());

    let mut created_ids = Vec::new();

    // Create 15 notes
    for i in 0..15 {
        let title = format!("note_{:02}", i);
        let content = format!("Content of note {}", i);
        let note = manager.create_note(&root, title, Some(content)).await?;
        created_ids.push(*note.id());
    }

    assert_eq!(manager.cache().len(), 15);

    // Verify all 15 can be retrieved
    for (i, id) in created_ids.iter().enumerate() {
        let note = manager.get_note(id).await?;
        assert_eq!(note.title(), format!("note_{:02}", i));
        assert_eq!(note.content(), format!("Content of note {}", i));
    }

    // Reload cache in a new manager
    let mut fresh_manager = NoteManager::new(root.clone());
    fresh_manager.load_cache().await?;
    assert_eq!(fresh_manager.cache().len(), 15);

    // Delete 5 notes
    for id in created_ids.iter().take(5) {
        fresh_manager.delete_note(id).await?;
    }
    assert_eq!(fresh_manager.cache().len(), 10);

    // Clear cache
    fresh_manager.clear_cache().await?;
    assert_eq!(fresh_manager.cache().len(), 0);

    let disk_cache = async_fs::read_to_string(root.join(".cache.json")).await?;
    assert_eq!(disk_cache.trim(), "{}");

    Ok(())
}
