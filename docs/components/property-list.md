# PropertyList

List of label and value pairs, such as device details or connection settings.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Row | At least `10 DU`, `1 DU` vertical padding, `0.25 DU` `divider` below. |
| Columns | Label 40%, value 60%, `2 DU` apart. |
| Label | `text_secondary`. |
| Value | `text_primary`; long values wrap and break anywhere. |
| Copy button | `6 DU` [IconButton](icon-button.md) with `content_copy`, at the end of the value. |

When the list is narrower than `60 DU`, the label moves above the value in `caption` style.

## Behavior

* The copy button writes the value to the clipboard as an async action, so it shows the success check or the error pulse.
* A value can be any view, for example a [StatusBadge](status-badge.md).

## API

| Component | Properties |
| :--- | :--- |
| `PropertyList` | `children`: properties. |
| `Property` | `label`, `value` (string) or `children` (view), `copyable` (`false`). |

### Customization API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `row_height` | optional number (DU) | `row_standard` | Overrides `--op-property-list-row-height`. |
| `padding_y` | optional number (DU) | `pad_tight` | Overrides `--op-property-list-padding-y`. |
| `gap` | optional number (DU) | `gutter` | Overrides `--op-property-list-gap`. |
| `value_gap` | optional number (DU) | `pad_tight` | Overrides `--op-property-list-value-gap`. |
| `copy_size` | optional number (DU) | `row_dense` | Overrides `--op-property-list-copy-size`. |
| `icon_size` | optional number (DU) | `3.5 DU` | Overrides `--op-property-list-icon-size`. |
| `stack_width` | optional number (DU) | `60 DU` | Overrides `--op-property-list-stack-width`. |
| `font_size` | optional number (TU) | `inherit` | Overrides `--op-property-list-font-size`. |
| `line_height` | optional number (TU) | `inherit` | Overrides `--op-property-list-line-height`. |
| `label_font_size` | optional number (TU) | `type_caption_size` | Overrides `--op-property-list-label-font-size`. |
| `label_line_height` | optional number (TU) | `type_caption_line` | Overrides `--op-property-list-label-line-height`. |
| `option` | scoped slot | none | Scoped `PropertyValue { label, value }`; replaces a Property row's terms and descriptions; return `dt` and `dd` elements. |
| `value_slot` | scoped slot | none | Scoped `PropertyValue`; replaces value content inside `dd`; copying still uses the original value signal. |
| `empty` | slot | none | Used when PropertyList has no children; the default remains blank. |
| `header` | slot | none | Above the properties. |
| `footer` | slot | none | Below the properties. |
| `before_options` | slot | none | Before the properties. |
| `after_options` | slot | none | After the properties. |
| `copy_label` | optional string | `property-list.copy` | Text override. |
| `copy_icon` | optional icon | `property-list.copy` | Icon override. |
| `class` | optional string | none | Added to the root classes. |
| `style` | optional string | none | Appended after root metric declarations. |

## Accessibility

A description list: labels are terms, values are descriptions.

## Customization

`class` and `style` apply to the root.
Explicit root styles take precedence over metric props; unset metrics retain the original rendering.
The row metric and root hooks are available on both PropertyList and Property.
The `stack_width` metric belongs to PropertyList.
The list supplies inherited `option` and `value_slot` callbacks; a Property can override `value_slot`.
`copy_label` and `copy_icon` belong to Property.
`stack_width` controls the list breakpoint; set it on PropertyList.
A private ResizeObserver compares the measured list width with the DU breakpoint and preserves the original narrow layout.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `row_height` | DU | `--op-property-list-row-height` | `row_standard` |
| `padding_y` | DU | `--op-property-list-padding-y` | `pad_tight` |
| `gap` | DU | `--op-property-list-gap` | `gutter` |
| `value_gap` | DU | `--op-property-list-value-gap` | `pad_tight` |
| `copy_size` | DU | `--op-property-list-copy-size` | `row_dense` |
| `icon_size` | DU | `--op-property-list-icon-size` | `3.5 DU` |
| `stack_width` | DU | `--op-property-list-stack-width` | `60 DU` |
| `font_size` | TU | `--op-property-list-font-size` | `inherit` |
| `line_height` | TU | `--op-property-list-line-height` | `inherit` |
| `label_font_size` | TU | `--op-property-list-label-font-size` | `type_caption_size` |
| `label_line_height` | TU | `--op-property-list-label-line-height` | `type_caption_line` |

Border widths remain the fixed geometry specified above.

| Color variable | Default |
| :--- | :--- |
| `--op-property-list-divider` | `divider` |
| `--op-property-list-label` | `text_secondary` |
| `--op-property-list-text` | `text_primary` |

| Slot | Place |
| :--- | :--- |
| `option` | Scoped `PropertyValue { label, value }`; replaces a Property row's terms and descriptions; return `dt` and `dd` elements. |
| `value_slot` | Scoped `PropertyValue`; replaces value content inside `dd`; copying still uses the original value signal. |
| `empty` | Used when PropertyList has no children; the default remains blank. |
| `header` | Above the properties. |
| `footer` | Below the properties. |
| `before_options` | Before the properties. |
| `after_options` | After the properties. |

### Strings

Resolution: prop, application lookup, English default.

| Key | Default |
| :--- | :--- |
| `property-list.copy` | `Copy value` |

The copy button retains IconButton's `icon-button.loading`, `icon-button.success` and `icon-button.error` feedback keys.

### Icons

| Key | Default |
| :--- | :--- |
| `property-list.copy` | `content_copy` |

The copy button retains IconButton's `icon-button.success` and `icon-button.error` icon keys.
