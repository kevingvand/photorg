# Photorg Architecture

## System Context (C4 - Level 1)

```mermaid
flowchart LR
  photographer[Photographer]
  photorg[Photorg Desktop App]
  filesystem[Local File System]
  sidecars[XMP Sidecar Files]
  catalog[(Local Catalog DB)]

  photographer --> photorg
  photorg --> filesystem
  photorg --> sidecars
  photorg --> catalog
```

## Container View (C4 - Level 2)

```mermaid
flowchart LR
  ui[Vue UI]
  core[Domain Core]
  tauri[Tauri Backend]
  fs[File System Adapter]
  xmp[XMP Sidecar Adapter]
  db[(Catalog Database)]

  ui --> core
  core --> tauri
  tauri --> fs
  tauri --> xmp
  tauri --> db
```

## Component Notes

- **Domain Core** owns pairing, culling state, rater logic, and conflict policy.
- **XMP Sidecar Adapter** writes interoperable outcomes (`xmp:Rating`, `xmp:Label`) only.
- **Catalog Database** stores operational internals (pairwise comparisons, confidence, session history).
- **File System Adapter** enforces immutable-source policy by default.
