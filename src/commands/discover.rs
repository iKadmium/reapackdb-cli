use anyhow::{Context, Result};
use std::path::PathBuf;
use inquire::MultiSelect;
use crate::manifest::{load_manifest, save_manifest, Package, Manifest};
use crate::reapack::{get_reapack_ini_path, read_remotes_from_ini, fetch_index, parse_packages_from_index};

pub fn discover(manifest_path: &PathBuf, ini_path: Option<PathBuf>) -> Result<()> {
    let ini_path = get_reapack_ini_path(ini_path)?;
    let remotes = read_remotes_from_ini(&ini_path)?;

    if remotes.is_empty() {
        println!("No enabled remotes found in reapack.ini");
        return Ok(());
    }

    // Select remote
    let remote_names: Vec<String> = remotes.iter()
        .map(|r| format!("{} ({})", r.name, r.url))
        .collect();

    let selected_remote_idx = inquire::Select::new("Select remote:", remote_names)
        .prompt()
        .context("Failed to select remote")?;

    // Extract index from selected option
    let selected_idx = remotes.iter()
        .position(|r| format!("{} ({})", r.name, r.url) == selected_remote_idx)
        .context("Failed to find selected remote")?;

    let remote = &remotes[selected_idx];

    println!("Fetching index from {}...", remote.url);
    let xml = fetch_index(&remote.url)?;

    let packages = parse_packages_from_index(&xml)?;

    if packages.is_empty() {
        println!("No packages found in index");
        return Ok(());
    }

    // Load current manifest
    let manifest = load_manifest(manifest_path)?;

    // Build lookup of currently selected packages
    let currently_in_manifest: Vec<Package> = manifest.packages.iter()
        .filter(|p| p.remote == remote.name)
        .cloned()
        .collect();

    // Build list with default selections
    let package_list: Vec<String> = packages.iter()
        .map(|(cat, pkg)| format!("{}/{}", cat, pkg))
        .collect();

    let default_indices: Vec<usize> = packages.iter()
        .enumerate()
        .filter(|(_, (cat, pkg))| {
            currently_in_manifest.iter().any(|p|
                &p.category == cat && &p.package == pkg
            )
        })
        .map(|(i, _)| i)
        .collect();

    let selected = MultiSelect::new("Select packages for manifest:", package_list)
        .with_help_message("Space = toggle | Enter = confirm")
        .with_default(&default_indices)
        .prompt()
        .context("Failed to select packages")?;

    // Build new package list for this remote
    let mut new_packages_for_remote = Vec::new();
    for selection in selected {
        if let Some((cat, pkg)) = selection.split_once('/') {
            new_packages_for_remote.push(Package {
                remote: remote.name.clone(),
                category: cat.to_string(),
                package: pkg.to_string(),
            });
        }
    }

    // Calculate changes
    let additions: Vec<Package> = new_packages_for_remote.iter()
        .filter(|p| !currently_in_manifest.contains(p))
        .cloned()
        .collect();

    let removals: Vec<Package> = currently_in_manifest.iter()
        .filter(|p| !new_packages_for_remote.contains(p))
        .cloned()
        .collect();

    // Show colored summary
    if additions.is_empty() && removals.is_empty() {
        println!("No changes to manifest");
        return Ok(());
    }

    println!("\nChanges to apply:");
    for pkg in &additions {
        println!("\x1b[32m+ {}/{}\x1b[0m", pkg.category, pkg.package);
    }
    for pkg in &removals {
        println!("\x1b[31m- {}/{}\x1b[0m", pkg.category, pkg.package);
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
    let mut new_manifest = Manifest {
        packages: manifest.packages.iter()
            .filter(|p| p.remote != remote.name)
            .cloned()
            .collect(),
    };
    new_manifest.packages.extend(new_packages_for_remote);

    save_manifest(manifest_path, &new_manifest)?;

    println!("Updated manifest: +{} packages, -{} packages", additions.len(), removals.len());

    Ok(())
}
