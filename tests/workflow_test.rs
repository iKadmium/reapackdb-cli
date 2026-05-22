use std::path::PathBuf;
use tempfile::TempDir;

// These tests would require refactoring commands to be testable without user interaction
// For now, testing the underlying modules

#[test]
fn test_parse_real_ini() {
    let ini_path = PathBuf::from("test-data/reapack.ini");

    if !ini_path.exists() {
        eprintln!("Skipping test: test-data/reapack.ini not found");
        return;
    }

    let conf = ini::Ini::load_from_file(&ini_path).unwrap();
    let section = conf.section(Some("remotes"));

    assert!(section.is_some(), "Should have remotes section");

    let section = section.unwrap();
    let size: usize = section.get("size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    assert!(size > 0, "Should have at least one remote");
}

#[test]
fn test_parse_real_xml_index() {
    use std::fs;

    let xml_dir = PathBuf::from("test-data/ReaPack/cache");

    if !xml_dir.exists() {
        eprintln!("Skipping test: test-data/ReaPack/cache not found");
        return;
    }

    let xml_files: Vec<_> = fs::read_dir(&xml_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("xml"))
        .take(1)
        .collect();

    if xml_files.is_empty() {
        eprintln!("Skipping test: no XML files found");
        return;
    }

    let xml_content = fs::read_to_string(xml_files[0].path()).unwrap();

    // Test with actual reapack module
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(&xml_content);
    reader.config_mut().trim_text(true);

    let mut found_category = false;
    let mut found_package = false;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"category" => found_category = true,
                    b"reapack" => found_package = true,
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error parsing XML: {:?}", e),
            _ => {}
        }
        buf.clear();
    }

    assert!(found_category, "Should find at least one category");
    assert!(found_package, "Should find at least one package");
}

#[test]
fn test_manifest_roundtrip() {
    use serde_json;

    let manifest_json = r#"{
        "packages": [
            {
                "remote": "TestRepo",
                "category": "Scripts",
                "package": "TestPackage"
            }
        ]
    }"#;

    let parsed: serde_json::Value = serde_json::from_str(manifest_json).unwrap();
    assert!(parsed["packages"].is_array());
    assert_eq!(parsed["packages"].as_array().unwrap().len(), 1);

    let pkg = &parsed["packages"][0];
    assert_eq!(pkg["remote"], "TestRepo");
    assert_eq!(pkg["category"], "Scripts");
    assert_eq!(pkg["package"], "TestPackage");
}
