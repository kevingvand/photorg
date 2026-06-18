# Photorg

Photorg is a desktop-first photo culling app focused on fast RAW+JPEG paired review, keyboard-driven rating/labeling, and goal-based best-shot selection.

## Current MVP Scope

- Paired RAW+JPEG preview as one logical unit
- Keyboard-first keep/reject/rating workflow
- Immutable source policy (no direct RAW/JPEG modification by default)
- Sidecar + internal database metadata model
- Early rater workflow for tournament-style selection

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
