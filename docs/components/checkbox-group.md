# CheckboxGroup

Vertical list of full-width selectable tiles; each tile saves its own selection asynchronously.

Status: **reviewed** on web.

## Use

* Multiple choice from a short list: notification channels, enabled zones, assigned groups.

## Geometry

| Property | Value |
| :--- | :--- |
| Width | Fills the parent. |
| Tile height | `row_control` (`12 DU`). |
| Tile padding | `gutter` (`2 DU`) horizontally. |
| Gap between tiles | `0.75 DU`. |
| Icon | Optional, before the name with a `gutter` (`2 DU`) gap; size set by `icon_size`. |
| Indicator box | `icon_large` (`6 DU`) square at the right edge, check and spinner `icon_small` (`4 DU`). |
| Corner radius | `0 DU`. |

### Icon Sizes

| Size | Value | Tile leading padding |
| :--- | :--- | :--- |
| `Small` | `icon_small` (`4 DU`) | `gutter` (`2 DU`) |
| `Medium` | `icon_large` (`6 DU`) | `gutter` (`2 DU`) |
| `Large` | Tile height minus `pad_tight` on each side (`10 DU`) | `pad_tight` (`1 DU`), so the icon sits at an equal inset on three sides |

## States

| Tile | Background | Text | Outline | Indicator |
| :--- | :--- | :--- | :--- | :--- |
| Unselected | `surface` | `text_primary`, 500; icon in `icon` | `border` | Empty box with `border` outline |
| Selected | `accent` at 15% | `accent`, 700; icon in `accent` | `accent` at 1.5x | `accent` box with `accent` check |
| Pressed | `pressed` | — | `border_strong` | — |
| `Pending` | — | — | Running segment | the [delayed spinner](../foundations/states.md#delayed-spinner) |
| `Success` / `Error` | — | — | Pulsing result outline | Check in the result color |

## Behavior

* Each tile runs its own request; several tiles may be pending at the same time.
* A pending tile ignores clicks; other tiles stay interactive.
* Result feedback does not block input, and superseded result timers cannot clear a newer request.
* The parent updates the selected list on success; the tile only shows the result.
* A result is held for `800 ms` and then fades out over `500 ms`.

## API

| Property | Type | Description |
| :--- | :--- | :--- |
| `items` | list of `GroupItem { id, name, icon? }` | The options; `icon` is an optional leading icon. |
| `selected` | list of ids | The confirmed selection. |
| `on_change` | async `((id, bool)) -> ActionResult` | Called with the item id and the requested value. |
| `icon_size` | `Small`, `Medium`, `Large` | Size of the item icons; `Small` by default. |
| `gap` | reactive DU | Overrides `--op-checkbox-group-gap`; see Customization; default: unset. |
| `height` | reactive DU | Overrides `--op-checkbox-group-height`; see Customization; default: unset. |
| `padding_x` | reactive DU | Overrides `--op-checkbox-group-padding-x`; see Customization; default: unset. |
| `item_gap` | reactive DU | Overrides `--op-checkbox-group-item-gap`; see Customization; default: unset. |
| `large_icon_inset` | reactive DU | Overrides `--op-checkbox-group-large-icon-inset`; see Customization; default: unset. |
| `box_size` | reactive DU | Overrides `--op-checkbox-group-box-size`; see Customization; default: unset. |
| `mark_size` | reactive DU | Overrides `--op-checkbox-group-mark-size`; see Customization; default: unset. |
| `item_icon_size` | reactive DU | Overrides `--op-checkbox-group-item-icon-size`; see Customization; default: unset. |
| `font_size` | reactive TU | Overrides `--op-checkbox-group-font-size`; see Customization; default: unset. |
| `line_height` | reactive TU | Overrides `--op-checkbox-group-line-height`; see Customization; default: unset. |
| `spinner_stroke` | reactive DU | Overrides `--op-checkbox-group-spinner-stroke`; see Customization; default: unset. |
| `width` | reactive DU | Overrides `--op-checkbox-group-width`; see Customization; default: unset. |
| `class` | reactive string | Applied to the root element; default: none. |
| `style` | reactive string | Applied to the root element; default: none. |
| `option` | scoped slot | Replaces an option label; receives the item; default: none. |
| `before_options` | slot | Before the options; default: none. |
| `after_options` | slot | After the options; default: none. |
| `loading_label` | reactive string | String override; see Strings; default: `checkbox-group.loading`. |
| `label` | reactive string | String override; see Strings; default: `checkbox-group.label`. |
| `checked_icon` | icon | Semantic icon override; see Icons; default: `checkbox-group.checked`. |

## Customization

`class` and `style` apply to the root element.
Metric props accept reactive DU or TU values; unset props keep the CSS variable or its exact design-system fallback.
Caller `style` is appended after metric declarations and takes precedence.

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `gap` | DU | `--op-checkbox-group-gap` | `calc(var(--op-du) * 0.75)` |
| `height` | DU | `--op-checkbox-group-height` | `var(--op-dim-row-control)` |
| `padding_x` | DU | `--op-checkbox-group-padding-x` | `var(--op-dim-gutter)` |
| `item_gap` | DU | `--op-checkbox-group-item-gap` | `var(--op-dim-gutter)` |
| `large_icon_inset` | DU | `--op-checkbox-group-large-icon-inset` | `var(--op-dim-pad-tight)` |
| `box_size` | DU | `--op-checkbox-group-box-size` | `var(--op-dim-icon-large)` |
| `mark_size` | DU | `--op-checkbox-group-mark-size` | `var(--op-dim-icon-small)` |
| `item_icon_size` | DU | `--op-checkbox-group-item-icon-size` | `var(--op-dim-icon-small)` |
| `font_size` | TU | `--op-checkbox-group-font-size` | `var(--op-type-body-size)` |
| `line_height` | TU | `--op-checkbox-group-line-height` | `var(--op-type-body-line)` |
| `spinner_stroke` | DU | `--op-checkbox-group-spinner-stroke` | `var(--op-dim-spinner-stroke)` |
| `width` | DU | `--op-checkbox-group-width` | `100%` |

`item_icon_size` overrides the existing `icon_size` enum; when unset, Small uses `icon_small`, Medium uses `icon_large`, and Large uses the resolved `height` minus two `large_icon_inset` values.

| Color variable | Default |
| :--- | :--- |
| `--op-checkbox-group-accent` | `var(--op-accent)` |
| `--op-checkbox-group-border` | `var(--op-border)` |
| `--op-checkbox-group-emphasis-border` | `var(--op-border-strong)` |
| `--op-checkbox-group-disabled-border` | `var(--op-disabled-border)` |
| `--op-checkbox-group-error` | `var(--op-error)` |
| `--op-checkbox-group-hover` | `var(--op-hover)` |
| `--op-checkbox-group-icon` | `var(--op-icon)` |
| `--op-checkbox-group-pending` | `var(--op-pending)` |
| `--op-checkbox-group-pressed` | `var(--op-pressed)` |
| `--op-checkbox-group-ripple` | `var(--op-ripple)` |
| `--op-checkbox-group-success` | `var(--op-success)` |
| `--op-checkbox-group-background` | `var(--op-surface)` |
| `--op-checkbox-group-text` | `var(--op-text-primary)` |
| Slot | Place |
| :--- | :--- |
| `option` | Replaces an option label; receives the item. |
| `before_options` | Before the options. |
| `after_options` | After the options. |

### Strings

| Prop | Key | Default |
| :--- | :--- | :--- |
| `loading_label` | `checkbox-group.loading` | `Loading` |
| `label` | `checkbox-group.label` | `Options` |

### Icons

| Prop | Key | Default |
| :--- | :--- | :--- |
| `checked_icon` | `checkbox-group.checked` | `check` |

Icons supplied as item data or through the existing `icon` prop remain caller-owned.
