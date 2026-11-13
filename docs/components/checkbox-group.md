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
| Tile padding | `margin_screen` (`4 DU`) horizontally. |
| Gap between tiles | `gutter` (`2 DU`). |
| Icon | Optional, before the name with a `gutter` (`2 DU`) gap; size set by `icon_size`. |
| Indicator box | `5 DU` square at the right edge, check `3 DU`. |
| Corner radius | `0 DU`. |

### Icon Sizes

| Size | Value | Tile leading padding |
| :--- | :--- | :--- |
| `Small` | `icon_small` (`4 DU`) | `margin_screen` (`4 DU`) |
| `Medium` | `icon_large` (`6 DU`) | `margin_screen` (`4 DU`) |
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
* The parent updates the selected list on success; the tile only shows the result.
* A result is held for `800 ms` and then fades out over `500 ms`.

## API

| Property | Type | Description |
| :--- | :--- | :--- |
| `items` | list of `GroupItem { id, name, icon? }` | The options; `icon` is an optional leading icon. |
| `selected` | list of ids | The confirmed selection. |
| `on_change` | async `((id, bool)) -> ActionResult` | Called with the item id and the requested value. |
| `icon_size` | `Small`, `Medium`, `Large` | Size of the item icons; `Small` by default. |
