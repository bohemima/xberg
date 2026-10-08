use super::*;

#[test]
fn old_framework_provenance_deserializes_without_thread_budget() {
    let provenance: FrameworkProvenance = serde_json::from_value(serde_json::json!({
        "name": "xberg-markdown-baseline-batch",
        "version": "1.0.0",
        "executable": null,
        "models": [],
        "batch_capability": null,
        "requested_workers": 4,
        "effective_workers": null,
        "worker_semantics": "legacy",
        "effective_warmup_iterations": 0,
        "eligible_documents": 4,
        "batch_partitions": 1
    }))
    .unwrap();

    assert_eq!(provenance.configured_thread_budget, None);
}
