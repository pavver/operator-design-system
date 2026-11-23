# ActionButton

Rectangular button that runs an async action and shows its outcome.

Status: **reviewed** on web.

## Anatomy

* **Container:** owns the state border and the result background.
* **Label:** optional icon followed by text.
* **Status content:** spinner or result icon, drawn in place of the label.
* **Ghost label:** an invisible copy of the label that keeps the width stable while status content is shown.

## Geometry

| Property | Value |
| :--- | :--- |
| Height | `row_standard` (`10 DU`). |
| Width | Content plus `margin_screen` (`4 DU`) on each side. |
| Corner radius | `0 DU`. |
| Icon | `icon_small` (`4 DU`), `gutter` (`2 DU`) gap to the text. |
| Text | `body` size, weight 500, centered on every line. |

## States

| State | Background | Content | Outline |
| :--- | :--- | :--- | :--- |
| `Idle` | `surface` | `text_primary` | `border` |
| `Pressed` | `pressed` | `text_primary` | `border_strong` |
| `Pending` | `surface` | Label, then the [delayed spinner](../foundations/states.md#delayed-spinner) | Running segment, unless disabled by `show_pending_border` |
| `Success` | `success` at 15% | `success` | Pulsing `success` |
| `Error` | `error` at 15% | `error` | Pulsing `error` |
| `Disabled` | `disabled_surface` | `text_disabled` | `disabled_border` |

A **dangerous** button uses `error` content on `error` at 15% when idle, and `error` at 30% when pressed.

## Behavior

* Inside a [ButtonGroup](button-group.md), the button also follows the group's `enabled` and, with `exclusive`, is disabled while another button of the group runs.
1. A click starts the action and enters `Pending`; clicks are ignored while pending.
2. On completion the button enters the result state for `800 ms`.
   For the first `500 ms` only the result icon is shown.
   For the remaining `300 ms` the label returns in the result color.
3. The result then fades out over `500 ms`.

With `keep_flash_icon_on_success`, a success keeps the check icon for the whole `800 ms`.
A dangerous button ignores clicks during its result phase.

## Link Mode

Setting `href` turns the button into a navigation link with the same look.
Use it when the press opens a page or an external resource rather than running an action.

* Only `Idle`, `Pressed`, and `Disabled` apply.
  `state`, `on_click`, `show_pending_border`, and `keep_flash_icon_on_success` are ignored.
* `new_tab` opens the target in a new tab or window where the platform has that concept.
* A disabled link keeps its look but does not navigate.

### Platform Notes

| Platform | Implementation |
| :--- | :--- |
| Web | A real `<a href>` element, so search engine crawlers follow it and middle-click, modifier clicks, and the link context menu work. `new_tab` adds `target="_blank"` and `rel="noopener noreferrer"`. A disabled link drops `href` and sets `aria-disabled`. |
| Native toolkits | A regular button whose press opens the target through the platform URL handler, for example `open_url` in egui. |

## API

| Property | Type | Default |
| :--- | :--- | :--- |
| `text` | string | `action-button.text` |
| `icon` | icon | none |
| `state` | `ControlState?` | none |
| `enabled` | bool | `true` |
| `show_pending_border` | bool | `true` |
| `keep_flash_icon_on_success` | bool | `false` |
| `dangerous` | bool | `false` |
| `on_click` | async `() -> ActionResult` | none |
| `href` | string | none |
| `new_tab` | bool | `false` |
| `height` | reactive DU | unset |
| `padding_x` | reactive DU | unset |
| `gap` | reactive DU | unset |
| `icon_size` | reactive DU | unset |
| `font_size` | reactive TU | unset |
| `line_height` | reactive TU | unset |
| `spinner_stroke` | reactive DU | unset |
| `class` | reactive string | none |
| `style` | reactive string | none |
| `leading` | slot | none |
| `trailing` | slot | none |
| `content` | slot | none |
| `loading_label` | reactive string | `action-button.loading` |
| `success_label` | reactive string | `action-button.success` |
| `error_label` | reactive string | `action-button.error` |
| `success_icon` | icon | `action-button.success` |
| `error_icon` | icon | `action-button.error` |

## Customization

`class` and `style` apply to the root element.
Metric props accept reactive DU or TU values; unset props keep the CSS variable or its exact design-system fallback.
Caller `style` is appended after metric declarations and takes precedence.
ActionButton and IconButton share `--op-button-*` variables for metrics and colors with the same meaning.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `height` | DU | `--op-button-height` | `var(--op-dim-row-standard)` |
| `padding_x` | DU | `--op-button-padding-x` | `var(--op-dim-margin-screen)` |
| `gap` | DU | `--op-button-gap` | `var(--op-dim-gutter)` |
| `icon_size` | DU | `--op-button-icon-size` | `var(--op-dim-icon-small)` |
| `font_size` | TU | `--op-button-font-size` | `var(--op-type-body-size)` |
| `line_height` | TU | `--op-button-line-height` | `var(--op-type-body-line)` |
| `spinner_stroke` | DU | `--op-button-spinner-stroke` | `var(--op-dim-spinner-stroke)` |
| Color variable | Default |
| :--- | :--- |
| `--op-button-accent` | `var(--op-accent)` |
| `--op-button-border` | `var(--op-border)` |
| `--op-button-emphasis-border` | `var(--op-border-strong)` |
| `--op-button-disabled-border` | `var(--op-disabled-border)` |
| `--op-button-disabled-background` | `var(--op-disabled-surface)` |
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
| `--op-button-disabled-text` | `var(--op-text-disabled)` |
| `--op-button-text` | `var(--op-text-primary)` |
| Slot | Place |
| :--- | :--- |
| `leading` | Before the default content. |
| `trailing` | After the default content. |
| `content` | Replaces the default text or icon; pending and result feedback keep priority. |

### Strings

| Prop | Key | Default |
| :--- | :--- | :--- |
| `loading_label` | `action-button.loading` | `Loading` |
| `success_label` | `action-button.success` | `Success` |
| `error_label` | `action-button.error` | `Error` |
| `text` | `action-button.text` | empty string |

### Icons

| Prop | Key | Default |
| :--- | :--- | :--- |
| `success_icon` | `action-button.success` | `check` |
| `error_icon` | `action-button.error` | `close` |

Icons supplied as item data or through the existing `icon` prop remain caller-owned.
