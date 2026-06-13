//! FormalAI provider-parity contract (Rust parity with
//! `tests/formal-ai-compat.test.js`).
//!
//! Asserts that the `web-search` crate is a superset of FormalAI's
//! `web_search_core` provider registry (issue #5). The expected FormalAI
//! surface lives in the shared compatibility map
//! `docs/case-studies/issue-5/formal-ai-compatibility.json`, which both this
//! Rust suite and the JavaScript `formal-ai-compat.test.js` suite read so the
//! two languages enforce identical guarantees.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use serde_json::Value;
use web_search::providers::{get_default_provider_ids, get_registry, RegistryEntry};

fn compat() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("docs")
        .join("case-studies")
        .join("issue-5")
        .join("formal-ai-compatibility.json");
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&raw).expect("valid compatibility JSON")
}

fn registry_by_id() -> HashMap<String, RegistryEntry> {
    get_registry()
        .into_iter()
        .map(|e| (e.id.clone(), e))
        .collect()
}

#[test]
fn registers_every_formal_ai_provider() {
    let map = compat();
    let registry = registry_by_id();
    for provider in map["formalAiProviders"].as_array().unwrap() {
        let id = provider["id"].as_str().unwrap();
        assert!(
            registry.contains_key(id),
            "web-search registry is missing FormalAI provider: {id}"
        );
    }
}

#[test]
fn matches_category_and_default_flags() {
    let map = compat();
    let registry = registry_by_id();
    for provider in map["formalAiProviders"].as_array().unwrap() {
        let id = provider["id"].as_str().unwrap();
        let entry = registry.get(id).unwrap();
        assert_eq!(
            entry.category,
            provider["category"].as_str().unwrap(),
            "category mismatch for {id}"
        );
        assert_eq!(
            entry.default_for_category,
            provider["defaultForCategory"].as_bool().unwrap(),
            "default-for-category mismatch for {id}"
        );
    }
}

#[test]
fn matches_cors_readability_except_documented_exceptions() {
    let map = compat();
    let registry = registry_by_id();
    let exceptions: HashMap<&str, bool> = map["corsReadableExceptions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| (e["id"].as_str().unwrap(), e["webSearch"].as_bool().unwrap()))
        .collect();

    for provider in map["formalAiProviders"].as_array().unwrap() {
        let id = provider["id"].as_str().unwrap();
        let entry = registry.get(id).unwrap();
        let expected = exceptions
            .get(id)
            .copied()
            .unwrap_or_else(|| provider["corsReadable"].as_bool().unwrap());
        assert_eq!(entry.cors_readable, expected, "CORS mismatch for {id}");
    }
}

#[test]
fn uses_the_formal_ai_default_plan() {
    let map = compat();
    let expected: Vec<&str> = map["formalAiDefaultPlan"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(get_default_provider_ids(), expected);
}

#[test]
fn default_plan_providers_are_cors_readable_except_duckduckgo() {
    let map = compat();
    let registry = registry_by_id();
    for id in map["formalAiDefaultPlan"].as_array().unwrap() {
        let id = id.as_str().unwrap();
        let entry = registry
            .get(id)
            .unwrap_or_else(|| panic!("default plan provider missing: {id}"));
        if id != "duckduckgo" {
            assert!(entry.cors_readable, "{id} should be CORS-readable");
        }
    }
}

#[test]
fn web_search_only_providers_stay_outside_formal_ai_surface() {
    let map = compat();
    let registry = registry_by_id();
    let formal_ids: HashSet<&str> = map["formalAiProviders"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["id"].as_str().unwrap())
        .collect();

    for extra in map["webSearchOnlyProviders"].as_array().unwrap() {
        let id = extra["id"].as_str().unwrap();
        if id == "wc:*" {
            let wc: Vec<&RegistryEntry> = registry
                .values()
                .filter(|e| e.id.starts_with("wc:"))
                .collect();
            assert!(!wc.is_empty(), "expected web-capture providers");
            for entry in wc {
                assert_eq!(entry.access, "component");
            }
            continue;
        }
        assert!(registry.contains_key(id), "extra provider missing: {id}");
        assert!(
            !formal_ids.contains(id),
            "{id} should not be part of the FormalAI surface"
        );
    }
}
