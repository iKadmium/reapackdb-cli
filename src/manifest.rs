use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub packages: Vec<Package>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Package {
    pub remote: String,
    pub category: String,
    pub package: String,
}

pub fn get_manifest_path(override_path: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = override_path {
        return Ok(path);
    }

    let config_dir = dirs::config_dir()
        .context("Failed to get config directory")?;

    Ok(config_dir.join("reapackdb-cli").join("manifest.json"))
}

pub fn load_manifest(path: &PathBuf) -> Result<Manifest> {
    if !path.exists() {
        return Ok(Manifest { packages: vec![] });
    }

    let content = fs::read_to_string(path)
        .context("Failed to read manifest file")?;

    let manifest: Manifest = serde_json::from_str(&content)
        .context("Failed to parse manifest JSON")?;

    Ok(manifest)
}

pub fn save_manifest(path: &PathBuf, manifest: &Manifest) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create manifest directory")?;
    }

    let content = serde_json::to_string_pretty(manifest)
        .context("Failed to serialize manifest")?;

    fs::write(path, content)
        .context("Failed to write manifest file")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_load_empty_manifest() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = temp_dir.path().join("manifest.json");

        let manifest = load_manifest(&manifest_path).unwrap();
        assert_eq!(manifest.packages.len(), 0);
    }

    #[test]
    fn test_save_and_load_manifest() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = temp_dir.path().join("manifest.json");

        let manifest = Manifest {
            packages: vec![
                Package {
                    remote: "TestRepo".to_string(),
                    category: "Scripts".to_string(),
                    package: "TestPackage".to_string(),
                },
            ],
        };

        save_manifest(&manifest_path, &manifest).unwrap();
        let loaded = load_manifest(&manifest_path).unwrap();

        assert_eq!(loaded.packages.len(), 1);
        assert_eq!(loaded.packages[0].remote, "TestRepo");
        assert_eq!(loaded.packages[0].category, "Scripts");
        assert_eq!(loaded.packages[0].package, "TestPackage");
    }

    #[test]
    fn test_package_equality() {
        let pkg1 = Package {
            remote: "Repo".to_string(),
            category: "Cat".to_string(),
            package: "Pkg".to_string(),
        };

        let pkg2 = Package {
            remote: "Repo".to_string(),
            category: "Cat".to_string(),
            package: "Pkg".to_string(),
        };

        assert_eq!(pkg1, pkg2);
    }
}
