# Photorg

Photorg is a desktop-first photo culling app focused on fast RAW+JPEG paired review, keyboard-driven rating/labeling, and goal-based best-shot selection.

## MVP Scope (Phase 1 — Locked)

Phase 1 scope is locked. The authoritative scope boundary, success metrics, test
strategy, and dependency chain live in [docs/MVP_SCOPE.md](./docs/MVP_SCOPE.md).
Per [ADR-004](./docs/DECISIONS.md#adr-004-mvp-scope-locked-to-phase-1), changes
to MVP scope require a PR against that document.

Headline capabilities in Phase 1:

- Paired RAW+JPEG preview as one logical unit
- Keyboard-first keep/reject/rating workflow
- Immutable source policy (no direct RAW/JPEG modification by default)
- Sidecar + internal database metadata model
- Goal-based rater workflow for tournament-style selection
- Single-user, single-catalog, single-directory per session

## Development

1. Install prerequisites (Node.js + Rust + platform tooling): https://tauri.app/start/prerequisites/
2. Install dependencies:
   ```bash
   npm install
   ```
3. Run desktop app:
   ```bash
   npm run tauri dev
   ```

## Documentation

- Setup guide: [SETUP.md](./SETUP.md)
- Architecture: [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md)
- Decisions log: [docs/DECISIONS.md](./docs/DECISIONS.md)
- **Testing Quick Start**: [TESTING_QUICKSTART.md](./TESTING_QUICKSTART.md)
- **Testing Strategy**: [docs/TESTING_STRATEGY.md](./docs/TESTING_STRATEGY.md)

## Testing

Run tests with:

```bash
# Frontend tests
npm run test:run

# Rust tests
npm run test:rust

# All tests
npm run test:all

# Coverage reports
npm run test:coverage          # Frontend coverage
npm run test:rust:coverage     # Rust coverage
npm run test:all:coverage      # Both
```

See [TESTING_QUICKSTART.md](./TESTING_QUICKSTART.md) for more details.
