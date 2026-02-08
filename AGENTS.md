# AGENTS.md

## Purpose
This repository contains [Advent of Code 2023](https://adventofcode.com/2023)
solutions in Rust, organized as a
Cargo workspace with one crate per day.

## Workspace Layout
- `Cargo.toml`: workspace definition and shared dependencies.
- `day_01/`, `day_02/`: per-day crates with `src/` modules.
- Inputs are embedded as `const DATA: &str` in `input.rs` using `indoc!`.

## Tooling Snapshot
- Rust edition: 2024 (workspace-wide).
- Test frameworks: `googletest` and `rstest`.
- No custom `rustfmt.toml` or `clippy.toml` detected.
- No Cursor or Copilot rules found (`.cursor/rules/`, `.cursorrules`,
  `.github/copilot-instructions.md`).

## Build / Run / Test / Lint

### Build
- Build the whole workspace:
  - `cargo build`
- Build a single day crate:
  - `cargo build -p day_01`
  - `cargo build -p day_02`

### Run
- Run a single day:
  - `cargo run -p day_01`
  - `cargo run -p day_02`

### Tests (all)
- Run all tests in workspace:
  - `cargo test`
- Run all tests in one crate:
  - `cargo test -p day_01`
  - `cargo test -p day_02`

### Tests (single test)
Use the test name or module path as a filter argument.
- By name within a crate:
  - `cargo test -p day_01 test_input`
  - `cargo test -p day_02 test_game_parser`
- By module path:
  - `cargo test -p day_01 solver::tests::test_input`
  - `cargo test -p day_02 game::tests::test_minimal_possible_draw`
- Run a specific rstest case (filter by the function name):
  - `cargo test -p day_01 tests::test`

### Formatting
- Format the workspace:
  - `cargo fmt`

### Linting (optional)
- Run clippy across all targets:
  - `cargo clippy --workspace --all-targets --all-features`

## Code Style Guidelines

### Imports
- Prefer `use` at the top of the file, grouped by crate/module.
- Use absolute paths within the crate (`crate::...`) for internal modules.
- For sibling modules, expose via `mod` and import from `crate::`.

### Module Structure
- Each day crate is a binary with `main.rs` and supporting modules.
- Keep parsing logic in dedicated modules (e.g., `game/draw/parser.rs`).
- Keep I/O isolated: runtime data is embedded in `input.rs`.

### Naming
- Types: `UpperCamelCase` (e.g., `Game`, `Draw`).
- Functions and variables: `snake_case`.
- Constants: `SCREAMING_SNAKE_CASE` for public constants (e.g., `MAX_DRAW`).
- Tests: use descriptive `test_*` function names.

### Types
- Favor explicit numeric types (`u32`) for puzzle outputs and counts.
- Use type aliases when helpful for clarity (e.g., `type Draws = Vec<Draw>`).
- Return tuples for multi-part results `(part1, part2)`.

### Error Handling
- Parsing currently uses `expect(...)` with clear messages.
- Prefer `expect`/`panic` only when input is trusted (AoC inputs) and failure
  indicates a logic error.
- If adding fallible APIs, consider returning `Result` instead of panicking.

### Formatting
- Use `cargo fmt`-compatible formatting (default rustfmt style).
- Keep lines readable; break method chains across lines as seen in `solve`.
- Align struct literals in tests for readability when fields are numerous.

### Functional Style
- Favor iterator pipelines for transformations (`map`, `filter`, `sum`).
- Keep helpers small and focused (`solve_line`, `replace_digit_line`).

### Regex Usage
- Compile regexes with `Regex::new` and `expect` on validity.
- Use named capture groups when it clarifies intent.

### Tests
- Use `googletest` for assertions (`expect_that!`, `eq`).
- Use `rstest` for parameterized cases.
- Keep test data inline or in small helper fixtures.

## Conventions Observed in This Repo
- `main` prints a single line with both results.
- Each day exposes a `solve` function returning `(u32, u32)`.
- Inputs are embedded and not loaded from files at runtime.
- Parsing functions are `pub` only when used across modules.

## Adding a New Day
- Create a new crate `day_XX/` and add it to `Cargo.toml` workspace members.
- Follow the existing structure:
  - `src/main.rs`, `src/solver.rs`, `src/input.rs`, and helper modules.
- Keep the public API minimal: `solve` and data access only.

## Notes for Agentic Changes
- Do not modify or delete the embedded AoC inputs unless explicitly requested.
- Keep behavior deterministic; avoid I/O, randomness, or time-based logic.
- Keep tests fast and runnable via `cargo test -p day_XX`.
