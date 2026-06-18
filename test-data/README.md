# Test Data Fixtures

This directory contains test fixtures and sample data for Photorg unit, integration, and E2E tests.

## Directory Structure

```
test-data/
  fixtures/          # Test fixture files
    sample-catalog/  # Sample RAW+JPEG pairs
    edge-cases/      # Corrupted or unusual files for error handling
    db-states/       # Pre-populated database snapshots
```

## Guidelines

- **Sample Catalog**: Contains 10-50 representative RAW/JPEG pairs (DNGs or CRs2 + JPEGs) for integration testing
- **Edge Cases**: Includes corrupted files, malformed metadata, missing sidecars to test error handling
- **DB States**: Snapshots of database states for testing transitions and queries
- **Isolation**: Tests use `tempfile` crate or Vue test utilities to copy fixtures to temp directories — never modify fixtures in place
- **Cleanup**: All temporary files and directories are cleaned up automatically after tests via test lifecycle hooks

## Adding New Fixtures

When adding test data:
1. Create appropriately named subdirectories in `fixtures/`
2. Document the fixture's purpose and expected behaviors
3. Ensure the fixture works in a temporary copy context
4. Update this README with fixture descriptions

## Notes

- Fixture files should be small enough to not bloat the repo (use compressed archives if needed)
- Use `.gitignore` patterns to exclude large binaries if committing RAW files
- For unit tests, prefer synthetic data over fixture files where possible
