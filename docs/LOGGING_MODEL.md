# Photorg Logging & Error Reporting Model

This document specifies how Photorg logs operations, errors, and user actions for debugging, audit, and diagnosability.

## 1. Log Levels

Configurable at startup. Default: `INFO`.

| Level | Purpose | Examples |
|-------|---------|----------|
| `ERROR` | Unrecoverable failures requiring user action | File I/O errors, DB corruption, permission denied |
| `WARN` | Recoverable anomalies or degraded states | Slow operation (>1s), retry exhausted, fallback used |
| `INFO` | Normal operational events | Image ingested, session started, export completed |
| `DEBUG` | Detailed execution flow for troubleshooting | Function entry/exit, value changes, decision points |

## 2. Structured Logging Format

All logs use JSON to enable parsing, filtering, and forwarding. Entries contain:

```json
{
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
    "image_b": "/path/to/IMG_001.JPG",
    "profile": "default"
  }
}
```

### Field Reference

| Field | Type | Required | Purpose |
|-------|------|----------|---------|
| `timestamp` | ISO 8601 | Yes | UTC time of log entry |
| `level` | string | Yes | ERROR, WARN, INFO, DEBUG |
| `target` | string | Yes | Module/component (e.g., `photorg::ingest`, `photorg::db`) |
| `user_id` | string | Conditional* | Session hash or user identifier (omit if unavailable) |
| `session_id` | uuid | Conditional* | Session UUID for correlation (omit if unavailable) |
| `operation` | string | Yes | What happened (verb: `image_pair_processed`, `conflict_detected`) |
| `result` | string | Yes | `success`, `failure`, `partial`, `skipped` |
| `duration_ms` | number | No | Elapsed time in milliseconds (for performance tracking) |
| `context` | object | No | Operation-specific data (paths, counts, flags) |
| `error` | object | No | Error details (see Error Format section) |

*For privacy and startup phases where session isn't available, these are optional.

## 3. Log Locations

Logs are written to user and catalog directories to prevent permission issues and enable per-catalog auditing:

### App Logs
- **Path**: `~/.photorg/logs/app.log`
- **Content**: Application lifecycle, ingest, previews, database operations
- **Ownership**: User (application creates directory at first run)
- **Frequency**: Every operational event
- **Retention**: 30 days by default (configurable via settings)

### Audit Logs
- **Path**: `<catalog_dir>/.photorg/audit.log`
- **Content**: Immutable record of decisions and exports (see Audit Log section)
- **Ownership**: Catalog user
- **Frequency**: Every user decision and export
- **Retention**: Until manually pruned; never auto-deleted

## 4. Log Rotation

App logs rotate based on file size to prevent unbounded growth.

### Rotation Policy
- **Trigger**: File size ≥ 100 MB
- **Action**: Rename `app.log` → `app.log.YYYY-MM-DD.HHmmss`; start new `app.log`
- **Archive**: Compressed (gzip) and moved to `~/.photorg/logs/archive/`
- **Retention**: Keep files modified within last 30 days (by mtime)
- **Cleanup**: Remove files older than 30 days daily at startup

### Retention Configuration
Users can override default via settings:
```json
{
  "logging": {
    "level": "INFO",
    "retention_days": 30,
    "max_file_size_mb": 100
  }
}
```

## 5. Structured Error Format

When logging errors, include categorization and context for frontend display:

```json
{
  "timestamp": "2026-06-18T23:34:25.511Z",
  "level": "ERROR",
  "target": "photorg::xmp",
  "operation": "write_sidecar",
  "result": "failure",
  "error": {
    "category": "system",
    "code": "PERMISSION_DENIED",
    "message": "Permission denied writing to sidecar",
    "user_message": "Cannot write to image sidecars. Check file permissions.",
    "recovery": "Try selecting a different folder or running as administrator.",
    "context": {
      "path": "/path/to/IMG_001.XMP",
      "errno": 13
    },
    "backtrace": "photorg::xmp::write_sidecar:42 → photorg::fs::write_file:108"
  }
}
```

### Error Categories

#### User Errors
**Definition**: Invalid input or user actions that violate constraints.

**Examples**:
- Missing required files
- Invalid import paths
- Unsupported file formats
- Out-of-range parameter values

**UI Treatment**:
- Clear, actionable message (e.g., "Image folder not found. Check the path and try again.")
- Recovery suggestion (e.g., "Try using a different folder")
- No stack trace
- No technical jargon

**Log Level**: `WARN` or `ERROR` (depends on severity)

**Example**:
```json
{
  "error": {
    "category": "user",
    "code": "MISSING_FILE",
    "message": "Image file not found",
    "user_message": "Could not find image file IMG_001.CR3.",
    "recovery": "Check the folder path and file name.",
    "context": { "path": "/nonexistent/IMG_001.CR3" }
  }
}
```

#### System Errors
**Definition**: Infrastructure failures (file system, database, permissions) outside user control.

**Examples**:
- Disk full
- Database corruption
- Permission denied
- Out of memory
- Network timeout (if applicable)

**UI Treatment**:
- Explain what failed and why
- Provide diagnostic info (space available, DB version)
- Suggest recovery ("Restart the app" or "Check disk space")
- Include option to send diagnostic report

**Log Level**: `ERROR`

**Example**:
```json
{
  "error": {
    "category": "system",
    "code": "DISK_FULL",
    "message": "No space left on device",
    "user_message": "Disk is full. Free up space and try again.",
    "recovery": "Delete unused files or move to a drive with more space.",
    "context": {
      "available_bytes": 0,
      "required_bytes": 104857600
    }
  }
}
```

#### Algorithm Errors
**Definition**: Failures in domain logic (rater, conflict detection, pairing).

**Examples**:
- Rater algorithm failure
- Unresolvable conflict
- Data invariant violation
- Pairing logic failure

**UI Treatment**:
- Non-blocking warnings to user
- Full technical details in logs for developer review
- Suggest workaround ("Try re-importing" or "Report this issue")

**Log Level**: `ERROR`

**Example**:
```json
{
  "error": {
    "category": "algorithm",
    "code": "UNRESOLVABLE_CONFLICT",
    "message": "Cannot auto-resolve conflict: both images rated equally",
    "user_message": "Conflict detected. Both images are equally good. Please choose one manually.",
    "recovery": "View conflict details and select a winner.",
    "context": {
      "image_a": "IMG_001",
      "image_b": "IMG_002",
      "rating_a": 0.5,
      "rating_b": 0.5
    },
    "backtrace": "photorg::rater::resolve:156"
  }
}
```

## 6. Audit Logging

A separate, immutable append-only log for decisions and exports. Used to:
- Audit all user actions on images
- Trace export history
- Recover decision state
- Validate data integrity

### Audit Log Entries

Written to `<catalog_dir>/.photorg/audit.log` (one JSON entry per line):

#### Culling Decision
```json
{
  "timestamp": "2026-06-18T23:34:25.511Z",
  "type": "culling_decision",
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "image_a": "IMG_001",
  "image_b": "IMG_002",
  "winner": "IMG_001",
  "reason": "user_selected"
}
```

#### Conflict Resolution
```json
{
  "timestamp": "2026-06-18T23:34:25.511Z",
  "type": "conflict_resolution",
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "image_a": "IMG_001",
  "image_b": "IMG_002",
  "resolution_method": "user_manual",
  "winner": "IMG_001",
  "before_state": { "rating_a": 0.5, "rating_b": 0.5 },
  "after_state": { "rating_a": 1.0, "rating_b": 0.0 }
}
```

#### Export Event
```json
{
  "timestamp": "2026-06-18T23:34:25.511Z",
  "type": "export",
  "session_id": "550e8400-e29b-41d4-a716-446655440000",
  "export_format": "xmp_sidecars",
  "images_exported": 125,
  "destination": "<catalog_dir>",
  "result": "success"
}
```

### Audit Log Properties
- **Immutability**: Append-only; no edits or deletions
- **Format**: One JSON object per line (JSONL)
- **No PII**: Only session_id and image identifiers
- **Ordering**: Strictly chronological by file position
- **Ownership**: Catalog user

## 7. Privacy & Sanitization

### What NOT to log:
- Raw file paths (hash instead, or use relative paths within catalog)
- Pixel data or image content
- Raw XMP or EXIF values
- User names or email addresses
- Absolute system paths outside catalog

### What IS safe to log:
- Relative paths within catalog (e.g., `subdir/IMG_001.CR3`)
- Image identifiers (filename or hash)
- Metadata counts (e.g., "10 ratings applied")
- Operation names and results
- Relative timestamps within session
- Session UUIDs (not tied to user identity)

### Sanitization Rules

1. **Paths**: Log relative path from catalog root or hash
   ```
   ✓ GOOD: "relative/path/IMG_001.CR3"
   ✗ BAD:  "/Users/alice/Pictures/Catalog/IMG_001.CR3"
   ```

2. **Sensitive data**: Omit or hash
   ```
   ✓ GOOD: "xmp_hash": "a3f2b1..."
   ✗ BAD:  "xmp_content": "<rdf:RDF>..."
   ```

3. **Error messages**: Generic messages in logs, detailed in UI only when needed
   ```
   ✓ GOOD: { "level": "ERROR", "message": "File write failed" }
   ✗ BAD:  { "level": "ERROR", "message": "Permission denied for /Users/alice/..." }
   ```

## 8. UI Error Display

### Error Dialog Component

When an error occurs, display a modal or toast with:

1. **Friendly Title**
   - No technical jargon
   - Explains what happened in user terms

2. **Description**
   - 1–2 sentences explaining the issue
   - Actionable recovery step(s)
   - Example: "Could not write to image sidecars. Check that the folder is not read-only."

3. **Recovery Actions** (buttons)
   - "Try Again" (retry the operation)
   - "Browse Folder" (let user select different location)
   - "Send Diagnostic Report" (optional)

4. **Details** (collapsible, dev-only)
   - Technical error code
   - Full error message
   - Option to view logs

### Example: User Error (Missing File)
```
❌ File Not Found

Could not find image file IMG_001.CR3. Check that the folder path is correct 
and the file hasn't been moved or deleted.

[  Try Again  ] [ Browse Folder ] [ Send Report ]
```

### Example: System Error (Disk Full)
```
❌ Disk Full

Cannot continue—disk is full. Free up space and try again. 
Need 100 MB, have 0 MB available.

[  Try Again  ] [ Open Disk Settings ] [ Send Report ]
```

### Diagnostic Report

When user clicks "Send Diagnostic Report":
1. Create `.photorg-diagnostic-report.zip` containing:
   - Last 100 lines of app.log
   - Last 100 lines of audit.log (if applicable)
   - Error details (error code, stack trace)
   - System info (OS, available disk, RAM)
2. Open folder containing the zip
3. Instructions: "Share this file with the developer for debugging"

## 9. Log Configuration & Control

### Startup Initialization

On app startup:
1. Create `~/.photorg/` directory if missing (user owns it)
2. Create `~/.photorg/logs/` if missing
3. Read log level from settings (default: `INFO`)
4. Initialize file appender to `~/.photorg/logs/app.log`
5. Clean up logs older than retention period
6. Start logging

### Runtime Control (Settings)

```json
{
  "logging": {
    "level": "INFO",
    "enabled": true,
    "retention_days": 30,
    "max_file_size_mb": 100,
    "send_crash_reports": false
  }
}
```

## 10. Testing Strategy

### Unit Tests

- **Log format**: Parse JSON output, verify all required fields present
- **Error categorization**: Verify error types are correctly categorized
- **Sanitization**: Ensure sensitive paths/data not in logs

### Integration Tests

- **File I/O**: Write logs to real file, verify rotation behavior
- **Audit immutability**: Append to audit log, verify no data loss or corruption
- **Retention**: Verify old logs cleaned up after retention period

### Simulation Tests

- **User error**: Try to open nonexistent file, capture error log
- **System error**: Simulate full disk, capture error log with recovery suggestion
- **Algorithm error**: Trigger unresolvable conflict, verify algorithm error categorization
- **Stress**: Rapid logging at high throughput, verify no data loss

### Example Test

```rust
#[test]
fn test_structured_log_format() {
    let json = r#"
    {
      "timestamp": "2026-06-18T23:34:25.511Z",
      "level": "INFO",
      "target": "photorg::ingest",
      "operation": "image_pair_processed",
      "result": "success"
    }
    "#;
    
    let parsed: serde_json::Value = serde_json::from_str(json).expect("Valid JSON");
    assert_eq!(parsed["level"].as_str(), Some("INFO"));
    assert_eq!(parsed["operation"].as_str(), Some("image_pair_processed"));
}
```

## 11. Migration & Versioning

### Log Format Versioning

If the log format changes in future releases, include a `version` field:
```json
{
  "version": "1.0",
  "timestamp": "2026-06-18T23:34:25.511Z",
  ...
}
```

### Backward Compatibility

Apps reading logs must handle missing or extra fields gracefully.

## 12. Performance Considerations

- **Async logging**: Writes to file happen off the main thread
- **Buffering**: Small batches of logs buffered before write
- **Sampling**: DEBUG logs can be sampled (not every entry written) in high-throughput scenarios
- **No I/O in hot paths**: Logging is deferred, never synchronous in UI threads

## 13. Open Questions & Future Work

- **Crash reporting**: Should core dumps or crash logs be sent home? (Deferred post-MVP)
- **Log forwarding**: Could logs be forwarded to external service? (Deferred post-MVP)
- **Real-time log viewer**: UI dashboard showing live logs? (Deferred post-MVP)
- **Encryption**: Should audit logs be encrypted? (Deferred post-MVP)
