# Units

Operator UI defines two platform-independent units.
Specifications and portable implementations use only these units.
Raw `dp`, `px`, `sp`, `pt`, and `rem` values are prohibited.

## Design Units (DU)

`1 DU = 4 logical points`.
DU measures every spatial value: sizes, paddings, gaps, borders, icon sizes, and offsets.
DU never scales with the user's text size setting.
Values are multiples or simple fractions of one DU, such as `0.25 DU`, `0.5 DU`, `1.5 DU`, or `10 DU`.

| Platform | Mapping |
| :--- | :--- |
| Web (CSS) | `--op-du: 4px`; a value is `calc(var(--op-du) * N)`. |
| Jetpack Compose | `1 DU = 4.dp`. |
| egui | `1 DU = 4.0` points. |

## Type Units (TU)

`1 TU = 1 scalable logical point`.
TU measures font sizes and line heights only.
TU follows the user's text scale setting.

| Platform | Mapping | Scale source |
| :--- | :--- | :--- |
| Web (CSS) | `--op-tu: 0.0625rem`. | Browser root font size. |
| Jetpack Compose | `1 TU = 1.sp`. | System font scale. |
| egui | `1 TU = 1.0 * text_scale` points. | Application setting. |

## Spatial Tokens

Prefer a named token over a literal.
The values live in [tokens.toml](../../tokens/tokens.toml).

| Token | Value | Typical use |
| :--- | :--- | :--- |
| `border` | `0.25 DU` | Standard outline. |
| `border_thick` | `0.5 DU` | Emphasis outline, focus ring. |
| `spinner_stroke` | `0.375 DU` | Stroke of small spinners. |
| `gap_micro` | `0.5 DU` | Inner padding of compact containers. |
| `pad_tight` | `1 DU` | Tight gaps, tick dots. |
| `pad_small` | `1.5 DU` | Icon-to-text gap in small controls. |
| `gutter` | `2 DU` | Gap between related elements. |
| `pad_content` | `2.5 DU` | Content padding. |
| `pad_card` | `3 DU` | Card and field inner padding. |
| `margin_screen` | `4 DU` | Screen margin, button side padding. |
| `section_gap` | `6 DU` | Gap between sections. |
| `icon_small` | `4 DU` | Icons inside controls. |
| `icon_medium` | `4.5 DU` | Toolbar icons. |
| `icon_large` | `6 DU` | Standalone icons, slider thumb. |
| `row_dense` | `6 DU` | Log and console rows. |
| `row_compact` | `8 DU` | Compact rows, slider height. |
| `row_standard` | `10 DU` | Buttons, fields, standard rows. |
| `row_control` | `12 DU` | Rows holding a control with a label. |
| `row_two_line` | `13 DU` | Rows with a title and a subtitle. |
| `row_avatar` | `14 DU` | Rows with an avatar. |

Row heights are minimum heights.
A row grows when its content needs more space.

## Clamp Model

A container with dynamic content never has a fixed width.
Its width is `clamp(MinDU, PreferredRatio * ViewportWidth, MaxDU)`.
Each component that uses the clamp model lists its three parameters.
