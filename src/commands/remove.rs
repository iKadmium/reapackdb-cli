use anyhow::Result;
use std::path::PathBuf;
use crate::manifest::{load_manifest, save_manifest, Package};

pub fn remove_package(manifest_path: &PathBuf, remote: String, category: String, package: String) -> Result<()> {
    let mut manifest = load_manifest(manifest_path)?;

    let target = Package {
        remote: remote.clone(),
        category: category.clone(),
        package: package.clone(),
    };

    let original_len = manifest.packages.len();
    manifest.packages.retain(|p| p != &target);

    if manifest.packages.len() == original_len {
        println!("Package not found in manifest: {}/{}/{}", remote, category, package);
        return Ok(());
    }

    save_manifest(manifest_path, &manifest)?;

    println!("Removed package: {}/{}/{}", remote, category, package);
    Ok(())
}
