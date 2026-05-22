# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project
Rust CLI tool for declarative ReaPack package management. ReaPack is REAPER's package manager - tracks state in SQLite DB but lacks declarative config. This tool bridges gap by maintaining manifest of desired packages, syncing to DB.

## Strategy
Don't reinvent installation - trick ReaPack into doing work:
1. Read manifest of desired packages
2. Insert entries into DB with version lower than latest
3. Set NO files (via empty `files` table for that entry)
4. ReaPack detects "outdated package with missing files" → triggers update on next launch

## ReaPack Database Schema
SQLite at REAPER's data path. Two tables:

```sql
CREATE TABLE entries (
  id INTEGER PRIMARY KEY,
  remote TEXT NOT NULL,           -- repo name (unique identifier)
  category TEXT NOT NULL,          -- package category
  package TEXT NOT NULL,           -- package name (unique within repo)
  desc TEXT NOT NULL,
  type INTEGER NOT NULL,
  version TEXT NOT NULL,           -- version string (semantic comparison)
  author TEXT NOT NULL,
  flags INTEGER DEFAULT 0,
  UNIQUE(remote, category, package)
);

CREATE TABLE files (
  id INTEGER PRIMARY KEY,
  entry INTEGER NOT NULL,          -- FK to entries.id
  path TEXT UNIQUE NOT NULL,       -- installed file path
  main INTEGER NOT NULL,
  type INTEGER NOT NULL,
  FOREIGN KEY(entry) REFERENCES entries(id)
);
```

**Key insights:**
- Package uniquely identified by `(remote, category, package)` tuple
- Version comparison: ReaPack uses `VersionName::compare()` - semantic versioning with numeric/string segment comparison
- Installation detection: `files` table has rows with FK to entry → package installed
- Update trigger: `entries.version` < latest version AND no `files` rows → ReaPack updates

## Manifest Format
JSON at `~/.config/reapackdb-cli/manifest.json` (or OS equivalents). Override with `--manifest`.

```json
{
  "packages": [
    {
      "remote": "ReaTeam/Extensions",
      "category": "Extensions",
      "package": "SWS Extension"
    }
  ]
}
```

**Fields required:**
- `remote`: Repo name (must match ReaPack's remote list)
- `category`: Package category within repo
- `package`: Package name within category

## Commands
- `reapackdb-cli` - Sync manifest to DB (idempotent, one-way)
- `reapackdb-cli add <remote> <category> <package>` - Add package to manifest
- `reapackdb-cli remove <remote> <category> <package>` - Remove from manifest

## Sync Logic
1. Read manifest
2. Query ReaPack DB for each package
3. For each package in manifest:
   - If entry exists with correct version + files → skip (already installed)
   - If entry missing OR has old version OR no files → insert/update entry with version "0.0.0", NO files
   - ReaPack handles actual installation as "update"
4. Packages in DB but not manifest → ignore (don't remove user's manual installs)

## Development Environment
Dev container configured with Rust toolchain (Debian Trixie base).
- Config: `.devcontainer/devcontainer.json`
- Cargo cache persisted in Docker volume
- **Start container**: `devcontainer up --workspace-folder .`
- **All commands run via devcontainer CLI**: `devcontainer exec --workspace-folder . <command>`

## Code Organization
Keep files small. Split functionality where it makes sense:
- Each CLI command in `src/commands/` directory, own file per command
- Shared types and logic in dedicated modules (manifest, reapack DB/INI, etc.)
- `main.rs` thin - just CLI definition and dispatch

When file grows beyond ~200-300 lines or handles multiple concerns, split it. Better many focused files than few large ones.

## Build & Test
Standard Rust workflow via devcontainer:
- `devcontainer exec --workspace-folder . cargo build` - Build project
- `devcontainer exec --workspace-folder . cargo test` - Run tests
- `devcontainer exec --workspace-folder . cargo run -- <args>` - Run CLI with args

## License
GNU Affero General Public License v3 (AGPL-3.0)
- Network use triggers source disclosure requirement
- Modified versions served over network must provide source access
