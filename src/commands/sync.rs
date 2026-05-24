use crate::manifest::load_manifest;
use crate::reapack::get_reapack_db_path;
use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::PathBuf;

pub fn sync(manifest_path: &PathBuf, db_path: Option<PathBuf>) -> Result<()> {
    let manifest = load_manifest(manifest_path)?;
    let db_path = get_reapack_db_path(db_path)?;

    if manifest.packages.is_empty() {
        println!("No packages in manifest to sync");
        return Ok(());
    }

    println!(
        "Syncing {} packages to ReaPack database...",
        manifest.packages.len()
    );

    let conn = Connection::open(&db_path).context("Failed to open ReaPack database")?;

    let mut updated = 0;
    let mut inserted = 0;

    for package in &manifest.packages {
        // Check if entry exists
        let exists: bool = conn
            .query_row(
                "SELECT 1 FROM entries WHERE remote = ? AND category = ? AND package = ? LIMIT 1",
                [&package.remote, &package.category, &package.package],
                |_| Ok(true),
            )
            .unwrap_or(false);

        if exists {
            // Check if it has files (fully installed)
            let has_files: bool = conn
                .query_row(
                    "SELECT 1 FROM files f
                 INNER JOIN entries e ON e.id = f.entry
                 WHERE e.remote = ? AND e.category = ? AND e.package = ?
                 LIMIT 1",
                    [&package.remote, &package.category, &package.package],
                    |_| Ok(true),
                )
                .unwrap_or(false);

            if has_files {
                // Package fully installed, skip
                continue;
            }

            // Update existing entry to version 0.0.0 (no files already means it needs update)
            conn.execute(
                "UPDATE entries SET version = '0.0.0'
                 WHERE remote = ? AND category = ? AND package = ?",
                [&package.remote, &package.category, &package.package],
            )
            .context(format!(
                "Failed to update entry: {}/{}/{}",
                package.remote, package.category, package.package
            ))?;

            updated += 1;
        } else {
            // Insert new entry with version 0.0.0, no files
            // Need minimal fields: remote, category, package, desc, type, version, author, flags
            conn.execute(
                "INSERT INTO entries (remote, category, package, desc, type, version, author, flags)
                 VALUES (?, ?, ?, '', 0, '0.0.0', '', 0)",
                [&package.remote, &package.category, &package.package]
            ).context(format!("Failed to insert entry: {}/{}/{}",
                package.remote, package.category, package.package))?;

            inserted += 1;
        }
    }

    if inserted > 0 || updated > 0 {
        println!("Synced to database: {} new, {} updated", inserted, updated);
        println!("ReaPack will install/update these packages on next launch");
    } else {
        println!("All packages already up to date");
    }

    Ok(())
}
