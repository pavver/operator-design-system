# Accordion

Sections with header rows that show or hide their content.
A standalone section is a Collapsible; an Accordion keeps only one section open.

Status: **reviewed** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Header | At least `10 DU`; `2 DU` start and `3 DU` end padding. |
| Chevron | `4 DU` `chevron_right` in `text_secondary`; rotated 90° and `accent` when open. |
| Title | Medium weight; optional subtitle in `caption` style. |
| Trailing | Optional content at the end, such as a [Counter](counter.md). |
| Content | Indented to the title; `1 DU` top and `3 DU` bottom padding. |
| Accordion | `0.25 DU` `border` outline, `divider` between sections. |

## Behavior

* A click on the header toggles the section.
* The content height animates over `expand` (`200 ms`) with the standard curve; the chevron rotates with it.
* Closed content stays mounted but is inert: it is not focusable and is hidden from assistive technology.
* In an Accordion, opening a section closes the open one.

## API

| Component | Properties |
| :--- | :--- |
| `Accordion` | `children`: sections. |
| `Collapsible` | `title`, `subtitle`, `open` (external signal), `initially_open`, `trailing`, `children`. |

### Customization API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `header_height` | optional number (DU) | `row_standard` | Overrides `--op-accordion-header-height`. |
| `padding_start` | optional number (DU) | `gutter` | Overrides `--op-accordion-padding-start`. |
| `padding_end` | optional number (DU) | `pad_card` | Overrides `--op-accordion-padding-end`. |
| `padding_y` | optional number (DU) | `pad_tight` | Overrides `--op-accordion-padding-y`. |
| `gap` | optional number (DU) | `gutter` | Overrides `--op-accordion-gap`. |
| `icon_size` | optional number (DU) | `icon_small` (`4 DU`) | Overrides `--op-accordion-icon-size`. |
| `content_padding_top` | optional number (DU) | `pad_tight` | Overrides `--op-accordion-content-padding-top`. |
| `content_padding_end` | optional number (DU) | `pad_card` | Overrides `--op-accordion-content-padding-end`. |
| `content_padding_bottom` | optional number (DU) | `pad_card` | Overrides `--op-accordion-content-padding-bottom`. |
| `content_padding_start` | optional number (DU) | `padding_start + gap + icon_size` (defaults to `2 × gutter + icon_small`) | Overrides `--op-accordion-content-padding-start`. |
| `font_size` | optional number (TU) | `inherit` | Overrides `--op-accordion-font-size`. |
| `line_height` | optional number (TU) | `inherit` | Overrides `--op-accordion-line-height`. |
| `subtitle_font_size` | optional number (TU) | `type_caption_size` | Overrides `--op-accordion-subtitle-font-size`. |
| `subtitle_line_height` | optional number (TU) | `type_caption_line` | Overrides `--op-accordion-subtitle-line-height`. |
| `header` | slot | none | Accordion: before sections; Collapsible: replaces the title and subtitle inside the toggle button. |
| `actions` | slot | none | Accordion: before sections; Collapsible: outside the toggle button, before the panel. |
| `footer` | slot | none | Accordion: after sections; Collapsible: after content inside the animated panel. |
| `trailing` | slot | none | Collapsible only; trailing content inside the toggle button. |
| `expand_label` | optional string | `accordion.expand` | Text override. |
| `collapse_label` | optional string | `accordion.collapse` | Text override. |
| `chevron_icon` | optional icon | `accordion.chevron` | Icon override. |
| `class` | optional string | none | Added to the root classes. |
| `style` | optional string | none | Appended after root metric declarations. |

## Accessibility

The header is a button with `aria-expanded` and `aria-controls` pointing at the content region.

## Customization

`class` and `style` apply to the root.
Explicit root styles take precedence over metric props; unset metrics retain the original rendering.
Metric and root hooks are available on both Accordion and standalone Collapsible; group variables inherit into sections.
`expand_label`, `collapse_label` and `chevron_icon` belong to Collapsible.
The action description follows the open state without replacing the title as the accessible name.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `header_height` | DU | `--op-accordion-header-height` | `row_standard` |
| `padding_start` | DU | `--op-accordion-padding-start` | `gutter` |
| `padding_end` | DU | `--op-accordion-padding-end` | `pad_card` |
| `padding_y` | DU | `--op-accordion-padding-y` | `pad_tight` |
| `gap` | DU | `--op-accordion-gap` | `gutter` |
| `icon_size` | DU | `--op-accordion-icon-size` | `icon_small` (`4 DU`) |
| `content_padding_top` | DU | `--op-accordion-content-padding-top` | `pad_tight` |
| `content_padding_end` | DU | `--op-accordion-content-padding-end` | `pad_card` |
| `content_padding_bottom` | DU | `--op-accordion-content-padding-bottom` | `pad_card` |
| `content_padding_start` | DU | `--op-accordion-content-padding-start` | `padding_start + gap + icon_size` (defaults to `2 × gutter + icon_small`) |
| `font_size` | TU | `--op-accordion-font-size` | `inherit` |
| `line_height` | TU | `--op-accordion-line-height` | `inherit` |
| `subtitle_font_size` | TU | `--op-accordion-subtitle-font-size` | `type_caption_size` |
| `subtitle_line_height` | TU | `--op-accordion-subtitle-line-height` | `type_caption_line` |

Border widths remain the fixed geometry specified above.

| Color variable | Default |
| :--- | :--- |
| `--op-accordion-border` | `border` |
| `--op-accordion-divider` | `divider` |
| `--op-accordion-text` | `text_primary` |
| `--op-accordion-hover` | `hover` |
| `--op-accordion-secondary-text` | `text_secondary` |
| `--op-accordion-accent` | `accent` |

| Slot | Place |
| :--- | :--- |
| `header` | Accordion: before sections; Collapsible: replaces the title and subtitle inside the toggle button. |
| `actions` | Accordion: before sections; Collapsible: outside the toggle button, before the panel. |
| `footer` | Accordion: after sections; Collapsible: after content inside the animated panel. |
| `trailing` | Collapsible only; trailing content inside the toggle button. |
| `children` | Sections in Accordion; panel content in Collapsible. |

### Strings

Resolution: prop, application lookup, English default.

| Key | Default |
| :--- | :--- |
| `accordion.expand` | `Expand` |
| `accordion.collapse` | `Collapse` |

### Icons

| Key | Default |
| :--- | :--- |
| `accordion.chevron` | `chevron_right` |
