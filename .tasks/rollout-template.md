Task: bring the listed Operator UI components to the customization model.

Read first, and follow:

* `AGENTS.md` (repository rules; note: no `.unwrap()`/`.expect()`, no raw px/rem, DU/TU only, every Markdown sentence on its own line, English code and docs).
* `docs/foundations/customization.md` (the model you implement).
* The reference implementation: `web/operator-ui/src/core/custom.rs` (`Metric`, `root_class`, `root_style`, `ui_text`, `ui_icon`, `Slot`, `ScopedSlot`), `web/operator-ui/src/components/field/` (Field shell) and `web/operator-ui/src/components/search_field/` with its spec `docs/components/search-field.md` (sections API, Customization, Strings, Icons).
  Copy these patterns exactly.

Do NOT edit: `web/operator-ui/src/core/**`, `web/operator-ui/src/components/field/**`, `web/operator-ui/src/components/search_field/**`, `web/operator-ui-demo/src/main.rs`, `web/operator-ui-demo/src/i18n.rs`, `web/operator-ui-demo/build.rs`, or any component not in your list (other agents work on them in parallel).
If you need a new helper in `core`, write it privately inside your component module and mention it in the summary.

## For every component in your list

1. **Root hooks.** Add `#[prop(optional, into)] class: MaybeProp<String>` and `#[prop(optional, into)] style: MaybeProp<String>`, applied to the root element with `root_class("op-xxx", class)` and `root_style(vec![...metrics], style)`.
   If the component already has a `class: String` prop, change its type to `MaybeProp<String>` (callers passing a string still compile).
2. **Metric props.** Every spacing and size in the component's CSS (heights, widths, paddings, gaps, icon sizes, font sizes, line heights, border widths if the spec allows variants) becomes:
   * a CSS variable read with the current value as fallback: `padding: var(--op-<family>-padding-x, var(--op-dim-pad-card))`;
   * a prop `#[prop(optional, into)] padding_x: MaybeProp<f32>` (DU, or TU for font sizes) passed to `root_style` as `Metric::du("--op-<family>-padding-x", padding_x)` / `Metric::tu(...)`.
   Family names: use the component name in kebab case (`--op-action-button-*`, `--op-menu-*`), except shared families: all fields use `--op-field-*` (build them on `Field`), `action-button` and `icon-button` share `--op-button-*` where the metric means the same thing.
   Existing numeric props that already set a size (for example `width`, `height`, `size` in DU) stay with the same name and meaning.
   Keep the rendered default look pixel-identical: the fallback of each variable is exactly the value used today.
3. **Color variables.** Every color the component draws reads `var(--op-<family>-<role>, var(--op-<token>))` (for example `--op-menu-background`).
   No new color values; no color props.
4. **Slots.** Add the slots of the component's family from `docs/foundations/customization.md` §3 where they make sense for this component, as `#[prop(optional, into)] name: Slot` (render with `name.run()` / `name.is_set()`), and scoped slots as `#[prop(optional)] option: Option<ScopedSlot<T>>`.
   Existing `ViewFn`/`Children`/`Option<ViewFn>` props stay compatible (prefer converting `Option<ViewFn>` props to `Slot`, it accepts the same arguments).
   Fields (text-like inputs) must be rebuilt on the public `Field` shell like SearchField, which gives them `label`, `hint`, `error`, `variant`, `prepend`, `append`, `before`, `after`, `label_slot`, `hint_slot` and the `--op-field-*` metrics.
5. **Strings.** Every visible or accessible English text inside the component (labels, `aria-label`s, placeholders, "Close", "Loading", month and weekday names, units) goes through `ui_text(prop, "<component>.<key>", "<English default>")`.
   If the component had a string prop with an English default, change it to `#[prop(optional, into)] x: MaybeProp<String>` resolved with `ui_text` (callers passing a string or a `Signal<String>` still compile).
6. **Icons.** Every built-in icon goes through `ui_icon(prop, "<component>.<key>", icons::X)` with an optional `Option<operator_ui_tokens::Icon>` prop to override it.
7. **Spec.** Update `docs/components/<name>.md`: the API table lists every new prop; add a `## Customization` section (metric props table with unit, variable and default; color variables table; slots table; a line that `class` and `style` apply to the root) and `### Strings` and `### Icons` tables, in the same format as `docs/components/search-field.md`.
   Do not change the `Status:` line.
8. **Change history.** Add one file per commit per component at `docs/changes/<name>/<YYYY-MM-DD>-<HHMM>-<short-kebab-slug>.toml`, using the local date and time of the entry:
   ```toml
   date = "<current time, RFC 3339 with offset>"
   status = "draft"
   kind = "feature"

   [en]
   summary = "Customization hooks: `class`/`style`, metric props (...), color variables, slots (...), translatable strings and replaceable icons."
   check = "Open the Customization example and inspect the named props and slots."

   [uk]
   summary = "Точки кастомізації: `class`/`style`, пропи розмірів (...), змінні кольорів, слоти (...), перекладні тексти й замінні іконки."
   check = "Відкрийте приклад Customization і перевірте зазначені пропи та слоти."
   ```
   Name the concrete props and slots in both languages; write natural Ukrainian.
   Use `kind` from `new`, `feature`, `fix`, `breaking`, `docs`, or `refactor`.
   Every `breaking` change requires `migration` in both language tables explaining what callers must change.
   Optional `commit` accepts 7–40 hexadecimal characters; otherwise the build may derive the adding commit from complete Git history.
   During review the maintainer approves the file, replaces it with the final description, or deletes it if the change was dropped.
   Update the component's Ukrainian specification in `docs/i18n/uk/components/` with its English specification.
   See [the history workflow](../docs/releasing.md#component-change-history) and [review queue](../docs/changes-to-review.md).
9. **Demo.** In the component's own demo module `web/operator-ui-demo/src/pages/<snake_name>.rs`, add one example "Customization" that shows a few metric props, a slot, and a translated string.
   Keep existing examples working.
10. Keep every Rust file at most 500 lines (split into submodules inside the component folder if needed).

## Verify (all must pass, from the repository root)

* `cargo fmt --all --check`
* `cargo clippy -q -p operator-ui --target wasm32-unknown-unknown --features full,extensions -- -D warnings`
* `cargo clippy -q -p operator-ui-demo --target wasm32-unknown-unknown -- -D warnings`
* For each component feature you touched: `cargo clippy -q -p operator-ui --target wasm32-unknown-unknown --no-default-features --features <feature> -- -D warnings`
* `cargo run -q -p operator-tokens -- --check`
* `cd web/operator-ui-demo && trunk build`

Do not commit.
At the end print a short summary per component: new props, variables, slots, string keys, icon keys, and anything you could not do.
