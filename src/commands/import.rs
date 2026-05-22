use anyhow::{Context, Result};
use std::path::PathBuf;
use inquire::MultiSelect;
use rusqlite::Connection;
use crate::manifest::{load_manifest, save_manifest, Package, Manifest};
use crate::reapack::get_reapack_db_path;

pub fn import(manifest_path: &PathBuf, db_path: Option<PathBuf>) -> Result<()> {
    let db_path = get_reapack_db_path(db_path)?;

    let conn = Connection::open(&db_path)
        .context("Failed to open ReaPack database")?;

    // Query entries with files (installed packages)
    let mut stmt = conn.prepare(
        "SELECT DISTINCT e.remote, e.category, e.package
         FROM entries e
         INNER JOIN files f ON f.entry = e.id
         ORDER BY e.remote, e.category, e.package"
    ).context("Failed to prepare query")?;

    let packages_iter = stmt.query_map([], |row| {
        Ok(Package {
            remote: row.get(0)?,
            category: row.get(1)?,
            package: row.get(2)?,
        })
    }).context("Failed to query packages")?;

    let mut installed: Vec<Package> = Vec::new();
    for pkg in packages_iter {
        installed.push(pkg?);
    }

    if installed.is_empty() {
        println!("No installed packages found in database");
        return Ok(());
    }

    // Load current manifest
    let manifest = load_manifest(manifest_path)?;

    // Build list with default selections
    let package_list: Vec<String> = installed.iter()
        .map(|p| format!("{}/{}/{}", p.remote, p.category, p.package))
        .collect();

    let default_indices: Vec<usize> = installed.iter()
        .enumerate()
        .filter(|(_, p)| manifest.packages.contains(p))
        .map(|(i, _)| i)
        .collect();

    let selected = MultiSelect::new("Select packages for manifest:", package_list)
        .with_help_message("Space = toggle | Enter = confirm")
        .with_default(&default_indices)
        .prompt()
        .context("Failed to select packages")?;

    // Build new package list from selection
    let mut new_packages = Vec::new();
    for selection in selected {
        // Parse back remote/category/package
        let parts: Vec<&str> = selection.split('/').collect();
        if parts.len() == 3 {
            new_packages.push(Package {
                remote: parts[0].to_string(),
                category: parts[1].to_string(),
                package: parts[2].to_string(),
            });
        }
    }

    // Calculate changes
    let additions: Vec<Package> = new_packages.iter()
        .filter(|p| !manifest.packages.contains(p))
        .cloned()
        .collect();

    let removals: Vec<Package> = manifest.packages.iter()
        .filter(|p| installed.contains(p) && !new_packages.contains(p))
        .cloned()
        .collect();

    // Show colored summary
    if additions.is_empty() && removals.is_empty() {
        println!("No changes to manifest");
        return Ok(());
    }

    println!("\nChanges to apply:");
    for pkg in &additions {
        println!("\x1b[32m+ {}/{}/{}\x1b[0m", pkg.remote, pkg.category, pkg.package);
    }
    for pkg in &removals {
        println!("\x1b[31m- {}/{}/{}\x1b[0m", pkg.remote, pkg.category, pkg.package);
    }

    // Confirm
    let confirm = inquire::Confirm::new("Apply changes?")
        .with_default(true)
        .prompt()
        .context("Failed to get confirmation")?;

    if !confirm {
        println!("Cancelled");
        return Ok(());
    }

    // Apply changes
    let final_manifest = Manifest {
        packages: manifest.packages.iter()
            .filter(|p| !installed.contains(p))
            .cloned()
            .chain(new_packages)
            .collect(),
    };

    save_manifest(manifest_path, &final_manifest)?;

    println!("Updated manifest: +{} packages, -{} packages", additions.len(), removals.len());

    Ok(())
}
