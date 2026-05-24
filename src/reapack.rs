use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Remote {
    pub name: String,
    pub url: String,
    pub enabled: bool,
}

impl std::fmt::Display for Remote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.url)
    }
}

fn parse_remote(data: &str) -> Option<Remote> {
    let parts: Vec<&str> = data.split('|').collect();
    if parts.len() < 3 {
        return None;
    }

    Some(Remote {
        name: parts[0].to_string(),
        url: parts[1].to_string(),
        enabled: parts[2].parse().unwrap_or(true),
    })
}

fn get_reaper_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not find system config directory")?;
    Ok(config_dir.join("REAPER"))
}

pub fn get_reapack_ini_path(override_path: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = override_path {
        return Ok(path);
    }

    let reaper_dir = get_reaper_config_dir()?;
    let ini_path = reaper_dir.join("reapack.ini");

    if !ini_path.exists() {
        anyhow::bail!(
            "reapack.ini not found at {}. Please specify path with --ini flag",
            ini_path.display()
        );
    }

    Ok(ini_path)
}

pub fn get_reapack_db_path(override_path: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = override_path {
        return Ok(path);
    }

    let reaper_dir = get_reaper_config_dir()?;
    let db_dir = reaper_dir.join("ReaPack");

    if !db_dir.exists() {
        anyhow::bail!(
            "ReaPack directory not found at {}. Please specify database path with --db flag",
            db_dir.display()
        );
    }

    // Look for reapack.db or similar database file
    let db_candidates = ["reapack.db", "ReaPack.db", "registry.db"];

    for candidate in &db_candidates {
        let db_path = db_dir.join(candidate);
        if db_path.exists() {
            return Ok(db_path);
        }
    }

    anyhow::bail!(
        "ReaPack database not found in {}. Please specify database path with --db flag",
        db_dir.display()
    );
}

pub fn read_remotes_from_ini(ini_path: &PathBuf) -> Result<Vec<Remote>> {
    let conf = ini::Ini::load_from_file(ini_path).context("Failed to read reapack.ini")?;

    let section = conf
        .section(Some("remotes"))
        .context("No [remotes] section in reapack.ini")?;

    let size: usize = section
        .get("size")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let mut remotes = Vec::new();
    for i in 0..size {
        if let Some(data) = section.get(format!("remote{}", i))
            && let Some(remote) = parse_remote(data)
            && remote.enabled
        {
            remotes.push(remote);
        }
    }

    Ok(remotes)
}

pub fn fetch_index(url: &str) -> Result<String> {
    let response = reqwest::blocking::get(url).context("Failed to fetch index")?;

    response.text().context("Failed to read index response")
}

pub fn parse_packages_from_index(xml: &str) -> Result<Vec<(String, String)>> {
    use quick_xml::Reader;
    use quick_xml::events::Event;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut packages = Vec::new();
    let mut current_category = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"category" => {
                        // Extract category name attribute
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"name" {
                                current_category =
                                    String::from_utf8_lossy(&attr.value).into_owned();
                                break;
                            }
                        }
                    }
                    b"reapack" => {
                        // Extract package name from desc attribute
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"desc" {
                                let package_name =
                                    String::from_utf8_lossy(&attr.value).into_owned();
                                packages.push((current_category.clone(), package_name));
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                anyhow::bail!(
                    "Error parsing XML at position {}: {:?}",
                    reader.buffer_position(),
                    e
                );
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(packages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_remote() {
        let remote = parse_remote("TestRepo|https://example.com/index.xml|true|indeterminate");
        assert!(remote.is_some());

        let remote = remote.unwrap();
        assert_eq!(remote.name, "TestRepo");
        assert_eq!(remote.url, "https://example.com/index.xml");
        assert!(remote.enabled);
    }

    #[test]
    fn test_parse_remote_invalid() {
        let remote = parse_remote("incomplete");
        assert!(remote.is_none());
    }

    #[test]
    fn test_parse_packages_from_index() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<index version="1" name="Test">
  <category name="Scripts">
    <reapack name="test1.lua" type="script" desc="Test Package 1">
      <version name="1.0.0" author="test"/>
    </reapack>
    <reapack name="test2.lua" type="script" desc="Test Package 2">
      <version name="1.0.0" author="test"/>
    </reapack>
  </category>
  <category name="Effects">
    <reapack name="test3.jsfx" type="effect" desc="Test Effect">
      <version name="1.0.0" author="test"/>
    </reapack>
  </category>
</index>"#;

        let packages = parse_packages_from_index(xml).unwrap();
        assert_eq!(packages.len(), 3);

        assert_eq!(packages[0].0, "Scripts");
        assert_eq!(packages[0].1, "Test Package 1");

        assert_eq!(packages[1].0, "Scripts");
        assert_eq!(packages[1].1, "Test Package 2");

        assert_eq!(packages[2].0, "Effects");
        assert_eq!(packages[2].1, "Test Effect");
    }
}
