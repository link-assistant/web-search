#!/usr/bin/env rust-script
//! Determine the version bump type from changelog fragments
//!
//! Reads every `*.md` fragment in `changelog.d/` (except README.md) and picks
//! the HIGHEST bump declared in their frontmatter, so a single `minor` fragment
//! promotes the whole release from `patch` to `minor`, and any `major` fragment
//! wins outright.
//!
//! Fragment format:
//! ```text
//! ---
//! bump: patch|minor|major
//! ---
//!
//! ### Added
//! - Your change here
//! ```
//!
//! Outputs (to `$GITHUB_OUTPUT` when set, and stdout):
//!   - bump_type:    patch | minor | major (defaults to --default when none found)
//!   - fragment_count: number of fragments found
//!   - has_fragments:  true | false
//!
//! Usage: rust-script rust/scripts/get-bump-type.rs [--default <patch|minor|major>] [--rust-root <path>]
//!
//! This is the Rust analogue of the JS changeset count check and mirrors the
//! link-foundation rust pipeline template (issue #17: the Rust crate had no
//! auto-bump step, so its version was permanently stuck and never produced new
//! releases). It uses the shared rust-paths.rs helper for layout detection so it
//! behaves identically in single- and multi-language repositories.
//!
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

use regex::Regex;
use std::env;
use std::fs;
use std::io::Write;
use std::process::exit;

#[path = "rust-paths.rs"]
mod rust_paths;

fn get_arg(name: &str) -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let flag = format!("--{name}");

    if let Some(idx) = args.iter().position(|a| a == &flag) {
        return args.get(idx + 1).cloned();
    }

    let env_name = name.to_uppercase().replace('-', "_");
    env::var(&env_name).ok().filter(|s| !s.is_empty())
}

fn set_output(key: &str, value: &str) {
    if let Ok(output_file) = env::var("GITHUB_OUTPUT") {
        if let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&output_file)
        {
            let _ = writeln!(file, "{key}={value}");
        }
    }
    println!("Output: {key}={value}");
}

fn bump_priority(bump_type: &str) -> u8 {
    match bump_type {
        "patch" => 1,
        "minor" => 2,
        "major" => 3,
        _ => 0,
    }
}

fn parse_frontmatter(content: &str) -> Option<String> {
    let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---").unwrap();

    if let Some(caps) = re.captures(content) {
        let frontmatter = caps.get(1).unwrap().as_str();
        let bump_re = Regex::new(r"^\s*bump\s*:\s*(.+?)\s*$").unwrap();
        for line in frontmatter.lines() {
            if let Some(bump_caps) = bump_re.captures(line) {
                return Some(bump_caps.get(1).unwrap().as_str().to_string());
            }
        }
    }

    None
}

fn determine_bump_type(changelog_dir: &std::path::Path, default_bump: &str) -> (String, usize) {
    if !changelog_dir.exists() {
        println!("No {} directory found", changelog_dir.display());
        return (default_bump.to_string(), 0);
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
        Err(_) => {
            println!("No changelog fragments found");
            return (default_bump.to_string(), 0);
        }
    };

    if files.is_empty() {
        println!("No changelog fragments found");
        return (default_bump.to_string(), 0);
    }

    files.sort();

    let mut highest_priority: u8 = 0;
    let mut highest_bump_type = default_bump.to_string();

    for file in &files {
        if let Ok(content) = fs::read_to_string(file) {
            let name = file.file_name().unwrap().to_string_lossy();
            if let Some(bump) = parse_frontmatter(&content) {
                let priority = bump_priority(&bump);
                if priority > highest_priority {
                    highest_priority = priority;
                    highest_bump_type = bump.clone();
                }
                println!("Fragment {name}: bump={bump}");
            } else {
                println!("Fragment {name}: no bump specified, using default");
            }
        }
    }

    (highest_bump_type, files.len())
}

fn main() {
    let default_bump = get_arg("default").unwrap_or_else(|| "patch".to_string());

    let rust_root = match rust_paths::get_rust_root(None, true) {
        Ok(root) => root,
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    };
    let changelog_dir = rust_paths::get_changelog_dir(&rust_root);

    let (bump_type, fragment_count) = determine_bump_type(&changelog_dir, &default_bump);

    println!("\nDetermined bump type: {bump_type} (from {fragment_count} fragment(s))");

    set_output("bump_type", &bump_type);
    set_output("fragment_count", &fragment_count.to_string());
    set_output(
        "has_fragments",
        if fragment_count > 0 { "true" } else { "false" },
    );
}
