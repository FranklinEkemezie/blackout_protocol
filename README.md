# Blackout Protocol

Terminal-based tactical simulation written in Rust.

## Project layout

- `src/main.rs` — game logic and terminal UI
- `tests/spec.rs` — deterministic regression tests
- `SPECS.md` — project specification

## Run

```bash
cargo run
```

## Test

```bash
cargo test
```

## Notes

- Standard library only.
- Mission state is tracked with tuples and constants.
- The code is organized for incremental exercises and spec-based checks.
