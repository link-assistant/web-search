#!/usr/bin/env rust-script
//! Collect changelog fragments into CHANGELOG.md and remove them
//!
//! Gathers every `*.md` fragment in `changelog.d/` (except README.md), strips
//! the `--- bump: ... ---` frontmatter, and prepends a dated section for the
//! CURRENT crate version (read from Cargo.toml — call this AFTER bump-version.rs)
//! to CHANGELOG.md. Processed fragments are then deleted so they are not
//! re-applied on the next release.
//!
//! Usage: rust-script rust/scripts/collect-changelog.rs [--rust-root <path>]
//!
//! Part of the Rust auto-release pipeline (issue #17). Mirrors the
//! link-foundation rust template; uses rust-paths.rs for layout detection.
//!
//! ```cargo
//! [dependencies]
//! regex = "1"
//! chrono = "0.4"
//! ```

use chrono::Utc;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::exit;

#[path = "rust-paths.rs"]
mod rust_paths;

const INSERT_MARKER: &str = "<!-- changelog-insert-here -->";

fn strip_frontmatter(content: &str) -> String {
    let re = Regex::new(r"(?s)^---\s*\n.*?\n---\s*\n(.*)$").unwrap();
    if let Some(caps) = re.captures(content) {
        caps.get(1).unwrap().as_str().trim().to_string()
    } else {
        content.trim().to_string()
    }
}

fn collect_fragments(changelog_dir: &Path) -> String {
    if !changelog_dir.exists() {
        return String::new();
    }

    let mut files: Vec<_> = match fs::read_dir(changelog_dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension().is_some_and(|ext| ext == "md")
                    && p.file_name().is_some_and(|name| name != "README.md")
            })
            .collect(),
        Err(_) => return String::new(),
    };

    files.sort();

    let mut fragments = Vec::new();
    for file in &files {
        if let Ok(raw_content) = fs::read_to_string(file) {
            let content = strip_frontmatter(&raw_content);
            if !content.is_empty() {
                fragments.push(content);
            }
        }
    }

    fragments.join("\n\n")
}

fn update_changelog(changelog_file: &Path, version: &str, fragments: &str) {
    let date_str = Utc::now().format("%Y-%m-%d").to_string();
    let new_entry = format!("\n## [{version}] - {date_str}\n\n{fragments}\n");

    if changelog_file.exists() {
        let mut content = fs::read_to_string(changelog_file).unwrap_or_default();

        if content.contains(INSERT_MARKER) {
            content = content.replace(INSERT_MARKER, &format!("{INSERT_MARKER}{new_entry}"));
        } else {
            // Insert before the first existing release section if present.
            let lines: Vec<&str> = content.lines().collect();
            let insert_index = lines.iter().position(|line| line.starts_with("## ["));

            if let Some(idx) = insert_index {
                let mut new_lines: Vec<String> =
                    lines[..idx].iter().map(|s| s.to_string()).collect();
                new_lines.push(new_entry.clone());
                new_lines.extend(lines[idx..].iter().map(|s| s.to_string()));
                content = new_lines.join("\n");
            } else {
                content.push_str(&new_entry);
            }
        }

        fs::write(changelog_file, content).expect("Failed to write changelog");
    } else {
        let content = format!(
            "# Changelog\n\n\
            All notable changes to this project will be documented in this file.\n\n\
            The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),\n\
            and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n\
            {INSERT_MARKER}\n{new_entry}\n"
        );
        fs::write(changelog_file, content).expect("Failed to write changelog");
    }

    println!("Updated {} with version {version}", changelog_file.display());
}

fn remove_fragments(changelog_dir: &Path) {
    if !changelog_dir.exists() {
        return;
    }

    if let Ok(entries) = fs::read_dir(changelog_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md")
                && path.file_name().is_some_and(|name| name != "README.md")
                && fs::remove_file(&path).is_ok()
            {
                println!("Removed {}", path.display());
            }
        }
    }
}

fn main() {
    let rust_root = match rust_paths::get_rust_root(None, true) {
        Ok(root) => root,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };
    let cargo_toml = rust_paths::get_cargo_toml_path(&rust_root);
    let manifest = match rust_paths::get_package_manifest_path(&cargo_toml) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };
    let changelog_dir = rust_paths::get_changelog_dir(&rust_root);
    let changelog_file = rust_paths::get_changelog_path(&rust_root);

    let info = match rust_paths::read_package_info(&manifest) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };

    println!("Collecting changelog fragments for version {}", info.version);

    let fragments = collect_fragments(&changelog_dir);

    if fragments.is_empty() {
        println!("No changelog fragments found");
        exit(0);
    }

    update_changelog(&changelog_file, &info.version, &fragments);
    remove_fragments(&changelog_dir);

    println!("Changelog collection complete");
}
