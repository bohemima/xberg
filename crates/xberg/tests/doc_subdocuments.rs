//! Regression test for #2054: header, footer, footnote and comment text of a `.doc` reaches the output.
//!
//! A legacy Word 97 `.doc` keeps header, footer, footnote, comment and text-box text in
//! subdocuments after the body. The parser collected that text but the extractor only emitted the
//! body paragraphs whenever there were any, so everything else was silently dropped.
//! Footnotes and comments are now definition elements, headers and footers sit on their
//! own layers, and `content_filter` treats them as it does for `.docx`.

#![cfg(feature = "office")]

mod helpers;
use helpers::extract_bytes_document_blocking;

use xberg::core::config::{ContentFilterConfig, ExtractionConfig, OutputFormat};
use xberg::{ContentLayer, ExtractedDocument};

const DOC_MIME: &str = "application/msword";
const HEADERS: [&str; 3] = ["ODD HEADER TEXT", "EVEN HEADER TEXT", "FIRST PAGE HEADER TEXT"];
const FOOTERS: [&str; 3] = ["ODD FOOTER TEXT", "EVEN FOOTER TEXT", "FIRST PAGE FOOTER TEXT"];
const SECOND_FOOTNOTE: &str = "SECOND FOOTNOTE PARAGRAPH ONE SECOND FOOTNOTE PARAGRAPH TWO";

fn extract(config: &ExtractionConfig) -> ExtractedDocument {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/office/doc_subdocuments.doc");
    let bytes = std::fs::read(&path).expect("Word 97 subdocument fixture must be present");
    extract_bytes_document_blocking(&bytes, DOC_MIME, config).expect("extraction must succeed")
}

#[test]
fn footnotes_and_comments_follow_the_body_and_headers_stay_out_of_plain_content() {
    let content = extract(&ExtractionConfig::default()).content;

    let mut last = 0;
    for text in [
        "BODY PARAGRAPH ONE",
        "BODY PARAGRAPH TWO",
        "FIRST FOOTNOTE",
        SECOND_FOOTNOTE,
        "FIRST COMMENT",
        "SECOND COMMENT",
    ] {
        let at = content
            .find(text)
            .unwrap_or_else(|| panic!("{text:?} missing from {content:?}"));
        assert!(at >= last, "{text:?} is out of order in {content:?}");
        last = at;
    }
    assert!(content.contains("TEXT BOX TEXT"), "text box missing from {content:?}");
    for furniture in HEADERS.iter().chain(&FOOTERS) {
        assert!(!content.contains(furniture), "{furniture:?} leaked into {content:?}");
    }
}

#[test]
fn markdown_output_carries_footnotes_and_comments() {
    let config = ExtractionConfig {
        output_format: OutputFormat::Markdown,
        ..Default::default()
    };
    let content = extract(&config).content;

    for text in ["FIRST FOOTNOTE", SECOND_FOOTNOTE, "FIRST COMMENT", "SECOND COMMENT"] {
        assert!(content.contains(text), "{text:?} missing from {content:?}");
    }
}

fn layer_texts(doc: &ExtractedDocument, layer: ContentLayer) -> Vec<String> {
    let structure = doc.document.as_ref().expect("document structure was requested");
    structure
        .nodes
        .iter()
        .filter(|node| node.content_layer == layer)
        .filter_map(|node| node.content.text().map(str::to_string))
        .collect()
}

fn has_any(texts: &[String], needle: &str) -> bool {
    texts.iter().any(|t| t.contains(needle))
}

fn has_all(texts: &[String], expected: &[&str]) -> bool {
    expected.iter().all(|e| has_any(texts, e))
}

#[test]
fn headers_and_footers_sit_on_their_own_layers_and_the_content_filter_selects_them() {
    let filter = |include_headers, include_footers| {
        Some(ContentFilterConfig {
            include_headers,
            include_footers,
            ..Default::default()
        })
    };
    // (content filter, headers kept, footers kept): the same rule `.docx` follows.
    let cases = [
        (None, true, true),
        (filter(false, false), false, false),
        (filter(true, false), true, false),
        (filter(false, true), false, true),
    ];

    for (content_filter, headers, footers) in cases {
        let config = ExtractionConfig {
            include_document_structure: true,
            content_filter,
            ..Default::default()
        };
        let doc = extract(&config);
        let (header_layer, footer_layer) = (
            layer_texts(&doc, ContentLayer::Header),
            layer_texts(&doc, ContentLayer::Footer),
        );

        for (texts, expected, kept) in [(&header_layer, &HEADERS, headers), (&footer_layer, &FOOTERS, footers)] {
            if kept {
                assert!(has_all(texts, expected), "missing from {texts:?}");
            } else {
                assert!(texts.is_empty(), "filtered out, but kept: {texts:?}");
            }
        }
        assert!(
            !has_any(&header_layer, "FOOTER"),
            "footer text on Header: {header_layer:?}"
        );
        assert!(
            !has_any(&footer_layer, "HEADER"),
            "header text on Footer: {footer_layer:?}"
        );
    }
}
