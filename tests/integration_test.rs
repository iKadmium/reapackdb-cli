use std::fs;
use std::path::PathBuf;

#[test]
fn test_read_test_data_ini() {
    let ini_path = PathBuf::from("test-data/reapack.ini");
    assert!(ini_path.exists(), "test-data/reapack.ini should exist");

    let content = fs::read_to_string(&ini_path).unwrap();
    assert!(
        content.contains("[remotes]"),
        "INI should have [remotes] section"
    );
}

#[test]
fn test_read_test_data_xml() {
    let xml_dir = PathBuf::from("test-data/ReaPack/cache");
    assert!(xml_dir.exists(), "test-data/ReaPack/cache should exist");

    let xml_files: Vec<_> = fs::read_dir(&xml_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("xml"))
        .collect();

    assert!(!xml_files.is_empty(), "Should have XML index files");

    // Test parsing one of the XML files
    let first_xml = &xml_files[0];
    let content = fs::read_to_string(first_xml.path()).unwrap();
    assert!(
        content.contains("<index"),
        "XML should be valid ReaPack index"
    );
    assert!(content.contains("<category"), "XML should have categories");
}

#[test]
fn test_reapack_db_exists() {
    let db_dir = PathBuf::from("test-data/ReaPack");
    assert!(db_dir.exists(), "test-data/ReaPack should exist");

    // Check for common ReaPack DB file patterns
    let entries: Vec<_> = fs::read_dir(&db_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert!(
        !entries.is_empty(),
        "ReaPack directory should contain files"
    );
}
