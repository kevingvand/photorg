use serde::{Deserialize, Serialize};

/// Audit log entry type for immutable decision tracking.
///
/// Entries are appended to an audit trail and never modified.
/// Each entry type captures all information necessary for compliance, debugging, and replay.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuditEntry {
    /// User made a culling decision (selected winner from pair)
    CullingDecision {
        timestamp: String,
        session_id: String,
        image_a: String,
        image_b: String,
        winner: String,
        reason: String,
    },

    /// Algorithm resolved conflicting ratings
    ConflictResolution {
        timestamp: String,
        session_id: String,
        image_a: String,
        image_b: String,
        resolution_method: String,
        winner: String,
        before_state: serde_json::Value,
        after_state: serde_json::Value,
    },

    /// Images were exported
    Export {
        timestamp: String,
        session_id: String,
        export_format: String,
        images_exported: usize,
        destination: String,
        result: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_culling_decision_serialization() {
        let entry = AuditEntry::CullingDecision {
            timestamp: "2026-06-18T23:00:00Z".to_string(),
            session_id: "sess-123".to_string(),
            image_a: "IMG_001".to_string(),
            image_b: "IMG_002".to_string(),
            winner: "IMG_001".to_string(),
            reason: "user_selected".to_string(),
        };

        let json = serde_json::to_string(&entry).expect("Failed to serialize");
        assert!(json.contains("\"type\":\"culling_decision\""));
        assert!(json.contains("\"winner\":\"IMG_001\""));
    }

    #[test]
    fn test_conflict_resolution_serialization() {
        let entry = AuditEntry::ConflictResolution {
            timestamp: "2026-06-18T23:00:00Z".to_string(),
            session_id: "sess-123".to_string(),
            image_a: "IMG_001".to_string(),
            image_b: "IMG_002".to_string(),
            resolution_method: "user_manual".to_string(),
            winner: "IMG_001".to_string(),
            before_state: serde_json::json!({ "rating_a": 0.5 }),
            after_state: serde_json::json!({ "rating_a": 1.0 }),
        };

        let json = serde_json::to_string(&entry).expect("Failed to serialize");
        assert!(json.contains("\"type\":\"conflict_resolution\""));
    }

    #[test]
    fn test_export_entry_serialization() {
        let entry = AuditEntry::Export {
            timestamp: "2026-06-18T23:00:00Z".to_string(),
            session_id: "sess-123".to_string(),
            export_format: "xmp_sidecars".to_string(),
            images_exported: 125,
            destination: "/path/to/catalog".to_string(),
            result: "success".to_string(),
        };

        let json = serde_json::to_string(&entry).expect("Failed to serialize");
        assert!(json.contains("\"type\":\"export\""));
        assert!(json.contains("\"images_exported\":125"));
    }
}
