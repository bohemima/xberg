use super::*;

#[test]
fn steady_adapter_is_not_registered_without_explicit_selection() {
    let config = BenchmarkConfig::default();
    let should_init = |_name: &str| true;
    let context = XbergRegistration {
        config: &config,
        batch_mode: false,
        has_explicit_frameworks: false,
        ocr: false,
        format: OutputFormat::Markdown,
        should_init: &should_init,
    };
    let mut registry = AdapterRegistry::new();

    assert!(!register_steady_xberg_adapter(
        &mut registry,
        &context,
        XbergPipeline::Layout,
        XbergPdfBackend::Native,
    ));
    assert!(registry.is_empty());
}
