# Blackout Protocol

Terminal-based tactical simulation written in Rust.

## Project layout

- `src/main.rs` — game logic and terminal UI
- `tests/spec.rs` — deterministic regression tests
- `docs/SPECS.md` — project specification
- `docs/BUGS.md` — known specification deviations

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
- Mission state is tracked with structs and an enum-backed system status.
- The code is organized for incremental exercises and spec-based checks.
