use crate::manifest::{Package, load_manifest, save_manifest};
use anyhow::Result;
use std::path::PathBuf;

pub fn add_package(
    manifest_path: &PathBuf,
    remote: String,
    category: String,
    package: String,
) -> Result<()> {
    let mut manifest = load_manifest(manifest_path)?;

    let new_package = Package {
        remote: remote.clone(),
        category: category.clone(),
        package: package.clone(),
    };

    if manifest.packages.contains(&new_package) {
        println!(
            "Package already in manifest: {}/{}/{}",
            remote, category, package
        );
        return Ok(());
    }

    manifest.packages.push(new_package);
    save_manifest(manifest_path, &manifest)?;

    println!("Added package: {}/{}/{}", remote, category, package);
    Ok(())
}
