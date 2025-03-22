# Testing the Obadh Engine

This document explains the testing process for the Obadh Engine.

## Testing Tools

The project includes several tools to make testing more efficient:

### 1. Integration Tests

Standard Rust integration tests in the `tests/` directory. Run them with:

```bash
cargo test
```

### 2. Efficient Test Runner

A dedicated test runner for fast testing with detailed output:

```bash
# Run all test cases with detailed output
cargo run --bin test_cases

# Run only a specific category
cargo run --bin test_cases basic

# Run a specific test case
cargo run --bin test_cases basic bhalo

# Get JSON output (useful for debugging)
cargo run --bin test_cases --json
```

### 3. Test Update Utility

When you make changes to the engine, you can automatically update test cases to match the new output:

```bash
cargo run --bin update_tests
```

This will update all test files in the `tests/` directory with the actual output from the current engine.

## Test Categories

Tests are organized into categories:

1. **basic** - Basic transliteration cases
2. **yaphala** - Tests for ya-phala (jofola) handling
3. **complex** - Complex conjuncts tests
4. **bofola** - Tests for bo-fola handling

## Testing the Engine Programmatically

For more advanced testing, you can use the JSON output feature:

```rust
let engine = ObadhEngine::new();
let json_output = engine.transliterate_json("bhalo");
println!("{}", json_output);
```

This will give you a structured JSON output with input, output, and token information.

## Debugging Failed Tests

When a test fails, the test runner will show the expected output versus the actual output. You can:

1. Fix the engine to produce the expected output
2. Update the test to expect the current output (if the current output is correct)

Use the `update_tests` binary to batch update all tests to match the current engine behavior. 