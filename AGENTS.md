# Agent Instructions

Rules for AI agents contributing to this repository.

## Scope

* The repository holds the design system, its tokens, and its platform implementations.
* Application-specific or third-party branded assets do not belong here.
* A new component starts as Draft and becomes Reviewed only after the maintainer has checked it in every state.
* Each change to a component updates its implementation and its specification in the same commit.
* The documentation site renders `docs/**/*.md` directly, so a specification is also the human-facing page.
* A new component needs a Markdown file in `docs/components/`, live examples in `web/operator-ui-demo/src/pages/`, and a sidebar entry in `web/operator-ui-demo/src/main.rs`.
* A new component gets its own folder in `web/operator-ui/src/components/` and its own feature in [web/operator-ui/Cargo.toml](web/operator-ui/Cargo.toml).
* Its feature lists the component features it uses.
* It joins `draft` and moves to `reviewed` when its spec status changes.

## Customization

* Every component follows [docs/foundations/customization.md](docs/foundations/customization.md): `class` and `style` on the root, every spacing and size as a DU/TU metric prop backed by a `--op-<family>-<metric>` variable, colors through `--op-<family>-<role>` variables, family slot names, string keys with English defaults (`ui_text`), semantic icon keys (`ui_icon`).
* A component never hard-codes visible or accessible text.
* Its specification lists the hooks in a **Customization** section (metric props, color variables, slots, strings, icons).

## Change History and Translations

* Each commit that changes a component adds one file in `docs/changes/<component>/<YYYY-MM-DD>-<HHMM>-<short-kebab-slug>.toml`, where `<component>` matches its specification filename.
* A new change file starts with `status = "draft"` in the same commit as the component change.
* Each file requires `date` (RFC 3339 with a UTC offset), `status` (`draft` or `approved`), and `kind` (`new`, `feature`, `fix`, `breaking`, `docs`, or `refactor`).
* `commit` is optional and accepts 7–40 hexadecimal characters; with complete Git history, the documentation build may derive the commit that added the file only if that commit also touched the component implementation directory or its specification.
  Grouped pages use an obvious mapping to the implementation they document, such as `value-fields` to `value_field`; otherwise no implementation mapping is inferred.
* Both `[en]` and `[uk]` require a Markdown `summary`, may include `check` describing the examples, states, or props a tester should inspect, and require `migration` explaining user actions when `kind = "breaking"`.
* Supply Ukrainian text for every supplied English summary, check, and migration instruction.
* During review, the maintainer approves a file by setting `status = "approved"`, replaces it to describe the final revision, or deletes it if the change was dropped.
* The documentation site lists all draft files on [Changes to review](docs/changes-to-review.md), newest first.
* Ukrainian versions of the documentation live in `docs/i18n/uk/` with the same relative paths; a change to an English page updates its Ukrainian page in the same commit.

## Language and Formatting

* All files are written in English, except the Ukrainian pages in `docs/i18n/uk/` and the `uk` texts of change entries.
* In Markdown files, every sentence is on its own line.
* Files are referenced with relative paths.

## Units

* Spatial values use Design Units (`1 DU = 4 logical points`) and the tokens in [tokens/tokens.toml](tokens/tokens.toml).
* Font sizes and line heights use Type Units (`1 TU = 1 scalable logical point`).
* Raw `px`, `dp`, `sp`, `pt`, and `rem` values are prohibited in specifications and component code.
  The only exceptions are the unit definitions themselves and geometry measured at runtime (pointer positions, element sizes).

## Geometry

* Corner radius is `0 DU` unless the component specification lists a round exception.
* Standard border `0.25 DU`, emphasis border `0.5 DU`, always drawn inside the bounds.

## Tokens

* [tokens/tokens.toml](tokens/tokens.toml) is the only place where token values are defined.
* Never edit generated files; run `cargo run -p operator-tokens` instead.
* Every palette must keep the same key set.

## Icons

* Use Material Symbols Sharp through [tokens/icons.toml](tokens/icons.toml) and `tools/import_material_symbols.py`.
* Draw a custom icon only when no Material symbol fits, and follow [docs/foundations/iconography.md](docs/foundations/iconography.md).

## Behavior

* Follow [docs/foundations/states.md](docs/foundations/states.md) for every interactive component.
* Pending blocks input, results do not, errors roll back with the same animation as the forward change.

## Rust

* No `.unwrap()` or `.expect()`.
* Run `cargo fmt` and `cargo clippy` (host crates, and the web crates with `--target wasm32-unknown-unknown`) before committing; the code must be warning-free.
* `scripts/check-features.sh` must pass before committing.
* `cargo run -p operator-tokens -- --check` must pass.

## Git

* Conventional Commits in English.
* No automated or AI signatures in commit messages.
