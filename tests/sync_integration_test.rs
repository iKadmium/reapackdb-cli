use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;
use tempfile::TempDir;

// Test helpers to create DB schema
fn create_test_db(path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE entries (
            id INTEGER PRIMARY KEY,
            remote TEXT NOT NULL,
            category TEXT NOT NULL,
            package TEXT NOT NULL,
            desc TEXT NOT NULL,
            type INTEGER NOT NULL,
            version TEXT NOT NULL,
            author TEXT NOT NULL,
            flags INTEGER DEFAULT 0,
            UNIQUE(remote, category, package)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE files (
            id INTEGER PRIMARY KEY,
            entry INTEGER NOT NULL,
            path TEXT UNIQUE NOT NULL,
            main INTEGER NOT NULL,
            type INTEGER NOT NULL,
            FOREIGN KEY(entry) REFERENCES entries(id)
        )",
        [],
    )?;

    Ok(conn)
}

#[test]
fn test_sync_inserts_new_package() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("reapack.db");

    // Create empty DB with schema
    create_test_db(&db_path)?;

    // Create manifest with one package
    let manifest_dir = TempDir::new()?;
    let manifest_path = manifest_dir.path().join("manifest.json");
    std::fs::write(
        &manifest_path,
        r#"{
            "packages": [
                {
                    "remote": "TestRepo",
                    "category": "Scripts",
                    "package": "TestPackage"
                }
            ]
        }"#,
    )?;

    // Run sync
    reapackdb::commands::sync(&manifest_path, Some(db_path.clone()))?;

    // Verify entry inserted
    let conn = Connection::open(&db_path)?;
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM entries WHERE remote = ? AND category = ? AND package = ?",
        ["TestRepo", "Scripts", "TestPackage"],
        |row| row.get(0),
    )?;
    assert_eq!(count, 1, "Entry should be inserted");

    // Verify version is 0.0.0
    let version: String = conn.query_row(
        "SELECT version FROM entries WHERE remote = ? AND category = ? AND package = ?",
        ["TestRepo", "Scripts", "TestPackage"],
        |row| row.get(0),
    )?;
    assert_eq!(version, "0.0.0", "Version should be 0.0.0");

    // Verify no files exist
    let file_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM files f
         INNER JOIN entries e ON e.id = f.entry
         WHERE e.remote = ? AND e.category = ? AND e.package = ?",
        ["TestRepo", "Scripts", "TestPackage"],
        |row| row.get(0),
    )?;
    assert_eq!(file_count, 0, "No files should exist");

    Ok(())
}

#[test]
fn test_sync_updates_existing_package_without_files() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("reapack.db");

    // Create DB with existing entry at version 1.0.0
    let conn = create_test_db(&db_path)?;
    conn.execute(
        "INSERT INTO entries (remote, category, package, desc, type, version, author, flags)
         VALUES ('TestRepo', 'Scripts', 'TestPackage', 'Test', 0, '1.0.0', 'TestAuthor', 0)",
        [],
    )?;
    drop(conn);

    // Create manifest
    let manifest_dir = TempDir::new()?;
    let manifest_path = manifest_dir.path().join("manifest.json");
    std::fs::write(
        &manifest_path,
        r#"{
            "packages": [
                {
                    "remote": "TestRepo",
                    "category": "Scripts",
                    "package": "TestPackage"
                }
            ]
        }"#,
    )?;

    // Run sync
    reapackdb::commands::sync(&manifest_path, Some(db_path.clone()))?;

    // Verify version updated to 0.0.0
    let conn = Connection::open(&db_path)?;
    let version: String = conn.query_row(
        "SELECT version FROM entries WHERE remote = ? AND category = ? AND package = ?",
        ["TestRepo", "Scripts", "TestPackage"],
        |row| row.get(0),
    )?;
    assert_eq!(version, "0.0.0", "Version should be updated to 0.0.0");

    Ok(())
}

#[test]
fn test_sync_skips_installed_package_with_files() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("reapack.db");

    // Create DB with entry + files (fully installed)
    let conn = create_test_db(&db_path)?;
    conn.execute(
        "INSERT INTO entries (remote, category, package, desc, type, version, author, flags)
         VALUES ('TestRepo', 'Scripts', 'TestPackage', 'Test', 0, '1.5.0', 'TestAuthor', 0)",
        [],
    )?;
    let entry_id: i64 = conn.last_insert_rowid();
    conn.execute(
        "INSERT INTO files (entry, path, main, type)
         VALUES (?, '/some/path/script.lua', 1, 0)",
        [entry_id],
    )?;
    drop(conn);

    // Create manifest
    let manifest_dir = TempDir::new()?;
    let manifest_path = manifest_dir.path().join("manifest.json");
    std::fs::write(
        &manifest_path,
        r#"{
            "packages": [
                {
                    "remote": "TestRepo",
                    "category": "Scripts",
                    "package": "TestPackage"
                }
            ]
        }"#,
    )?;

    // Run sync
    reapackdb::commands::sync(&manifest_path, Some(db_path.clone()))?;

    // Verify version NOT changed (skip installed packages)
    let conn = Connection::open(&db_path)?;
    let version: String = conn.query_row(
        "SELECT version FROM entries WHERE remote = ? AND category = ? AND package = ?",
        ["TestRepo", "Scripts", "TestPackage"],
        |row| row.get(0),
    )?;
    assert_eq!(version, "1.5.0", "Version should remain unchanged");

    Ok(())
}

#[test]
fn test_sync_handles_multiple_packages() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("reapack.db");

    // Create empty DB
    create_test_db(&db_path)?;

    // Create manifest with multiple packages
    let manifest_dir = TempDir::new()?;
    let manifest_path = manifest_dir.path().join("manifest.json");
    std::fs::write(
        &manifest_path,
        r#"{
            "packages": [
                {
                    "remote": "Repo1",
                    "category": "Scripts",
                    "package": "Package1"
                },
                {
                    "remote": "Repo2",
                    "category": "Extensions",
                    "package": "Package2"
                }
            ]
        }"#,
    )?;

    // Run sync
    reapackdb::commands::sync(&manifest_path, Some(db_path.clone()))?;

    // Verify both entries inserted
    let conn = Connection::open(&db_path)?;
    let count: i32 = conn.query_row("SELECT COUNT(*) FROM entries", [], |row| row.get(0))?;
    assert_eq!(count, 2, "Both packages should be inserted");

    Ok(())
}
