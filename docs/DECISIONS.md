# Architecture Decisions

## ADR-001: Immutable source files by default

- **Status:** Accepted
- **Decision:** RAW and JPEG source files are not modified during culling by default.
- **Rationale:** Preserves trust, avoids accidental source mutation, and keeps workflows reversible.

## ADR-002: Sidecar + DB metadata split

- **Status:** Accepted
- **Decision:** Interoperable final outcomes live in sidecar metadata; operational rater internals live in app DB.
- **Rationale:** Keeps metadata portable while preserving performance and algorithm auditability.

## ADR-003: Keyboard-first culling workflow

- **Status:** Proposed
- **Decision:** Primary MVP interaction is shortcut-driven review/rating with minimal mouse dependency.
- **Rationale:** Aligns with high-volume professional culling workflows.
