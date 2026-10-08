#![cfg(all(feature = "ocr", feature = "pdf"))]

mod helpers;

use helpers::{extract_uri_document_blocking, get_test_file_path, skip_if_missing};
use xberg::core::config::{ExtractionConfig, OcrConfig, OutputFormat};
use xberg::types::TesseractConfig;

fn config() -> ExtractionConfig {
    ExtractionConfig {
        force_ocr: true,
        use_cache: false,
        output_format: OutputFormat::Markdown,
        ocr: Some(OcrConfig {
            backend: "tesseract".to_string(),
            language: vec!["eng".to_string()],
            tesseract_config: Some(TesseractConfig {
                use_cache: false,
                enable_table_detection: true,
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn assert_columned_prose_survives(config: &ExtractionConfig) {
    let path = get_test_file_path("pdf/scanned.pdf");
    let document = extract_uri_document_blocking(path, Some("application/pdf"), config)
        .expect("forced OCR of the columned-prose fixture must succeed");

    assert!(
        document.tables.is_empty(),
        "opaque fixture 36b85460d4 must not be emitted as one page-sized table: {:?}",
        document
            .tables
            .iter()
            .map(|table| table.cells.len())
            .collect::<Vec<_>>()
    );
    assert!(
        document.content.split_whitespace().count() >= 400,
        "rejecting the false table must retain the page's prose"
    );
    assert!(document.content.contains("STATE UNIVERSITIES"));
    assert!(document.content.contains("SUAACTION"));
    assert!(document.content.contains("$30 .$25"));
}

#[test]
fn opaque_36b85460d4_stays_prose_without_layout_detection() {
    if skip_if_missing("pdf/scanned.pdf") {
        return;
    }
    assert_columned_prose_survives(&config());
}

#[cfg(feature = "layout-detection")]
#[test]
fn opaque_36b85460d4_stays_prose_with_layout_detection() {
    use xberg::core::config::layout::LayoutDetectionConfig;

    if skip_if_missing("pdf/scanned.pdf") {
        return;
    }
    let mut extraction_config = config();
    extraction_config.layout = Some(LayoutDetectionConfig::default());
    assert_columned_prose_survives(&extraction_config);
}
