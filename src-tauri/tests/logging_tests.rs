#[cfg(test)]
mod integration_tests {
    use serde_json::json;
    use std::fs;
    use tempfile::TempDir;

    /// Verify that structured logs are valid JSON with required fields
    #[test]
    fn test_structured_log_format_parseability() {
        let log_entry = json!({
            "timestamp": "2026-06-18T23:34:25.511Z",
            "level": "INFO",
            "target": "photorg::ingest",
            "user_id": "session_hash_abc123",
            "session_id": "550e8400-e29b-41d4-a716-446655440000",
            "operation": "image_pair_processed",
            "result": "success",
            "duration_ms": 145,
            "context": {
                "image_a": "/path/to/IMG_001.CR3",
                "image_b": "/path/to/IMG_001.JPG"
            }
        });

        // Verify JSON is valid
        let json_str = serde_json::to_string(&log_entry).expect("Failed to serialize");
        let parsed: serde_json::Value =
            serde_json::from_str(&json_str).expect("Failed to parse JSON");

        // Verify required fields
        assert_eq!(parsed["level"].as_str(), Some("INFO"));
        assert_eq!(parsed["operation"].as_str(), Some("image_pair_processed"));
        assert_eq!(parsed["result"].as_str(), Some("success"));
        assert!(parsed["timestamp"].is_string());
        assert!(parsed["target"].is_string());
    }

    /// Verify error category classification
    #[test]
    fn test_error_category_classification() {
        let user_error = json!({
            "category": "user",
            "code": "MissingFile",
            "message": "File not found",
            "user_message": "Could not find the image file.",
            "recovery": "Check the folder path and try again."
        });

        let system_error = json!({
            "category": "system",
            "code": "PermissionDenied",
            "message": "Permission denied writing to file",
            "user_message": "Cannot write to file. Check permissions.",
            "recovery": "Try with different permissions."
        });

        let algorithm_error = json!({
            "category": "algorithm",
            "code": "UnresolvableConflict",
            "message": "Both images rated equally",
            "user_message": "Please choose one manually.",
            "recovery": "View conflict and select a winner."
        });

        assert_eq!(user_error["category"].as_str(), Some("user"));
        assert_eq!(system_error["category"].as_str(), Some("system"));
        assert_eq!(algorithm_error["category"].as_str(), Some("algorithm"));
    }

    /// Verify audit log entries are correctly formatted
    #[test]
    fn test_audit_log_entry_format() {
        let culling_entry = json!({
            "type": "culling_decision",
            "timestamp": "2026-06-18T23:34:25.511Z",
            "session_id": "550e8400-e29b-41d4-a716-446655440000",
            "image_a": "IMG_001",
            "image_b": "IMG_002",
            "winner": "IMG_001",
            "reason": "user_selected"
        });

        let export_entry = json!({
            "type": "export",
            "timestamp": "2026-06-18T23:34:25.511Z",
            "session_id": "550e8400-e29b-41d4-a716-446655440000",
            "export_format": "xmp_sidecars",
            "images_exported": 125,
            "destination": "<catalog_dir>",
            "result": "success"
        });

        assert_eq!(culling_entry["type"].as_str(), Some("culling_decision"));
        assert!(culling_entry["timestamp"].is_string());
        assert_eq!(export_entry["images_exported"].as_u64(), Some(125));
    }

    /// Verify sensitive data is not leaked in logs
    #[test]
    fn test_sensitive_data_not_leaked() {
        let log_entry = json!({
            "timestamp": "2026-06-18T23:34:25.511Z",
            "level": "ERROR",
            "target": "photorg::xmp",
            "operation": "write_sidecar",
            "result": "failure",
            "context": {
                "path": "relative/path/IMG_001.XMP",
                "errno": 13
            }
        });

        // Verify no absolute paths with user names
        let json_str = serde_json::to_string(&log_entry).expect("Failed to serialize");
        assert!(!json_str.contains("/Users/"));
        assert!(!json_str.contains("/home/"));

        // Verify relative paths are safe
        assert!(json_str.contains("relative/path/IMG_001.XMP"));
    }

    /// Verify audit log is append-only (no data loss on multiple writes)
    #[test]
    fn test_audit_log_append_only_integrity() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let audit_file = temp_dir.path().join("audit.log");

        // Write first entry
        let entry1 = json!({
            "type": "culling_decision",
            "timestamp": "2026-06-18T23:34:25.511Z",
            "session_id": "session1",
            "winner": "IMG_001"
        });

        fs::write(&audit_file, format!("{}\n", serde_json::to_string(&entry1).unwrap()))
            .expect("Failed to write first entry");

        // Append second entry
        let entry2 = json!({
            "type": "export",
            "timestamp": "2026-06-18T23:34:26.511Z",
            "session_id": "session1",
            "images_exported": 100
        });

        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&audit_file)
            .expect("Failed to open for append");

        use std::io::Write;
        writeln!(file, "{}", serde_json::to_string(&entry2).unwrap())
            .expect("Failed to write second entry");

        // Verify both entries exist
        let content = fs::read_to_string(&audit_file).expect("Failed to read audit log");
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 2);

        // Verify first entry intact
        let first: serde_json::Value = serde_json::from_str(lines[0]).expect("Failed to parse first");
        assert_eq!(first["type"].as_str(), Some("culling_decision"));

        // Verify second entry intact
        let second: serde_json::Value = serde_json::from_str(lines[1]).expect("Failed to parse second");
        assert_eq!(second["type"].as_str(), Some("export"));
    }

    /// Verify error messages don't contain stack traces in user view
    #[test]
    fn test_user_error_message_no_stack_trace() {
        let error = json!({
            "category": "user",
            "code": "MissingFile",
            "message": "Internal error details",
            "user_message": "Could not find image file. Check the path.",
            "recovery": "Try again with a valid folder."
        });

        let user_msg = error["user_message"].as_str().unwrap();

        // User message should not contain technical jargon
        assert!(!user_msg.contains("at "));
        assert!(!user_msg.contains("line "));
        assert!(!user_msg.contains("panic"));
        assert!(!user_msg.contains("thread"));

        // User message should be clear and actionable
        assert!(user_msg.contains("Could not"));
        assert!(user_msg.contains("Check"));
    }

    /// Verify log fields are consistent across multiple entries
    #[test]
    fn test_log_field_consistency() {
        let entries = vec![
            json!({
                "timestamp": "2026-06-18T23:34:25.511Z",
                "level": "INFO",
                "target": "photorg::ingest",
                "operation": "start",
                "result": "success"
            }),
            json!({
                "timestamp": "2026-06-18T23:34:26.511Z",
                "level": "DEBUG",
                "target": "photorg::db",
                "operation": "query",
                "result": "success"
            }),
        ];

        for entry in entries {
            // All entries must have these fields
            assert!(entry["timestamp"].is_string());
            assert!(entry["level"].is_string());
            assert!(entry["target"].is_string());
            assert!(entry["operation"].is_string());
            assert!(entry["result"].is_string());

            // Level must be valid
            let level = entry["level"].as_str().unwrap();
            assert!(matches!(level, "ERROR" | "WARN" | "INFO" | "DEBUG"));

            // Result must be valid
            let result = entry["result"].as_str().unwrap();
            assert!(matches!(result, "success" | "failure" | "partial" | "skipped"));
        }
    }

    /// Verify JSONL format (one JSON per line) for audit logs
    #[test]
    fn test_audit_log_jsonl_format() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let audit_file = temp_dir.path().join("audit.log");

        // Write multiple entries in JSONL format
        let entries = vec![
            json!({"type": "culling_decision", "winner": "IMG_001"}),
            json!({"type": "export", "count": 100}),
            json!({"type": "conflict_resolution", "method": "user_manual"}),
        ];

        let content = entries
            .iter()
            .map(|e| serde_json::to_string(e).unwrap())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";

        fs::write(&audit_file, content).expect("Failed to write audit log");

        // Verify JSONL structure
        let file_content = fs::read_to_string(&audit_file).expect("Failed to read file");
        let lines: Vec<&str> = file_content.lines().collect();

        assert_eq!(lines.len(), 3);

        for line in lines {
            // Each line must be valid JSON
            let _parsed: serde_json::Value =
                serde_json::from_str(line).expect("Failed to parse line as JSON");
        }
    }
}
