# LauncherTile

Square launcher tile for dashboards, tool launchpads, and application grids.

Status: **draft** on web.

## Use

Use `LauncherTile` when presenting an array of application launchers, modules, or tools in a dashboard grid.
Do not use `LauncherTile` for inline form triggers or toolbars; use [ActionButton](action-button.md) or [IconButton](icon-button.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Default dimensions | `24 DU` by `24 DU`. |
| Border | Standard `0.25 DU` inside the bounds. |
| Padding | `2 DU` inside padding. |
| Gap | `2.5 DU` between icon and label. |

## Behavior

* Clicking the tile fires `on_click` or runs `on_async_click`.
* While an async action runs, the tile follows [States](../foundations/states.md): the state border shows the running segment, and after the delay the [delayed spinner](../foundations/states.md#delayed-spinner) replaces the icon at the icon size; the label stays.
* The result flashes the border and replaces the icon with the success or error icon in its tone, then the tile returns to rest.
* `loading` shows the tile as pending from outside.
* Supports keyboard navigation via `Enter` and `Space`.
* When `href` is supplied, renders as an anchor link styled identically to the button.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `label` | string | `"Launch"` | Title text displayed beneath the icon. |
| `icon` | Icon | none | Primary icon. |
| `icon_slot` | Slot | none | Custom icon slot overriding `icon`. |
| `badge_slot` | Slot | none | Top-right badge slot (counter or status indicator). |
| `content_slot` | Slot | none | Custom content replacing the default icon and label. |
| `state` | ControlState | none | Explicit control state override. |
| `enabled` | bool | `true` | Enabled flag; disabled tiles ignore input. |
| `loading` | bool | `false` | Shows the tile as pending from outside. |
| `loading_label`, `success_label`, `error_label` | string | `launcher-tile.loading`, `.success`, `.error` | Accessible names of the spinner and the result icons. |
| `success_icon`, `error_icon` | icon | `launcher-tile.success`, `.error` | Result icons. |
| `selected` | bool | `false` | Selected flag for selectable grid tiles. |
| `on_click` | callback `()` | none | Synchronous click callback. |
| `on_async_click` | AsyncAction | none | Asynchronous click action. |
| `href` | string | none | Navigation target URL. |
| `new_tab` | bool | `false` | Opens link in a new browser tab. |
| `size` | DU | `24.0` | Outer size in DU (`--op-launcher-tile-size`). |
| `icon_size` | DU | `8.0` | Icon size in DU (`--op-launcher-tile-icon-size`). |
| `gap` | DU | `2.5` | Gap between icon and label in DU (`--op-launcher-tile-gap`). |
| `padding` | DU | `2.0` | Inner padding in DU (`--op-launcher-tile-padding`). |
| `border_width` | DU | `0.25` | Border stroke width in DU (`--op-launcher-tile-border-width`). |
| `font_size` | TU | `11.0` | Label font size in TU (`--op-launcher-tile-font-size`). |
| `line_height` | TU | `14.0` | Label line height in TU (`--op-launcher-tile-line-height`). |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `size` | DU | `--op-launcher-tile-size` | `24 DU` |
| `icon_size` | DU | `--op-launcher-tile-icon-size` | `icon_xlarge` (`8 DU`) |
| `gap` | DU | `--op-launcher-tile-gap` | `2.5 DU` |
| `padding` | DU | `--op-launcher-tile-padding` | `gutter` (`2 DU`) |
| `border_width` | DU | `--op-launcher-tile-border-width` | `0.25 DU` |
| `font_size` | TU | `--op-launcher-tile-font-size` | `11 TU` |
| `line_height` | TU | `--op-launcher-tile-line-height` | `14 TU` |

| Color variable | Default |
| :--- | :--- |
| `--op-launcher-tile-background` | `surface` |
| `--op-launcher-tile-text` | `text_primary` |
| `--op-launcher-tile-border` | `border` |
| `--op-launcher-tile-hover` | `hover` |
| `--op-launcher-tile-pressed` | `pressed` |
| `--op-launcher-tile-selection` | `selection` |
| `--op-launcher-tile-disabled-background` | `disabled_surface` |
| `--op-launcher-tile-disabled-text` | `text_disabled` |
| `--op-launcher-tile-icon` | `icon` |
| `--op-launcher-tile-disabled-icon` | `icon_disabled` |
| `--op-launcher-tile-success` | `success` |
| `--op-launcher-tile-error` | `error` |
| `--op-launcher-tile-pending` | `pending` |
| `--op-launcher-tile-accent` | `accent` |

`class` and `style` apply to the root `<button>` or `<a>` element.

### Strings

| Key | Default |
| :--- | :--- |
| `launcher-tile.label` | `"Launch"` |
| `launcher-tile.loading` | `"Loading"` |
| `launcher-tile.success` | `"Success"` |
| `launcher-tile.error` | `"Error"` |

### Icons

| Key | Default |
| :--- | :--- |
| `launcher-tile.success` | `check` |
| `launcher-tile.error` | `close` |

## Accessibility

The tile renders as a native `<button>` or `<a>` element with appropriate ARIA attributes.
While an action runs, sets `aria-busy`; when it cannot be pressed, sets `aria-disabled="true"`.
Keyboard activation via `Enter` or `Space` is fully supported.
