# Banner

Persistent message inside a screen or section about its current state.
It stays until the condition is gone or the user dismisses it.

Status: **reviewed** on web.

## Use

* A state that affects the whole view: offline mode, full storage, a pending firmware update.
* Prefer a [Toast](toast.md) for events and a [Dialog](dialog.md) for decisions that block work.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Full width of its container. |
| Padding | `3 DU`. |
| Background | Tone color at 15% over `surface`. |
| Start bar | `0.5 DU` in the tone color. |
| Icon | `icon_xlarge` (`8 DU`), tone color, `2 DU` from the text; the first text line is centered on it. |
| Title | Body style, bold. |
| Actions | Under the text, `8 DU` tall [ActionButtons](action-button.md), `2 DU` apart. |
| Action colors | Transparent background, tone-colored label and icon, outline in the tone at 50%; hover adds a 12% tone tint and a full tone outline, press 25%. Danger buttons keep their own colors. |
| Close | `8 DU` square button at the top end. |

The tones and their icons are the same as in [Toast](toast.md).

## Behavior

* The banner does not disappear on its own.
* With a dismiss handler it shows a close button; the caller removes the banner.
* Text wraps; long words break.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `tone` | tone | `Info` | Info, Success, Warning, or Error. |
| `title` | string | none | Optional bold first line. |
| `text` | string | required | Message. |
| `actions` | `Slot` | none | Buttons under the text. |
| `on_dismiss` | callback | none | Shows the close button. |

The customization properties are optional and accept reactive values where indicated.

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `class`, `style` | reactive string | none | Root hooks. |
| `gap` | reactive DU metric | design-system value | Sets `--op-banner-gap`. |
| `padding` | reactive DU metric | design-system value | Sets `--op-banner-padding`. |
| `body_gap` | reactive DU metric | design-system value | Sets `--op-banner-body-gap`. |
| `icon_size` | reactive DU metric | design-system value | Sets `--op-banner-icon-size`. |
| `line_height` | reactive TU metric | design-system value | Sets `--op-banner-line-height`. |
| `button_size` | reactive DU metric | design-system value | Sets `--op-banner-button-size`. |
| `close_outset` | reactive DU metric | design-system value | Sets `--op-banner-close-outset`. |
| `font_size` | reactive TU metric | design-system value | Sets `--op-banner-font-size`. |
| `close_icon_size` | reactive DU metric | design-system value | Sets `--op-banner-close-icon-size`. |
| `width` | reactive DU metric | design-system value | Sets `--op-banner-width`. |
| `header` | `Slot` | none | Replaces the title content. |
| `footer` | `Slot` | none | Content after the body and actions. |
| `dismiss_label` | reactive string | `banner.dismiss` | Dismiss button accessible name. |
| `icon` | icon | `tone icon key` | Tone icon override. |
| `dismiss_icon` | icon | `banner.dismiss` | Dismiss icon override. |

## Customization

`class` and `style` apply to the banner container.
Metric props write the corresponding variables; an explicit `style` declaration wins.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `gap` | DU | `--op-banner-gap` | `gutter` |
| `padding` | DU | `--op-banner-padding` | `pad_card` |
| `body_gap` | DU | `--op-banner-body-gap` | `pad_tight` |
| `icon_size` | DU | `--op-banner-icon-size` | `icon_xlarge` (`8 DU`) |
| `line_height` | TU | `--op-banner-line-height` | `body_line` |
| `button_size` | DU | `--op-banner-button-size` | `row_compact` |
| `close_outset` | DU | `--op-banner-close-outset` | `pad_small` |
| `font_size` | TU | `--op-banner-font-size` | `body_size` |
| `close_icon_size` | DU | `--op-banner-close-icon-size` | `icon_small` (`4 DU`) |
| `width` | DU | `--op-banner-width` | `100%` |

| Color variable | Default |
| :--- | :--- |
| `--op-banner-tone` | `var(--tone)` |
| `--op-banner-surface` | `surface` |
| `--op-banner-text` | `text_primary` |
| `--op-banner-background` | `color-mix(in srgb, var(--op-banner-tone, var(--tone)) 15%, var(--op-banner-surface, surface))` |
| `--op-banner-secondary` | `text_secondary` |
| `--op-banner-hover` | `hover` |
| `--op-banner-ripple` | `ripple` |

| Slot | Place |
| :--- | :--- |
| `header` | Replaces the title content. |
| `actions` | Actions after the body. |
| `footer` | Content after the body and actions. |

### Strings

| Key | Default | Prop |
| :--- | :--- | :--- |
| `banner.dismiss` | `Dismiss` | `dismiss_label` |

### Icons

| Key | Default | Prop |
| :--- | :--- | :--- |
| `banner.info` | `info` | `icon` |
| `banner.success` | `check_circle` | `icon` |
| `banner.warning` | `warning` | `icon` |
| `banner.error` | `error` | `icon` |
| `banner.dismiss` | `close` | `dismiss_icon` |

## Accessibility

Warning and error banners have the `alert` role; info and success have the `status` role.
