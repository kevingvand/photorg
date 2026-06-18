# Photorg

Photorg is a desktop-first photo culling app focused on fast RAW+JPEG paired review, keyboard-driven rating/labeling, and goal-based best-shot selection.

## Current MVP Scope

Phase 1 MVP scope is now locked in [docs/MVP_SCOPE.md](./docs/MVP_SCOPE.md).

- Must-haves: paired preview, keyboard culling, best-shot rater workflow, sidecar output
- Explicitly out of scope: cloud sync, plugin ecosystem, advanced color correction
- Delivery model: single-user first (no multi-user concurrency in MVP)

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
- MVP scope lock (Phase 1): [docs/MVP_SCOPE.md](./docs/MVP_SCOPE.md)
