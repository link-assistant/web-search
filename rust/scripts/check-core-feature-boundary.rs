#!/usr/bin/env rust-script
//! Verify that a no-default-features library build contains only core dependencies.
//!
//! Usage: rust-script rust/scripts/check-core-feature-boundary.rs --rust-root rust

use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};

const FORBIDDEN_PACKAGES: &[&str] = &[
    "async-trait",
    "axum",
    "clap",
    "futures",
    "openssl",
    "openssl-sys",
    "regex",
    "reqwest",
    "scraper",
    "serde_json",
    "thiserror",
    "tokio",
    "tower-http",
    "tracing",
    "tracing-subscriber",
    "urlencoding",
    "web-capture",
];

fn rust_root() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    args.iter()
        .position(|arg| arg == "--rust-root")
        .and_then(|index| args.get(index + 1))
        .map_or_else(|| PathBuf::from("."), PathBuf::from)
}

fn main() {
    let manifest = rust_root().join("Cargo.toml");
    let output = Command::new("cargo")
        .args([
            "tree",
            "--manifest-path",
            manifest.to_str().expect("manifest path must be UTF-8"),
            "--no-default-features",
            "--edges",
            "normal",
            "--prefix",
            "none",
        ])
        .output()
        .expect("failed to execute cargo tree");

    if !output.status.success() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    let tree = String::from_utf8_lossy(&output.stdout);
    let present: Vec<&str> = FORBIDDEN_PACKAGES
        .iter()
        .copied()
        .filter(|package| {
            tree.lines()
                .any(|line| line.split_whitespace().next() == Some(package))
        })
        .collect();

    if !present.is_empty() {
        eprintln!(
            "no-default-features dependency boundary includes runtime packages: {}",
            present.join(", ")
        );
        exit(1);
    }

    println!("Merge-only dependency boundary passed");
}
