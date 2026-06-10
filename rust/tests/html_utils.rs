//! Integration tests for the shared HTML text utilities (Rust parity with
//! `tests/html-utils.test.js`).

use web_search::providers::{clean_text, decode_html_entities, strip_html};

#[test]
fn decodes_named_entities() {
    assert_eq!(
        decode_html_entities("a &amp; b &lt;c&gt; &quot;d&quot; &#39;e&#39;"),
        "a & b <c> \"d\" 'e'"
    );
    assert_eq!(decode_html_entities("x&nbsp;y"), "x y");
    assert_eq!(decode_html_entities("a&hellip;"), "a…");
    assert_eq!(decode_html_entities("a&mdash;b&ndash;c"), "a—b–c");
}

#[test]
fn decodes_numeric_and_hex_entities() {
    assert_eq!(decode_html_entities("&#169;"), "©");
    assert_eq!(decode_html_entities("&#xA9;"), "©");
}

#[test]
fn leaves_unresolvable_references_untouched() {
    // Out-of-range code point: keep the original token rather than crashing.
    assert_eq!(decode_html_entities("&#xFFFFFFFF;"), "&#xFFFFFFFF;");
    assert_eq!(decode_html_entities(""), "");
}

#[test]
fn strips_tags() {
    assert_eq!(strip_html("<b>hi</b> <a href=\"x\">there</a>"), "hi there");
    assert_eq!(strip_html("plain"), "plain");
}

#[test]
fn clean_text_strips_decodes_and_collapses() {
    assert_eq!(clean_text("  <b>A</b>   B&amp;C  "), "A B&C");
    assert_eq!(clean_text("<p>line\n\n  two</p>"), "line two");
    assert_eq!(clean_text(""), "");
}
