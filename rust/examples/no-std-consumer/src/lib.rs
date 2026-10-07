#![no_std]
extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec::Vec};
use web_search::{get_provider_ids, merger::merge_results, MergeOptions, SearchResult};

/// Fuse three caller-fetched provider lists without a network or server runtime.
pub fn fuse(lists: BTreeMap<String, Vec<SearchResult>>) -> Vec<SearchResult> {
    merge_results(&lists, &MergeOptions::new())
}

/// Select paper providers from the shared catalog.
pub fn paper_providers() -> Vec<String> {
    get_provider_ids(Some("papers"))
}
