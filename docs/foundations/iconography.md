# Iconography

## Source

Icons come from [Material Symbols](https://fonts.google.com/icons?icon.style=Sharp) in the **Sharp** style.
Sharp corners match the `0 DU` geometry of the rest of the system.
Material Symbols are licensed under the Apache License 2.0.

An icon is drawn from scratch only when no Material symbol fits the meaning.
Custom icons follow the rules in [Custom Icons](#custom-icons).

## Variants

| Axis | Value | Notes |
| :--- | :--- | :--- |
| Style | Sharp | Always. |
| Fill | Outlined | Filled only where the specification asks for it, for example to mark an active state. |
| Weight | 400 | The only weight in use. |
| Optical size | 20 | For icons up to `5 DU`; use 24 for `icon_large` and above. |

## Sizes

| Token | Size | Use |
| :--- | :--- | :--- |
| `icon_small` | `4 DU` | Icons inside controls: buttons, fields, segments. |
| `icon_medium` | `4.5 DU` | Toolbar and header actions. |
| `icon_large` | `6 DU` | Standalone icons. |

Status icons inside small round elements, such as the slider thumb, use `3.5 DU`.

## Color

Icons are single-color and take the current content color.
They follow the same state colors and transitions as text, see [states.md](states.md).
Icons never carry their own palette.

## Meaning

* An icon on a toggle shows the action it performs, not the current state.
  The password reveal toggle shows `visibility` while the text is hidden.
* `check` and `close` are reserved for success and error feedback and for confirm and cancel actions.
* An icon-only control always has an accessible name.

## Adding an Icon

1. Find the symbol at <https://fonts.google.com/icons?icon.style=Sharp>.
2. Add an entry to [tokens/icons.toml](../../tokens/icons.toml), for example `wifi = { material = "wifi" }`.
3. Run `python3 tools/import_material_symbols.py`, then `cargo run -p operator-tokens`.

The importer keeps only the `viewBox` and path data of the official SVG.
CI fails when an icon file differs from its Material source or is missing from the manifest.

## Custom Icons

* Declare the icon in the manifest with `custom = true`.
* Use a `0 0 24 24` viewBox with filled paths only; no strokes, no transforms.
* Keep a `2` unit margin inside the viewBox and a stroke-like line width of `2` units.
* Use sharp corners and 45° or 90° angles where possible.
* Match the visual weight of the neighbouring Material Sharp icons at `4 DU`.
