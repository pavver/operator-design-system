# Layout

Application shell layout container with header, footer, start and end drawers, and page content.
Includes PageSticky and PageScroller helper components.

Status: **draft** on web.

## Use

Use Layout as the top-level frame of an application or a scoped viewport section.
It manages structured layout regions without duplicating components like [Header](header.md), [NavigationRail](navigation-rail.md), or [SidePanel](side-panel.md).

## Geometry

| Region | Default metric | Variable |
| :--- | :--- | :--- |
| Header height | `14 DU` | `--op-layout-header-height` |
| Footer height | `12 DU` | `--op-layout-footer-height` |
| Start drawer width | `60 DU` | `--op-layout-start-drawer-width` |
| End drawer width | `60 DU` | `--op-layout-end-drawer-width` |
| Sticky offset X | `4 DU` | `--op-page-sticky-offset-x` |
| Sticky offset Y | `4 DU` | `--op-page-sticky-offset-y` |

## Behavior

* In viewport mode (`container = false`), Layout occupies the full screen (`100vw`, `100vh`) with fixed positioning.
* In container mode (`container = true`), Layout fits within its parent container (`100%` width and height).
* Drawers can be opened or collapsed smoothly using motion tokens.
* PageScroller monitors scroll position and fades in when scrolled past the configured threshold.
* Clicking PageScroller scrolls smoothly back to the top (or bottom in reverse mode).

## API

### Layout

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `container` | bool signal | `false` | Scopes layout to parent container instead of the viewport. |
| `header` | slot | none | Content rendered in the top header bar. |
| `footer` | slot | none | Content rendered in the bottom footer bar. |
| `start_drawer` | slot | none | Content rendered in the left/start navigation drawer. |
| `end_drawer` | slot | none | Content rendered in the right/end utility drawer. |
| `start_drawer_open` | bool signal | `true` | Controls visibility of the start drawer. |
| `end_drawer_open` | bool signal | `true` | Controls visibility of the end drawer. |
| `header_height` | DU metric | `14 DU` | Height of the top header. |
| `footer_height` | DU metric | `12 DU` | Height of the bottom footer. |
| `start_drawer_width` | DU metric | `60 DU` | Width of the start drawer. |
| `end_drawer_width` | DU metric | `60 DU` | Width of the end drawer. |

### PageSticky

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `position` | `StickyPosition` | `BottomEnd` | Edge or corner anchor point. |
| `offset_x` | DU metric | `4 DU` | Horizontal offset from the anchor. |
| `offset_y` | DU metric | `4 DU` | Vertical offset from the anchor. |

### PageScroller

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `scroll_offset` | DU metric | `50 DU` | Scroll distance before the button appears. |
| `position` | `StickyPosition` | `BottomEnd` | Position of the scroller button. |
| `reverse` | bool | `false` | Scrolls to the bottom instead of top. |
| `aria_label` | string | `layout.scroll-top` | Accessible name for screen readers. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `header_height` | DU | `--op-layout-header-height` | `14 DU` |
| `footer_height` | DU | `--op-layout-footer-height` | `12 DU` |
| `start_drawer_width` | DU | `--op-layout-start-drawer-width` | `60 DU` |
| `end_drawer_width` | DU | `--op-layout-end-drawer-width` | `60 DU` |
| `offset_x` | DU | `--op-page-sticky-offset-x` | `4 DU` |
| `offset_y` | DU | `--op-page-sticky-offset-y` | `4 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-layout-background` | `surface` |
| `--op-layout-text` | `text_primary` |
| `--op-layout-divider` | `divider` |

## Strings

| Key | Default (en) | Description |
| :--- | :--- | :--- |
| `layout.scroll-top` | `Scroll to top` | Accessible label for the scroll-to-top button. |
| `layout.scroll-bottom` | `Scroll to bottom` | Accessible label for the reverse scroll button. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `icons::ARROW_UPWARD` | Material arrow upward | Default icon for the PageScroller button. |
| `icons::ARROW_DOWNWARD` | Material arrow downward | Icon used in reverse mode. |

## Accessibility

* Landmarks are used: `role="banner"` for header, `role="contentinfo"` for footer, `role="complementary"` for drawers, and `role="main"` for page content.
* Floating actions maintain a minimum `8 DU` hit target.
