# Toolbar

Horizontal bar of commands for the view below it.

Status: **draft** on web.

## Geometry

| Property | Value |
| :--- | :--- |
| Height | At least `10 DU`; dense `8 DU`. |
| Padding | `1 DU` horizontal. |
| Gap | `1 DU`; dense `0.5 DU`. |
| Background | `surface` with a `0.25 DU` `border` line below. |
| Title | Medium weight, `1 DU` horizontal padding, followed by a separator. |
| Separator | `0.25 DU` × `6 DU` `divider` with `1 DU` margins. |
| Commands | [IconButton](icon-button.md), `8 DU`; dense `6 DU`. |
| Overflow button | Same size as a command, `more_vert` icon, right after the last visible command. |
| End controls | After the free space, at the end of the bar; fields are `8 DU` tall. |

## Layout

```
[ Title | ⟳  ⏷  ⤓  ⋮            ( Search…     ) ]
          commands  overflow      end controls
```

## Behavior

* Commands are data, not free-form children, so each one can appear either as a button in the bar or as a row in the overflow menu.
* When the commands do not fit, they move into the overflow [Menu](menu.md) from the end, one at a time, and come back when space returns.
  The overflow button appears only when at least one command is in the menu.
* In the menu a command shows its icon and label; a separated command starts a group in both places.
* Commands marked `overflow` always live in the menu, after the commands that did not fit, as a separate group.
  Use them for rare or destructive commands.
* The title and the end controls never move into the menu.
* Each command button has a tooltip with its label.
* An async command shows its states on its own button.
  Run from the menu, its states show on the overflow button.
* Left and Right arrows move focus between the visible controls, except inside text inputs.

## API

`Toolbar`:

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `items` | list of `ToolbarItem` | required | Commands in order. |
| `title` | string signal | none | Title at the start. |
| `start` | slot | none | Controls before the title at the start of the bar. |
| `end` | slot | none | Controls at the end that never overflow, such as a search field. |
| `label` | string | `toolbar.label` | Accessible name. |
| `dense` | bool | `false` | Smaller variant. |
| `height` | DU | `row_standard` | Bar height. |
| `dense_height` | DU | `row_compact` | Dense bar height. |
| `padding_x` | DU | `pad_tight` | Horizontal padding. |
| `gap` | DU | `pad_tight` | Items gap. |
| `dense_gap` | DU | `gap_micro` | Dense items gap. |
| `title_padding_x` | DU | `pad_tight` | Title horizontal padding. |
| `title_font_size` | TU | `inherit` | Title font size. |
| `separator_width` | DU | `border` | Separator line thickness. |
| `separator_height` | DU | `icon_large` | Separator line height. |
| `separator_margin_x` | DU | `pad_tight` | Separator horizontal margins. |
| `button_size` | DU | `row_compact` (dense: `row_dense`) | Command button size. |
| `more_label` | string | `toolbar.more` | Accessible name for the overflow button. |
| `more_icon` | icon | `toolbar.more` | Icon override for the overflow button. |
| `class` | string | none | Classes for the root element. |
| `style` | string | none | Inline styles for the root element. |

`ToolbarItem`:

| Builder | Description |
| :--- | :--- |
| `new(id, label, icon)` | Command; the label is the tooltip, the accessible name, and the menu text. |
| `on_click(callback)` | Runs the callback. |
| `on_async(action)` | Runs an async action with state feedback. |
| `separated()` | Starts a new group. |
| `overflow()` | Always in the overflow menu. |
| `danger()` | Destructive; error color in the menu. |
| `disabled()` | Not available. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-toolbar-height` | `row_standard` (`10 DU`) |
| `dense_height` | DU | `--op-toolbar-dense-height` | `row_compact` (`8 DU`) |
| `padding_x` | DU | `--op-toolbar-padding-x` | `pad_tight` (`1 DU`) |
| `gap` | DU | `--op-toolbar-gap` | `pad_tight` (`1 DU`) |
| `dense_gap` | DU | `--op-toolbar-dense-gap` | `gap_micro` (`0.5 DU`) |
| `title_padding_x` | DU | `--op-toolbar-title-padding-x` | `pad_tight` (`1 DU`) |
| `title_font_size` | TU | `--op-toolbar-title-font-size` | `inherit` |
| `separator_width` | DU | `--op-toolbar-separator-width` | `border` (`0.25 DU`) |
| `separator_height` | DU | `--op-toolbar-separator-height` | `icon_large` (`6 DU`) |
| `separator_margin_x` | DU | `--op-toolbar-separator-margin-x` | `pad_tight` (`1 DU`) |
| `button_size` | DU | `--op-toolbar-button-size` | `row_compact` (`8 DU`), dense `row_dense` (`6 DU`) |

| Color variable | Default |
| :--- | :--- |
| `--op-toolbar-background` | `surface` |
| `--op-toolbar-border` | `border` |
| `--op-toolbar-title-color` | `text_primary` |
| `--op-toolbar-separator-color` | `divider` |

| Slot | Place |
| :--- | :--- |
| `start` | At the start of the toolbar before the title. |
| `end` | At the end of the toolbar after free space. |

`class` and `style` apply to the root element.

### Strings

| Key | Default |
| :--- | :--- |
| `toolbar.label` | `Toolbar` |
| `toolbar.more` | `More commands` |

### Icons

| Key | Default |
| :--- | :--- |
| `toolbar.more` | `more_vert` |

## Accessibility

The bar has the `toolbar` role and the given name.
The overflow button is named "More commands" and opens a `menu`.
