# MVP Scope Lock - Phase 1

- **Status:** Accepted
- **Date:** 2026-06-18
- **Related issue:** Closes [#9](https://github.com/kevingvand/photorg/issues/9)
- **Related ADRs:** ADR-001, ADR-002, ADR-003

## Decision

Photorg Phase 1 (MVP) is explicitly locked to a **single-user, single-catalog, single-directory** desktop workflow focused on fast culling throughput and interoperable metadata output.

## Must-Have Features (In Scope)

1. **Paired preview:** RAW+JPEG are handled as one logical review unit.
2. **Keyboard culling:** Keyboard-first keep/reject/rating flow is the default interaction model.
3. **Sidecar output:** Final culling outcomes are written to sidecar metadata (`xmp:Rating`, `xmp:Label`) while internal state remains in the local app DB.

## Explicit Non-Features (Out of Scope for MVP)

- Cloud sync and remote collaboration
- Plugin ecosystem/extensibility framework
- Advanced color correction or editing workflows
- Multi-user concurrency
- Multi-location catalog management (deferred beyond single-directory workflows)

## Success Metrics (MVP Complete Thresholds)

- Sustained culling pace target: **1000 photos/hour** on a typical shoot
- Preview responsiveness target: **sub-100ms** perceived preview load during culling navigation
- Workflow target: full keyboard-only culling completion without required mouse interaction
- Data integrity target: source files remain immutable by default (per ADR-001)

## Phase 1 Issue Dependency Chain

This scope lock is the upstream gate for all Phase 1 implementation issues.

1. **Scope lock (this issue #9)**
2. Pairing + preview loading implementation
3. Keyboard culling interaction and state transitions
4. Sidecar writer + metadata persistence split validation
5. Throughput/performance tuning against MVP success metrics
6. End-to-end MVP completion validation and sign-off

All Phase 1 issues after #9 should reference this chain and declare which predecessor item they depend on.

## Initial Test Coverage Strategy (Agreed)

- **Unit tests:** pairing logic, keyboard action mapping, state transitions, metadata serialization.
- **Integration tests:** UI-to-domain flow for culling actions, sidecar write behavior, DB/sidecar split consistency.
- **End-to-end tests:** representative single-directory culling workflow from ingest/open to final sidecar outputs, including keyboard-only execution path.

## Rationale

Locking scope protects delivery focus and avoids accidental expansion into collaboration, editing, or platform concerns that are not required for the first usable release. The selected boundaries align with existing ADRs and prioritize high-volume culling speed, predictable metadata output, and operational simplicity for initial users.
