#!/usr/bin/env rust-script
//! Bump the crate version in Cargo.toml
//!
//! Reads the current `version` from the resolved package manifest (handling the
//! workspace case via rust-paths.rs) and writes the next semver according to the
//! requested bump type. The version line is rewritten in place, preserving the
//! rest of the manifest verbatim.
//!
//! Usage: rust-script rust/scripts/bump-version.rs --bump-type <major|minor|patch> [--dry-run] [--rust-root <path>]
//!
//! Part of the Rust auto-release pipeline (issue #17): without a bump step the
//! crate version never advanced past its initial value, so no new crates.io or
//! GitHub releases were ever produced. Mirrors the link-foundation rust template.
//!
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

use regex::Regex;
use std::env;
use std::process::exit;

#[path = "rust-paths.rs"]
mod rust_paths;

#[derive(Debug, Clone, Copy, PartialEq)]
enum BumpType {
    Major,
    Minor,
    Patch,
}

impl BumpType {
    fn from_str(s: &str) -> Option<BumpType> {
        match s.to_lowercase().as_str() {
            "major" => Some(BumpType::Major),
            "minor" => Some(BumpType::Minor),
            "patch" => Some(BumpType::Patch),
            _ => None,
        }
    }
}

struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    fn bump(&self, bump_type: BumpType) -> String {
        match bump_type {
            BumpType::Major => format!("{}.0.0", self.major + 1),
            BumpType::Minor => format!("{}.{}.0", self.major, self.minor + 1),
            BumpType::Patch => format!("{}.{}.{}", self.major, self.minor, self.patch + 1),
        }
    }

    fn to_display(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

fn get_arg(name: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let flag = format!("--{name}");

    if let Some(idx) = args.iter().position(|a| a == &flag) {
        return args.get(idx + 1).cloned();
    }

    let env_name = name.to_uppercase().replace('-', "_");
    env::var(&env_name).ok().filter(|s| !s.is_empty())
}

fn has_flag(name: &str) -> bool {
    let args: Vec<String> = env::args().collect();
    let flag = format!("--{name}");
    args.contains(&flag)
}

fn parse_version(version: &str) -> Result<Version, String> {
    let re = Regex::new(r"^(\d+)\.(\d+)\.(\d+)").unwrap();
    if let Some(caps) = re.captures(version) {
        Ok(Version {
            major: caps.get(1).unwrap().as_str().parse().unwrap(),
            minor: caps.get(2).unwrap().as_str().parse().unwrap(),
            patch: caps.get(3).unwrap().as_str().parse().unwrap(),
        })
    } else {
        Err(format!("Could not parse semver from version: {version}"))
    }
}

fn update_cargo_toml(manifest_path: &std::path::Path, new_version: &str) -> Result<(), String> {
    let content = std::fs::read_to_string(manifest_path)
        .map_err(|e| format!("Failed to read {}: {}", manifest_path.display(), e))?;

    // Only rewrite the FIRST top-of-line `version = "..."` (the [package]
    // version). Dependency version pins are written as `version = "..."` inside
    // `[dependencies.*]` tables but are indented or inline, so anchoring to the
    // line start with a single replacement keeps them untouched.
    let re = Regex::new(r#"(?m)^(version\s*=\s*")[^"]+(")"#).unwrap();
    let new_content = re.replace(&content, format!("${{1}}{new_version}${{2}}").as_str());

    std::fs::write(manifest_path, new_content.as_ref())
        .map_err(|e| format!("Failed to write {}: {}", manifest_path.display(), e))?;

    Ok(())
}

fn main() {
    let bump_type_str = match get_arg("bump-type") {
        Some(s) => s,
        None => {
            eprintln!("Usage: rust-script rust/scripts/bump-version.rs --bump-type <major|minor|patch> [--dry-run] [--rust-root <path>]");
            exit(1);
        }
    };

    let bump_type = match BumpType::from_str(&bump_type_str) {
        Some(bt) => bt,
        None => {
            eprintln!("Invalid bump type: {bump_type_str}. Must be major, minor, or patch.");
            exit(1);
        }
    };

    let dry_run = has_flag("dry-run");

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

    let info = match rust_paths::read_package_info(&manifest) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };

    let current = match parse_version(&info.version) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };

    let new_version = current.bump(bump_type);

    println!("Current version: {}", current.to_display());
    println!("New version: {new_version}");

    // Emit for workflow consumption (mirrors get-bump-type.rs convention).
    if let Ok(output_file) = env::var("GITHUB_OUTPUT") {
        use std::io::Write;
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&output_file)
        {
            let _ = writeln!(file, "old_version={}", current.to_display());
            let _ = writeln!(file, "new_version={new_version}");
        }
    }

    if dry_run {
        println!("Dry run - no changes made");
    } else {
        if let Err(e) = update_cargo_toml(&manifest, &new_version) {
            eprintln!("Error: {e}");
            exit(1);
        }
        println!("Updated {}", manifest.display());
    }
}
