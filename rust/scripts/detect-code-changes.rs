#!/usr/bin/env rust-script
//! Detect code changes for CI/CD pipeline
//!
//! This script detects what types of files have changed in the latest commit
//! and outputs the results for use in GitHub Actions workflow conditions.
//!
//! Key behavior:
//! - For PRs: detects GitHub Actions' synthetic merge commit and uses
//!   HEAD^2^..HEAD^2 to get the per-commit diff of the actual PR head,
//!   so a commit touching only non-code files correctly skips CI jobs
//!   even when earlier commits in the same PR touched code files.
//! - For pushes: compares HEAD against HEAD^
//! - Excludes certain folders and file types from "code changes" detection
//!
//! Excluded from code changes (don't require changelog fragments):
//! - Markdown files (*.md) in any folder
//! - changelog.d/ folder (changelog fragments)
//! - docs/ folder (documentation)
//! - experiments/ folder (experimental scripts)
//! - examples/ folder (example scripts)
//!
//! Usage: rust-script rust/scripts/detect-code-changes.rs --rust-root rust
//!
//! Environment variables (set by GitHub Actions):
//!   - GITHUB_EVENT_NAME: 'pull_request' or 'push'
//!
//! Outputs (written to GITHUB_OUTPUT):
//!   - rust-changed: 'true' if any Rust tree or Rust workflow file changed
//!   - rust-scripts-changed: 'true' if any Rust CI/CD script changed
//!   - rust-workflow-changed: 'true' if the Rust or parity workflow changed
//!   - rust-code-changed: 'true' if Rust source, tests, scripts, Cargo, or workflows changed
//!   - any-rust-code-changed: alias for rust-code-changed
//!   - rs-changed: 'true' if any Rust .rs files changed
//!   - toml-changed: 'true' if any Rust .toml files changed
//!   - mjs-changed: 'true' if any .mjs files changed
//!   - docs-changed: 'true' if any .md files changed
//!   - workflow-changed: 'true' if any .github/workflows/ files changed
//!   - any-code-changed: 'true' if any code files changed (excludes docs, changelog.d, experiments, examples)
//!
//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

use regex::Regex;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn exec(command: &str, args: &[&str]) -> String {
    match Command::new(command).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                eprintln!("Error executing {} {:?}", command, args);
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                String::new()
            }
        }
        Err(e) => {
            eprintln!("Failed to execute {} {:?}: {}", command, args, e);
            String::new()
        }
    }
}

fn set_output(name: &str, value: &str) {
    if let Ok(output_file) = env::var("GITHUB_OUTPUT") {
        if let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&output_file)
        {
            let _ = writeln!(file, "{}={}", name, value);
        }
    }
    println!("{}={}", name, value);
}

fn is_merge_commit() -> bool {
    let output = exec("git", &["cat-file", "-p", "HEAD"]);
    output
        .lines()
        .filter(|line| line.starts_with("parent "))
        .count()
        > 1
}

fn get_changed_files() -> Vec<String> {
    // GitHub Actions checks out a synthetic merge commit for pull_request
    // events: HEAD is the merge commit, HEAD^ is the base branch, HEAD^2
    // is the actual PR head. To get the per-commit diff (what the latest
    // push actually changed), we compare HEAD^2^ to HEAD^2.
    // For push events, HEAD is the real commit, so HEAD^ to HEAD works.
    if is_merge_commit() {
        println!("Merge commit detected (pull_request event)");
        println!("Comparing HEAD^2^ to HEAD^2 (per-commit diff of PR head)");
        let output = exec("git", &["diff", "--name-only", "HEAD^2^", "HEAD^2"]);
        if !output.is_empty() {
            return output
                .lines()
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();
        }
        // Fallback: first commit in PR, compare base to PR head
        println!("HEAD^2^ not available (first commit in PR), comparing HEAD^ to HEAD^2");
        let output = exec("git", &["diff", "--name-only", "HEAD^", "HEAD^2"]);
        if !output.is_empty() {
            return output
                .lines()
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();
        }
    }

    println!("Comparing HEAD^ to HEAD");
    let output = exec("git", &["diff", "--name-only", "HEAD^", "HEAD"]);

    if output.is_empty() {
        println!("HEAD^ not available, listing all files in HEAD");
        let output = exec("git", &["ls-tree", "--name-only", "-r", "HEAD"]);
        return output
            .lines()
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
    }

    output
        .lines()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

fn get_rust_root() -> String {
    let args: Vec<String> = env::args().collect();
    if let Some(idx) = args.iter().position(|arg| arg == "--rust-root") {
        if let Some(root) = args.get(idx + 1) {
            return root.clone();
        }
    }

    if let Ok(root) = env::var("RUST_ROOT") {
        if !root.is_empty() {
            return root;
        }
    }

    if Path::new("./Cargo.toml").exists() {
        return ".".to_string();
    }

    if Path::new("./rust/Cargo.toml").exists() {
        return "rust".to_string();
    }

    ".".to_string()
}

fn rust_prefix(rust_root: &str) -> String {
    if rust_root == "." {
        String::new()
    } else {
        format!("{rust_root}/")
    }
}

fn is_rust_workflow(file_path: &str) -> bool {
    matches!(
        file_path,
        ".github/workflows/rust.yml" | ".github/workflows/parity.yml"
    )
}

fn is_rust_file(file_path: &str, rust_root: &str) -> bool {
    file_path.starts_with(&rust_prefix(rust_root)) || is_rust_workflow(file_path)
}

fn is_rust_script(file_path: &str, rust_root: &str) -> bool {
    let prefix = rust_prefix(rust_root);
    file_path.starts_with(&format!("{prefix}scripts/"))
}

fn is_excluded_from_code_changes(file_path: &str, rust_root: &str) -> bool {
    // Exclude markdown files in any folder
    if file_path.ends_with(".md") {
        return true;
    }

    // Exclude specific folders from code changes
    let prefix = rust_prefix(rust_root);
    let excluded_folders = [
        format!("{prefix}changelog.d/"),
        format!("{prefix}docs/"),
        format!("{prefix}experiments/"),
        format!("{prefix}examples/"),
    ];

    for folder in excluded_folders {
        if file_path.starts_with(&folder) {
            return true;
        }
    }

    false
}

fn main() {
    println!("Detecting file changes for CI/CD...\n");

    let rust_root = get_rust_root();
    let prefix = rust_prefix(&rust_root);
    println!("Rust root: {rust_root}");

    let changed_files = get_changed_files();

    println!("Changed files:");
    if changed_files.is_empty() {
        println!("  (none)");
    } else {
        for file in &changed_files {
            println!("  {}", file);
        }
    }
    println!();

    let rust_changed = changed_files
        .iter()
        .any(|file| is_rust_file(file, &rust_root));
    set_output("rust-changed", if rust_changed { "true" } else { "false" });

    let rust_scripts_changed = changed_files
        .iter()
        .any(|file| is_rust_script(file, &rust_root));
    set_output(
        "rust-scripts-changed",
        if rust_scripts_changed {
            "true"
        } else {
            "false"
        },
    );

    let rust_workflow_changed = changed_files.iter().any(|file| is_rust_workflow(file));
    set_output(
        "rust-workflow-changed",
        if rust_workflow_changed {
            "true"
        } else {
            "false"
        },
    );

    // Detect .rs file changes (Rust source)
    let rs_changed = changed_files
        .iter()
        .any(|f| f.starts_with(&prefix) && f.ends_with(".rs"));
    set_output("rs-changed", if rs_changed { "true" } else { "false" });

    // Detect .toml file changes (Cargo.toml, Cargo.lock, etc.)
    let toml_changed = changed_files
        .iter()
        .any(|f| f.starts_with(&prefix) && f.ends_with(".toml"));
    set_output("toml-changed", if toml_changed { "true" } else { "false" });

    // Detect .mjs file changes (scripts)
    let mjs_changed = changed_files.iter().any(|f| f.ends_with(".mjs"));
    set_output("mjs-changed", if mjs_changed { "true" } else { "false" });

    // Detect documentation changes (any .md file)
    let docs_changed = changed_files.iter().any(|f| f.ends_with(".md"));
    set_output("docs-changed", if docs_changed { "true" } else { "false" });

    // Detect workflow changes
    let workflow_changed = changed_files
        .iter()
        .any(|f| f.starts_with(".github/workflows/"));
    set_output(
        "workflow-changed",
        if workflow_changed { "true" } else { "false" },
    );

    // Detect code changes (excluding docs, changelog.d, experiments, examples folders, and markdown files)
    let code_changed_files: Vec<&String> = changed_files
        .iter()
        .filter(|f| is_rust_file(f, &rust_root))
        .filter(|f| !is_excluded_from_code_changes(f, &rust_root))
        .collect();

    println!("\nFiles considered as code changes:");
    if code_changed_files.is_empty() {
        println!("  (none)");
    } else {
        for file in &code_changed_files {
            println!("  {}", file);
        }
    }
    println!();

    // Check if any Rust code files changed (.rs, .toml, .lock, scripts, or workflow files)
    let code_pattern = Regex::new(r"\.(rs|toml|lock|yml|yaml)$|\.github/workflows/").unwrap();
    let code_changed = code_changed_files.iter().any(|f| code_pattern.is_match(f));
    set_output(
        "rust-code-changed",
        if code_changed { "true" } else { "false" },
    );
    set_output(
        "any-rust-code-changed",
        if code_changed { "true" } else { "false" },
    );
    set_output(
        "any-code-changed",
        if code_changed { "true" } else { "false" },
    );

    println!("\nChange detection completed.");
}
