# Tokens and Code Generation

[tokens/tokens.toml](../tokens/tokens.toml) is the single source of truth for units, font, spatial tokens, type styles, motion, and color palettes.
[tokens/icons](../tokens/icons) holds the icons as plain SVG files.

Components are written by hand once per platform.
Only tokens are generated, because component behavior depends on each toolkit's event and animation model.

## Running the Generator

From the repository root:

```bash
cargo run -p operator-tokens            # write outputs
cargo run -p operator-tokens -- --check # fail when outputs are stale
```

| Output | Consumer |
| :--- | :--- |
| [web/operator-ui/style/tokens.css](../web/operator-ui/style/tokens.css) | Any web stack through CSS custom properties. |
| [crates/operator-ui-tokens/src/lib.rs](../crates/operator-ui-tokens/src/lib.rs) | Rust toolkits (Leptos, egui); `no_std`. |
| [generated/kotlin/OperatorTokens.kt](../generated/kotlin/OperatorTokens.kt) | Jetpack Compose. |

Generated files must never be edited by hand.
CI runs the `--check` mode.

## File Format

| Table | Unit | Content |
| :--- | :--- | :--- |
| `[units]` | points | `du_points` and `tu_points`. |
| `[font]` | — | Family and fallback list. |
| `[dimensions]` | DU | Spatial tokens. |
| `[typography.<style>]` | TU | `size`, `line`, `weight`. |
| `[motion]` | ms | Durations. |
| `[themes.<theme>.<mode>]` | `#RRGGBB` or `#RRGGBBAA` | One palette per theme and mode. |

Every palette must define the same keys.
The generator rejects a palette with a missing or extra key.

## Naming in Outputs

| Kind | CSS | Rust | Kotlin |
| :--- | :--- | :--- | :--- |
| Dimension | `--op-dim-row-standard` | `dim::ROW_STANDARD` | `OperatorDu.RowStandard` |
| Type style | `--op-type-body-size` | `typography::BODY` | `OperatorType.Body` |
| Motion | `--op-motion-state-color` | `motion::STATE_COLOR_MS` | `OperatorMotion.StateColor` |
| Color | `--op-accent` | `Palette::accent` | `OperatorPalette.accent` |

## Icons

An icon is an SVG file with a `viewBox` and one or more `<path d="...">` elements.
Paths are filled with the current content color.
The generator embeds every icon in the Rust output as `icons::<NAME>`.
