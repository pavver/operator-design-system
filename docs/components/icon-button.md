# IconButton

Square button that shows only an icon and supports synchronous and async clicks.

Status: **reviewed** on web.

## Use

* Toolbar, header, and inline row actions.
* Compact actions where a label would not fit.

## Geometry

| Property | Value |
| :--- | :--- |
| Size | Square, `row_compact` (`8 DU`) by default. Common sizes are `7`, `8`, `9`, and `11 DU`. |
| Icon | `icon_small` (`4 DU`) by default, centered. |
| Corner radius | `0 DU`. |
| Outline | State border, `0.25 DU`. |

## States

| State | Background | Icon |
| :--- | :--- | :--- |
| `Idle` | `surface` | `icon` |
| `Pressed` | `pressed` | `icon` |
| `Selected` | `selection` | `icon` |
| `Pending` | `surface` | Running border; icon, then the [delayed spinner](../foundations/states.md#delayed-spinner) at the icon size |
| `Success` | `success` at 15% | `check` in `success` |
| `Error` | `error` at 15% | `close` in `error` |
| `Disabled` | `disabled_surface` | `icon_disabled` |

## Behavior

* A synchronous click runs its callback immediately and has no async states.
* An async click enters `Pending` and ignores further clicks until the result arrives.
* The result icon is shown for `500 ms`, then the original icon returns in the result color for `300 ms`.
* The result then fades out over `500 ms`.
* `loading = true` forces `Pending` from outside, for example while a parent loads data.

## Link Mode

Setting `href` turns the button into a navigation link with the same look.
Use it when the press opens a page or an external resource rather than running an action.

* Only `Idle`, `Pressed`, and `Disabled` apply.
  `state`, `loading`, `on_click`, and `on_async_click` are ignored.
* `new_tab` opens the target in a new tab or window where the platform has that concept.
* A disabled link keeps its look but does not navigate.

### Platform Notes

| Platform | Implementation |
| :--- | :--- |
| Web | A real `<a href>` element, so search engine crawlers follow it and middle-click, modifier clicks, and the link context menu work. `new_tab` adds `target="_blank"` and `rel="noopener noreferrer"`. A disabled link drops `href` and sets `aria-disabled`. |
| Native toolkits | A regular button whose press opens the target through the platform URL handler, for example `open_url` in egui. |

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `icon` | icon | required | The icon. |
| `label` | string | `icon-button.label` | Accessible name, also shown as a library [Tooltip](tooltip.md) unless the button is already inside one. Required in practice. |
| `size` | DU | `8` | Button size. |
| `icon_size` | DU | `4` | Icon size. |
| `state` | `ControlState?` | none | External state override. |
| `enabled` | bool | `true` | Interactivity. |
| `loading` | bool | `false` | Forces `Pending`. |
| `on_click` | callback | none | Synchronous action. |
| `on_async_click` | async `() -> ActionResult` | none | Async action; takes precedence over `on_click`. |
| `href` | string | none | Navigation target; enables link mode. |
| `new_tab` | bool | `false` | Opens the link in a new tab (link mode only). |
| `gap` | reactive DU | unset | Overrides `--op-button-gap`; see Customization. |
| `spinner_stroke` | reactive DU | unset | Overrides `--op-button-spinner-stroke`; see Customization. |
| `class` | reactive string | none | Applied to the root element. |
| `style` | reactive string | none | Applied to the root element. |
| `leading` | slot | none | Before the default content. |
| `trailing` | slot | none | After the default content. |
| `content` | slot | none | Replaces the default text or icon; pending and result feedback keep priority. |
| `loading_label` | reactive string | `icon-button.loading` | String override; see Strings. |
| `success_label` | reactive string | `icon-button.success` | String override; see Strings. |
| `error_label` | reactive string | `icon-button.error` | String override; see Strings. |
| `success_icon` | icon | `icon-button.success` | Semantic icon override; see Icons. |
| `error_icon` | icon | `icon-button.error` | Semantic icon override; see Icons. |

## Customization

`class` and `style` apply to the root element.
Metric props accept reactive DU or TU values; unset props keep the CSS variable or its exact design-system fallback.
Caller `style` is appended after metric declarations and takes precedence.
ActionButton and IconButton share `--op-button-*` variables for metrics and colors with the same meaning.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-button-size` | `var(--op-dim-row-compact)` |
| `icon_size` | DU | `--op-button-icon-size` | `var(--op-dim-icon-small)` |
| `gap` | DU | `--op-button-gap` | `var(--op-dim-gutter)` |
| `spinner_stroke` | DU | `--op-button-spinner-stroke` | `var(--op-dim-spinner-stroke)` |

The existing numeric size props keep their names and DU meaning and now also accept reactive values.

| Color variable | Default |
| :--- | :--- |
| `--op-button-accent` | `var(--op-accent)` |
| `--op-button-border` | `var(--op-border)` |
| `--op-button-emphasis-border` | `var(--op-border-strong)` |
| `--op-button-disabled-border` | `var(--op-disabled-border)` |
| `--op-button-error` | `var(--op-error)` |
| `--op-button-hover` | `var(--op-hover)` |
| `--op-button-icon` | `var(--op-icon)` |
| `--op-button-disabled-icon` | `var(--op-icon-disabled)` |
| `--op-button-pending` | `var(--op-pending)` |
| `--op-button-pressed` | `var(--op-pressed)` |
| `--op-button-ripple` | `var(--op-ripple)` |
| `--op-button-selection` | `var(--op-selection)` |
| `--op-button-success` | `var(--op-success)` |
| `--op-button-background` | `var(--op-surface)` |
| Slot | Place |
| :--- | :--- |
| `leading` | Before the default content. |
| `trailing` | After the default content. |
| `content` | Replaces the default text or icon; pending and result feedback keep priority. |

### Strings

| Prop | Key | Default |
| :--- | :--- | :--- |
| `loading_label` | `icon-button.loading` | `Loading` |
| `success_label` | `icon-button.success` | `Success` |
| `error_label` | `icon-button.error` | `Error` |
| `label` | `icon-button.label` | `Action` |

### Icons

| Prop | Key | Default |
| :--- | :--- | :--- |
| `success_icon` | `icon-button.success` | `check` |
| `error_icon` | `icon-button.error` | `close` |

Icons supplied as item data or through the existing `icon` prop remain caller-owned.
