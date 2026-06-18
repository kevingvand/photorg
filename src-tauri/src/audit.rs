use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Audit log entry type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuditEntry {
    /// User made a culling decision
    CullingDecision {
        timestamp: String,
        session_id: String,
        image_a: String,
        image_b: String,
        winner: String,
        reason: String,
    },
    /// Conflict was resolved
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

/// Audit logger for immutable append-only audit trail
pub struct AuditLogger {
    path: PathBuf,
}

impl AuditLogger {
    /// Create a new audit logger for the given catalog directory
    ///
    /// # Arguments
    /// * `catalog_dir` - Path to the catalog directory where audit.log will be stored at .photorg/audit.log
    pub fn new(catalog_dir: impl AsRef<Path>) -> std::io::Result<Self> {
        let audit_dir = catalog_dir.as_ref().join(".photorg");
        std::fs::create_dir_all(&audit_dir)?;

        Ok(AuditLogger {
            path: audit_dir.join("audit.log"),
        })
    }

    /// Log a culling decision
    pub fn log_culling_decision(
        &self,
        session_id: &str,
        image_a: &str,
        image_b: &str,
        winner: &str,
        reason: &str,
    ) -> std::io::Result<()> {
        let entry = AuditEntry::CullingDecision {
            timestamp: Utc::now().to_rfc3339(),
            session_id: session_id.to_string(),
            image_a: image_a.to_string(),
            image_b: image_b.to_string(),
            winner: winner.to_string(),
            reason: reason.to_string(),
        };

        self.append_entry(&entry)
    }

    /// Log a conflict resolution
    pub fn log_conflict_resolution(
        &self,
        session_id: &str,
        image_a: &str,
        image_b: &str,
        method: &str,
        winner: &str,
        before: serde_json::Value,
        after: serde_json::Value,
    ) -> std::io::Result<()> {
        let entry = AuditEntry::ConflictResolution {
            timestamp: Utc::now().to_rfc3339(),
            session_id: session_id.to_string(),
            image_a: image_a.to_string(),
            image_b: image_b.to_string(),
            resolution_method: method.to_string(),
            winner: winner.to_string(),
            before_state: before,
            after_state: after,
        };

        self.append_entry(&entry)
    }

    /// Log an export event
    pub fn log_export(
        &self,
        session_id: &str,
        format: &str,
        count: usize,
        destination: &str,
        result: &str,
    ) -> std::io::Result<()> {
        let entry = AuditEntry::Export {
            timestamp: Utc::now().to_rfc3339(),
            session_id: session_id.to_string(),
            export_format: format.to_string(),
            images_exported: count,
            destination: destination.to_string(),
            result: result.to_string(),
        };

        self.append_entry(&entry)
    }

    /// Append an entry to the audit log (append-only)
    fn append_entry(&self, entry: &AuditEntry) -> std::io::Result<()> {
        let json = serde_json::to_string(entry)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        writeln!(file, "{}", json)?;
        file.sync_all()?;

        // Log to main trace system as well
        tracing::info!(
            target: "photorg::audit",
            operation = "audit_entry_written",
            result = "success",
            entry_type = ?entry,
            "Audit entry logged"
        );

        Ok(())
    }

    /// Read all audit entries from the log
    ///
    /// This is primarily for testing and diagnostic purposes.
    pub fn read_entries(&self) -> std::io::Result<Vec<AuditEntry>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let content = std::fs::read_to_string(&self.path)?;
        let mut entries = Vec::new();

        for line in content.lines() {
            if let Ok(entry) = serde_json::from_str(line) {
                entries.push(entry);
            }
        }

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use uuid::Uuid;

    #[test]
    fn test_audit_logger_creation() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let _logger = AuditLogger::new(temp_dir.path()).expect("Failed to create logger");

        let audit_path = temp_dir.path().join(".photorg").join("audit.log");
        assert!(!audit_path.exists()); // Log doesn't exist until first write
    }

    #[test]
    fn test_culling_decision_logging() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let logger = AuditLogger::new(temp_dir.path()).expect("Failed to create logger");
        let session_id = Uuid::new_v4().to_string();

        logger
            .log_culling_decision(&session_id, "IMG_001", "IMG_002", "IMG_001", "user_selected")
            .expect("Failed to log");

        let entries = logger.read_entries().expect("Failed to read entries");
        assert_eq!(entries.len(), 1);

        if let AuditEntry::CullingDecision { winner, .. } = &entries[0] {
            assert_eq!(winner, "IMG_001");
        } else {
            panic!("Expected CullingDecision entry");
        }
    }

    #[test]
    fn test_audit_log_append_only() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let logger = AuditLogger::new(temp_dir.path()).expect("Failed to create logger");
        let session_id = Uuid::new_v4().to_string();

        // Write first entry
        logger
            .log_culling_decision(&session_id, "IMG_001", "IMG_002", "IMG_001", "user_selected")
            .expect("Failed to log first");

        // Write second entry
        logger
            .log_culling_decision(&session_id, "IMG_003", "IMG_004", "IMG_004", "user_selected")
            .expect("Failed to log second");

        // Verify both entries exist
        let entries = logger.read_entries().expect("Failed to read entries");
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_conflict_resolution_logging() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let logger = AuditLogger::new(temp_dir.path()).expect("Failed to create logger");
        let session_id = Uuid::new_v4().to_string();

        let before = serde_json::json!({ "rating_a": 0.5, "rating_b": 0.5 });
        let after = serde_json::json!({ "rating_a": 1.0, "rating_b": 0.0 });

        logger
            .log_conflict_resolution(
                &session_id,
                "IMG_001",
                "IMG_002",
                "user_manual",
                "IMG_001",
                before,
                after,
            )
            .expect("Failed to log");

        let entries = logger.read_entries().expect("Failed to read entries");
        assert_eq!(entries.len(), 1);
    }

    #[test]
    fn test_export_logging() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let logger = AuditLogger::new(temp_dir.path()).expect("Failed to create logger");
        let session_id = Uuid::new_v4().to_string();

        logger
            .log_export(&session_id, "xmp_sidecars", 125, "/path/to/catalog", "success")
            .expect("Failed to log");

        let entries = logger.read_entries().expect("Failed to read entries");
        assert_eq!(entries.len(), 1);

        if let AuditEntry::Export {
            images_exported,
            result,
            ..
        } = &entries[0]
        {
            assert_eq!(*images_exported, 125);
            assert_eq!(result, "success");
        } else {
            panic!("Expected Export entry");
        }
    }
}
