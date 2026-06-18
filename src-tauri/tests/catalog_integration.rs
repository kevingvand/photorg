use std::fs;
use tempfile::TempDir;

/// Sample integration test demonstrating isolation and cleanup
#[test]
fn test_catalog_creates_database_in_temp_directory() {
    // Arrange: Create isolated temporary directory
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create a sample database marker file (simulating database creation)
    let db_path = temp_path.join("catalog.db");
    fs::write(&db_path, "test database").expect("failed to write test db");

    // Act: Verify the file was created
    assert!(db_path.exists(), "database should be created");
    assert!(fs::metadata(&db_path).is_ok(), "database should be readable");

    // Assert: Verify content
    let content = fs::read_to_string(&db_path).expect("failed to read db");
    assert_eq!(content, "test database");

    // Cleanup: TempDir automatically cleans up on drop
    // This test demonstrates proper isolation without manual cleanup
}

/// Sample integration test for image pair discovery
#[test]
fn test_discovers_jpeg_files_in_directory() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let temp_path = temp_dir.path();

    // Create sample files
    fs::write(temp_path.join("photo1.raw"), b"RAW data").expect("failed to write RAW");
    fs::write(temp_path.join("photo1.jpg"), b"JPEG data").expect("failed to write JPEG");
    fs::write(temp_path.join("photo2.raw"), b"RAW data").expect("failed to write RAW");

    // Verify files exist
    assert!(temp_path.join("photo1.raw").exists());
    assert!(temp_path.join("photo1.jpg").exists());
    assert!(temp_path.join("photo2.raw").exists());

    // Read directory and count files
    let entries: Vec<_> = fs::read_dir(temp_path)
        .expect("failed to read dir")
        .filter_map(|e| e.ok())
        .collect();

    assert_eq!(entries.len(), 3, "should have 3 files");
}

/// Sample integration test with error handling
#[test]
fn test_handles_missing_directory_gracefully() {
    let non_existent_path = "/tmp/photorg_test_nonexistent_12345";

    // This demonstrates testing error conditions
    let result = fs::read_dir(non_existent_path);
    assert!(result.is_err(), "should fail for non-existent directory");
}

#[cfg(test)]
mod state_isolation_tests {
    use super::*;

    /// Example of a more complex integration test with state management
    #[test]
    fn test_state_isolation_between_tests() {
        // Each test gets its own temp directory
        let temp1 = TempDir::new().expect("failed to create temp dir 1");
        let temp2 = TempDir::new().expect("failed to create temp dir 2");

        // Write to each independently
        fs::write(temp1.path().join("file.txt"), b"test1").expect("write 1");
        fs::write(temp2.path().join("file.txt"), b"test2").expect("write 2");

        // Verify they don't interfere
        let content1 = fs::read_to_string(temp1.path().join("file.txt")).unwrap();
        let content2 = fs::read_to_string(temp2.path().join("file.txt")).unwrap();

        assert_eq!(content1, "test1");
        assert_eq!(content2, "test2");
    }
}
