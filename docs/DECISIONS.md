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

## ADR-004: MVP scope locked to Phase 1

- **Status:** Accepted
- **Decision:** Photorg's first shippable release is bounded by [docs/MVP_SCOPE.md](./MVP_SCOPE.md). That document enumerates the must-have capabilities, explicit non-features, success metrics, single-user assumption, test coverage strategy, and dependency chain across Phase 1 issues. Pull requests that implement deferred capabilities against the Phase 1 milestone will be rejected.
- **Rationale:** Without an explicit, document-backed boundary, feature discussions drift and post-MVP work blocks the critical path. Locking scope in a single source of truth keeps contributors (human and agent) aligned and makes the "done" condition for MVP unambiguous.
- **Consequences:**
  - Scope changes require a PR against `docs/MVP_SCOPE.md` with linked rationale.
  - Architectural changes that affect the single-user, immutable-source, sidecar/DB-split, or keyboard-first assumptions require a new ADR in this file.
  - Deferred issues remain open in the tracker but are not part of the Phase 1 milestone.
