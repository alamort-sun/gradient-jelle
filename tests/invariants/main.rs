//! Jelle stateless coordinator invariant tests.
//!
//! These tests use grep-based static analysis to verify that jelle
//! maintains its boundary conditions relative to the coordinator.

#[test]
fn no_coordinator_dependency() {
    let src_files = ["src/jelle.rs", "src/orchestrator.rs", "src/weather.rs", "src/dialogue.rs",
                     "src/category_map.rs", "src/codec_gate.rs", "src/jepa_moe.rs",
                     "src/persistence.rs", "src/prediction.rs", "src/boundary.rs"];

    for file in src_files {
        let content = std::fs::read_to_string(file).unwrap();
        assert!(!content.contains("spacetimedb"), "{} imports spacetimedb", file);
        assert!(!content.contains("gradient_codec"), "{} imports gradient_codec", file);
        assert!(!content.contains("coordinator"), "{} imports coordinator", file);
    }
}

#[test]
fn no_telemetry() {
    let src_files = ["src/jelle.rs", "src/orchestrator.rs", "src/weather.rs", "src/dialogue.rs",
                     "src/category_map.rs", "src/codec_gate.rs", "src/jepa_moe.rs",
                     "src/persistence.rs", "src/prediction.rs", "src/boundary.rs"];

    for file in src_files {
        let content = std::fs::read_to_string(file).unwrap();
        assert!(!content.contains("use log"), "{} imports log", file);
        assert!(!content.contains("use tracing"), "{} imports tracing", file);
    }
}

#[test]
fn no_coordinator_writes() {
    // Jelle should not write to coordinator space-time. Its persistence is local
    // (device and fog layer), not coordinator storage.
    let src_files = ["src/jelle.rs", "src/orchestrator.rs", "src/weather.rs", "src/dialogue.rs",
                     "src/category_map.rs", "src/codec_gate.rs", "src/jepa_moe.rs",
                     "src/persistence.rs", "src/prediction.rs", "src/boundary.rs"];

    for file in src_files {
        let content = std::fs::read_to_string(file).unwrap();
        assert!(!content.contains("spacetimedb"), "{} uses spacetimedb", file);
        assert!(!content.contains("call_reducer"), "{} calls a reducer", file);
        assert!(!content.contains("insert_row"), "{} inserts a row", file);
        assert!(!content.contains("update_row"), "{} updates a row", file);
        assert!(!content.contains("delete_row"), "{} deletes a row", file);
    }
}
