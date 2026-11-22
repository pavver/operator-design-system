Task: add new Operator UI components (inspired by the Quasar framework's component set) to the design system, as Draft.

Read first, and follow:

* `AGENTS.md` (repository rules: new component = spec in `docs/components/`, own folder in `web/operator-ui/src/components/`, own feature in `web/operator-ui/Cargo.toml` listing the features it uses and joining the `draft` group, CSS registered in `web/operator-ui/src/core/theme.rs`, re-export in the `prelude` of `web/operator-ui/src/lib.rs`, module in `web/operator-ui/src/components/mod.rs`; no `.unwrap()`/`.expect()`; no raw px/rem, DU/TU and tokens only; corners `0 DU`; every Markdown sentence on its own line; English code and docs).
* `docs/foundations/customization.md` and `docs/foundations/states.md`: every new component has `class`/`style`, metric props backed by `--op-<family>-*` variables, color variables, family slots, `ui_text` strings and `ui_icon` icons from the start.
* Reference: `web/operator-ui/src/core/custom.rs`, `web/operator-ui/src/components/field/`, `web/operator-ui/src/components/search_field/`, `docs/components/search-field.md`, `docs/components/field.md`.
* Quasar's source is available read-only at [Quasar components](../../../testapp/quasar/ui/src/components/) (`Q*.js` behaviour, `Q*.json` API with props, slots and events).
  Use it to decide which props and slots make sense, but write the component in the Operator style (dense, square, DU/TU, Operator states and motion tokens, Material Symbols Sharp icons from `operator_ui_tokens::icons`; if an icon is missing, reuse the closest existing one and mention it in the summary — do not add icons).
  Do not copy Quasar code, CSS or texts.
* Reuse existing building blocks instead of re-implementing them: `Popover` (anchored overlays), `VirtualList`, `Field`, `ActionButton`, `IconButton`, `Menu`, `Slider`, `ProgressIndicator`, `Icon`, `use_state_visual`, `AsyncAction`, `JobToken`, `spawn_ripple`.

Do NOT edit other components, `web/operator-ui/src/core/custom.rs`, `web/operator-ui-demo/build.rs`, or `web/operator-ui-demo/src/i18n.rs`.
Shared registration files (`Cargo.toml`, `theme.rs`, `lib.rs`, `components/mod.rs`, demo `pages/mod.rs`, demo `main.rs` sidebar `NAV`) may only get new lines for your components, kept in alphabetical order where the file is alphabetical, so parallel branches merge cleanly.

## For every component in your list

1. Implementation in `web/operator-ui/src/components/<snake_name>/mod.rs` + `<snake_name>.css` (split into submodules if a file would exceed 500 lines).
2. Spec `docs/components/<kebab-name>.md` with `Status: **draft** on web.` and the sections used by the other specs: Use (when to use it), Geometry, Behavior, API, Customization (metric props, color variables, slots, `class`/`style`), Strings, Icons, Accessibility.
3. Change history: add one `docs/changes/<kebab-name>/<YYYY-MM-DD>-<HHMM>-<short-kebab-slug>.toml` file per commit per component.
   Start with `status = "draft"`, an RFC 3339 `date` with offset, and `kind = "new"` for the initial component.
   Both `[en]` and `[uk]` tables require a Markdown `summary` describing the component in natural English and Ukrainian.
   Add bilingual `check` texts naming the examples, states, and props a tester should inspect.
   Subsequent changes use the appropriate `kind`: `new`, `feature`, `fix`, `breaking`, `docs`, or `refactor`.
   Every `breaking` change requires `migration` in both language tables explaining what users must change.
   An optional `commit` accepts 7–40 hexadecimal characters; otherwise the build may derive the adding commit from complete Git history.
   During review the maintainer approves the file, replaces it with the final description, or deletes it if the change was dropped.
   Also add the Ukrainian specification at `docs/i18n/uk/components/<kebab-name>.md`.
   See [the history workflow](../docs/releasing.md#component-change-history).
4. Demo module `web/operator-ui-demo/src/pages/<snake_name>.rs` with live examples (default, states, customization), registered in `pages/mod.rs`, and a sidebar entry in the right `NAV` group of `web/operator-ui-demo/src/main.rs` (or wherever the sidebar groups are defined now).
5. Keyboard and accessibility: correct roles, labels through `ui_text`, focus handling.

## Verify (all must pass, from the repository root)

* `cargo fmt --all --check`
* `cargo clippy -q -p operator-ui --target wasm32-unknown-unknown --features full,extensions -- -D warnings`
* For each new feature: `cargo clippy -q -p operator-ui --target wasm32-unknown-unknown --no-default-features --features <feature> -- -D warnings`
* `cargo clippy -q -p operator-ui-demo --target wasm32-unknown-unknown -- -D warnings`
* `cargo run -q -p operator-tokens -- --check`
* `cd web/operator-ui-demo && trunk build`

Do not commit.
At the end print a short summary per component and anything you could not do.
