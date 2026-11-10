# Agent Instructions

Rules for AI agents contributing to this repository.

## Scope

* The repository holds the design system, its tokens, and its platform implementations.
* Application-specific or third-party branded assets do not belong here.
* A component is added only after it has been reviewed in every state.
* Each change to a component updates its implementation and its specification in the same commit.

## Language and Formatting

* All files are written in English.
* In Markdown files, every sentence is on its own line.
* Files are referenced with relative paths.

## Units

* Spatial values use Design Units (`1 DU = 4 logical points`) and the tokens in [tokens/tokens.toml](tokens/tokens.toml).
* Font sizes and line heights use Type Units (`1 TU = 1 scalable logical point`).
* Raw `px`, `dp`, `sp`, `pt`, and `rem` values are prohibited in specifications and component code.
  The only exceptions are the unit definitions themselves and runtime pointer geometry.

## Geometry

* Corner radius is `0 DU` unless the component specification lists a round exception.
* Standard border `0.25 DU`, emphasis border `0.5 DU`, always drawn inside the bounds.

## Tokens

* [tokens/tokens.toml](tokens/tokens.toml) is the only place where token values are defined.
* Never edit generated files; run `cargo run -p operator-tokens` instead.
* Every palette must keep the same key set.

## Behavior

* Follow [docs/foundations/states.md](docs/foundations/states.md) for every interactive component.
* Pending blocks input, results do not, errors roll back with the same animation as the forward change.

## Rust

* No `.unwrap()` or `.expect()`.
* Run `cargo fmt` and `cargo clippy` (host crates, and the web crates with `--target wasm32-unknown-unknown`) before committing; the code must be warning-free.
* `cargo run -p operator-tokens -- --check` must pass.

## Git

* Conventional Commits in English.
* No automated or AI signatures in commit messages.
