# Testing Quick Start Guide

This guide covers the essentials for writing and running tests in Photorg.

## Quick Commands

```bash
# Run all frontend tests
npm run test

# Run frontend tests once
npm run test:run

# Run frontend tests with UI
npm run test:ui

# Generate frontend coverage report
npm run test:coverage

# Run all Rust tests
npm run test:rust

# Run all tests (Rust + Frontend)
npm run test:all
```

## Project Structure

```
photorg/
├── src/                          # Vue frontend code
│   ├── *.vue                     # Components
│   └── *.spec.ts                 # Component tests (vitest)
├── src-tauri/src/                # Rust backend code
│   ├── lib.rs                    # Main library (tests inline)
│   ├── domain.rs                 # Domain logic (tests inline)
│   └── tests/                    # Integration tests
├── test-data/                    # Test fixtures and data
│   ├── README.md                 # Fixture documentation
│   └── fixtures/                 # Fixture files
├── docs/
│   └── TESTING_STRATEGY.md       # Comprehensive testing strategy
└── .github/workflows/
    └── tests.yml                 # CI configuration
```

## Writing Your First Test

### Rust Unit Test

Add tests directly to the module file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function_returns_correct_value() {
        let result = my_function(5);
        assert_eq!(result, 10);
    }
}
```

Run it:
```bash
npm run test:rust
```

### Frontend Component Test

Create a `.spec.ts` or `.test.ts` file next to your component:

```typescript
import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import MyComponent from './MyComponent.vue';

describe('MyComponent', () => {
  it('renders the expected text', () => {
    const wrapper = mount(MyComponent, {
      props: { message: 'Hello' },
    });
    expect(wrapper.text()).toContain('Hello');
  });
});
```

Run it:
```bash
npm run test:run
```

## Test Data & Fixtures

Test data lives in `test-data/fixtures/`. For integration tests:

```rust
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_with_fixture() {
    let temp = TempDir::new().unwrap();
    let temp_path = temp.path();
    
    // Copy fixture or create test data
    // Run your test
    
    // Cleanup happens automatically
}
```

For frontend tests, mock Tauri API and file operations.

## Troubleshooting

### "Cannot find module X"
- Run `npm install` to ensure all dependencies are installed
- Check that import paths match the actual file location

### Rust tests fail to compile
- Ensure `[dev-dependencies]` are in `src-tauri/Cargo.toml`
- Check that `tempfile` and `mockito` are listed

### Frontend tests timeout
- Increase timeout: `it('name', async () => {...}, 10000)`
- Check for infinite loops or unresolved promises
- Use `beforeEach`/`afterEach` for cleanup

### "Cannot find function in tests"
- Ensure function is `pub` (public)
- Add `use super::*;` to bring parent scope into test module

## Coverage Requirements

- **Minimum (MVP)**: 70% for all code
- **Target (Post-MVP)**: 80%+ for domain logic

Check coverage:
```bash
npm run test:rust:coverage    # Rust: opens HTML report
npm run test:coverage          # Frontend: generates report
```

## Running Tests in CI

Tests run automatically on every push to `master` and `develop` branches, and on all pull requests. See `.github/workflows/tests.yml` for details.

To run the full CI suite locally:
```bash
npm run test:all
npm run test:all:coverage
```

## File Naming Conventions

- **Rust**: Tests in same file, suffixed with `_test` or in `tests/` directory
- **Frontend**: Test files named `*.spec.ts` or `*.test.ts` (next to component)
- **Fixtures**: Descriptive names in `test-data/fixtures/` subdirectories

## Best Practices

1. **Test behavior, not implementation** — if you refactor code without changing behavior, tests should still pass
2. **Keep tests focused** — one test = one assertion or small related group
3. **Use descriptive names** — `test_rating_increases_with_duplicate_selection` is better than `test_rating`
4. **Isolate tests** — use `tempfile` (Rust) and mock setup/teardown (frontend)
5. **Avoid test interdependence** — tests should run in any order
6. **Mock external dependencies** — don't test third-party libraries, test how you use them

## Adding Performance Tests (Future)

Performance tests are deferred for MVP but can be added with `criterion` (Rust):

```bash
cargo install cargo-criterion
```

See `docs/TESTING_STRATEGY.md` for performance test guidelines.

## Need Help?

- **Rust docs**: `rustup doc` (open Rust documentation locally)
- **vitest docs**: https://vitest.dev
- **Vue Test Utils**: https://test-utils.vuejs.org
- **Test Strategy**: See `docs/TESTING_STRATEGY.md` for comprehensive guidance

---

**Last Updated**: June 2026
