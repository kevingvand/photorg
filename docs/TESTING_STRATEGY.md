# Testing Strategy

This document defines Photorg's comprehensive testing approach across all layers, covering unit tests, integration tests, E2E tests, and performance considerations.

## Overview

Photorg employs a multi-layered testing strategy to ensure reliability, maintainability, and performance across the Tauri desktop application. Testing is integrated from the start and covers both the Rust backend and Vue frontend.

## Test Categories

### 1. Unit Tests (Target: 80%+ Coverage)

**Scope**: Core domain logic operating in isolation.

**What to Test**:
- Rating and scoring algorithms
- Pair matching logic
- Conflict detection and resolution
- Metadata parsing and validation
- Data transformation functions

**Tools**:
- **Rust**: cargo test + assertions
- **Frontend**: vitest with Vue component/utility testing

**Guidelines**:
- Test behavior, not implementation details
- Use descriptive test names: `test_rating_increases_with_duplicate_selection()`
- Isolate domain logic from I/O and external dependencies
- Mock filesystem, database, and network operations

**Example (Rust)**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_validation_rejects_missing_raw() {
        let pair = ImagePair::new("test.jpg", None);
        assert!(!pair.is_valid());
    }
}
```

### 2. Integration Tests (Critical Path Coverage)

**Scope**: Adapter boundaries where domain logic meets I/O (filesystem, database, XMP metadata).

**What to Test**:
- Database operations (create, read, update, delete)
- Filesystem interactions (reading RAW/JPEG files, writing sidecars)
- XMP metadata reading and writing
- File import workflows
- Catalog initialization

**Tools**:
- **Rust**: cargo test with `tempfile` for isolation
- **Frontend**: vitest with mock Tauri API

**Guidelines**:
- Use temporary directories (`tempfile::TempDir`) for each test
- Reset database state between tests (transactions or snapshots)
- Test error paths (missing files, corrupted metadata, permission denied)
- Clean up all temporary files in test teardown

**Example (Rust)**:
```rust
#[cfg(test)]
mod integration_tests {
    use tempfile::TempDir;

    #[test]
    fn test_catalog_import_creates_database() {
        let temp = TempDir::new().unwrap();
        let catalog = Catalog::import(temp.path()).unwrap();
        assert!(catalog.db_path().exists());
    }
}
```

### 3. End-to-End (E2E) Tests (3–5 Key Workflows)

**Scope**: Complete user workflows from start to finish.

**Key Workflows**:
1. **Ingest**: Open catalog → scan directory → pair RAW+JPEG → verify database state
2. **Cull**: Rate images via keyboard shortcuts → apply conflict resolution → generate rater outcomes
3. **Export**: Filter by rating/goal → generate sidecar → export recommendations
4. **Metadata Round-trip**: Write XMP → close → reopen → verify metadata persists
5. **Error Recovery**: Handle corrupted files → user sees clear error → continues

**Tools**:
- **Frontend**: vitest with simulated Tauri API + mock filesystem
- May defer full automation if time is limited; manual E2E testing acceptable for MVP

**Guidelines**:
- Test happy path and one critical error path per workflow
- Verify UI state changes and command output
- Use consistent test data across workflows
- Document expected behavior for manual testing

### 4. Performance Tests (Optional for MVP)

**Scope**: Performance-critical operations under realistic load.

**What to Test**:
- Preview rendering for 100+ images
- Rater ranking algorithm with 1000+ images
- Database query performance under load
- Catalog initialization time

**Tools**:
- **Rust**: criterion crate for benchmarking
- **Frontend**: vitest with performance markers

**Status**: Optional for MVP; may be deferred to Phase 2 if time is constrained.

## Test Data & Fixtures

### Fixture Organization

```
test-data/
  fixtures/
    sample-catalog/    # 10–50 representative RAW+JPEG pairs
    edge-cases/        # Corrupted/unusual files
    db-states/         # Pre-populated DB snapshots
```

### Fixture Guidelines

- **Sample Catalog**: Realistic variety of image types and sizes; use smallest viable files to keep repo size manageable
- **Edge Cases**: Corrupted JPEGs, missing XMP, malformed metadata, permission issues
- **DB Snapshots**: Captured database states for deterministic testing
- **Isolation**: Tests copy fixtures to temporary directories; fixtures are read-only

### Test Data Cleanup

- All temporary files and directories are automatically cleaned up via `tempfile` crate (Rust) or test lifecycle hooks (frontend)
- Database transactions are rolled back or reset between tests
- No leftover files in project directory after test suite completes

## Coverage Requirements

### Minimum Coverage (MVP)
- **Unit tests**: 70% of domain logic
- **Integration tests**: Critical path coverage (100% of adapter boundaries)
- **E2E tests**: 3–5 key workflows

### Target Coverage (Post-MVP)
- **Unit tests**: 80%+ across all modules
- **Integration tests**: 90%+ adapter boundary coverage
- **E2E tests**: 10+ workflows covering edge cases

### Coverage Tools

- **Rust**: `cargo tarpaulin` for coverage reporting
  ```bash
  cargo tarpaulin --out html --exclude-files tests
  ```
- **Frontend**: `vitest` with v8 coverage provider
  ```bash
  npm run test:coverage
  ```

### CI Integration

- Coverage reports are generated on every push
- Minimum coverage thresholds are enforced (fail CI if below 70%)
- Coverage reports are uploaded to artifact storage or coverage service

## Running Tests

### Rust Backend

```bash
# Run all tests
cargo test --lib

# Run specific test
cargo test --lib test_pair_validation

# Run with output
cargo test --lib -- --nocapture

# Run integration tests
cargo test --test '*'

# Generate coverage report
cargo tarpaulin --out html
```

### Frontend

```bash
# Run all tests
npm run test:run

# Run tests in watch mode
npm run test

# Run with UI
npm run test:ui

# Generate coverage report
npm run test:coverage
```

## Adding a New Test

### Rust Unit Test

1. Locate the module being tested (e.g., `src-tauri/src/rater.rs`)
2. Add a `#[cfg(test)]` module at the bottom of the file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_feature() {
        // Arrange
        let input = vec![1, 2, 3];
        
        // Act
        let result = process(input);
        
        // Assert
        assert_eq!(result, vec![2, 4, 6]);
    }
}
```

3. Run: `cargo test --lib`

### Rust Integration Test

1. Create a file in `src-tauri/tests/` (e.g., `tests/catalog_import.rs`)
2. Write your test:

```rust
use tempfile::TempDir;

#[test]
fn test_catalog_workflow() {
    let temp = TempDir::new().unwrap();
    // Your test here
}
```

3. Run: `cargo test --test '*'`

### Frontend Component Test

1. Create a test file next to the component: `MyComponent.test.ts` (or `.spec.ts`)
2. Write your test:

```typescript
import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import MyComponent from './MyComponent.vue';

describe('MyComponent', () => {
  it('renders correctly', () => {
    const wrapper = mount(MyComponent);
    expect(wrapper.text()).toContain('Expected text');
  });
});
```

3. Run: `npm run test`

## Troubleshooting

### Common Issues

**Rust: "failed to resolve: use of undeclared type `Path`"**
- Add `use std::path::Path;` at the top of your test module

**Rust: "cannot find function `tempfile`"**
- Ensure `tempfile` is in `[dev-dependencies]` in `Cargo.toml`
- Run `cargo test --lib` to trigger dependency resolution

**Frontend: "jsdom is not installed"**
- Run `npm install` to ensure all dev dependencies are installed
- Check `vitest.config.ts` specifies `environment: 'jsdom'`

**Frontend: "Cannot find module '@vue/test-utils'"**
- Run `npm install` and verify `@vue/test-utils` is in `package.json`

**Tests hang or timeout**
- Check for infinite loops or unresolved promises in test code
- Increase test timeout: `it('test name', async () => {...}, 10000)` (10 seconds)
- Use `beforeEach` and `afterEach` hooks to clean up resources

**Database tests fail with "cannot open database"**
- Verify temp directory is created: `let temp = TempDir::new()?;`
- Check database schema is initialized in test setup
- Ensure no parallel tests write to the same database

### Getting Help

- Check test output for detailed error messages
- Review test isolation: ensure tests don't share mutable state
- Verify fixtures exist and are readable from test context
- Consult Rust docs: `rustup doc`; Frontend: `vitest.dev`, `vue-test-utils.vuejs.org`

## Test Lifecycle & Isolation

### Rust Tests

- **Setup**: Use `TempDir::new()` for isolated filesystem; initialize fresh database
- **Teardown**: Automatic via RAII; `TempDir` cleans up on drop
- **Isolation**: Each test runs in separate process (cargo test default)

### Frontend Tests

- **Setup**: Use vitest's `beforeEach` hook to mount components and reset mocks
- **Teardown**: Use vitest's `afterEach` hook to unmount and clean up
- **Isolation**: Test files are independent; component state is reset between tests

### Mocking

- **Filesystem**: Use `tempfile` crate for Rust; mock file I/O in frontend
- **Database**: Use in-memory or temp database for tests; roll back transactions
- **Tauri API**: Mock `@tauri-apps/api` calls in frontend tests
- **External Services**: Mock HTTP calls and network operations

## Future Considerations

- **Snapshot Testing**: Consider for UI regression detection in later phases
- **Mutation Testing**: Verify test quality by injecting code mutations
- **Property-Based Testing**: Use `proptest` crate for fuzzing domain logic
- **Load Testing**: Benchmark catalog operations with 10,000+ images
- **Accessibility Testing**: Add a11y checks for frontend components

---

**Last Updated**: June 2026  
**Version**: 1.0 (MVP)
