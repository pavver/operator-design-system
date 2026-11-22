# InfiniteScroll

Incremental content loader triggered when a bottom sentinel element enters the viewport.

Status: **draft** on web.

## Use

Use InfiniteScroll to paginate continuous streams of data, such as message logs, activity feeds, or search results.
It automatically triggers fetch requests before the user reaches the bottom, and provides built-in pending and error retry states.

## Geometry

| Property | Default metric | Variable |
| :--- | :--- | :--- |
| Lookahead offset | `10 DU` | `--op-infinite-scroll-offset` |
| Indicator row height | `12 DU` | `--op-infinite-scroll-pending-height` |

## Behavior

* A lightweight sentinel element is observed via `IntersectionObserver`.
* When the sentinel intersects within the lookahead margin, `on_load` is invoked.
* While `loading` is true, the sentinel is paused and a loading indicator row is shown.
* If `error` is set, an error row with a retry button appears, allowing the user to resume fetching.
* When `has_more` is false, the sentinel is permanently unmounted.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `on_load` | callback `()` | required | Callback triggered to load the next batch of items. |
| `has_more` | bool signal | `true` | Indicates whether more items remain to be loaded. |
| `loading` | bool signal | `false` | True while an asynchronous fetch is in progress. |
| `error` | `Option<String>` signal | `None` | Optional error message to display in the error row. |
| `on_retry` | callback `()` | none | Callback executed when the retry button is clicked. |
| `offset` | DU metric | `10 DU` | Distance in DU from bottom before triggering `on_load`. |
| `pending_height` | DU metric | `12 DU` | Height of the loading or error message row. |
| `loading_label` | string | `infinite-scroll.loading` | Text displayed during loading. |
| `error_label` | string | `infinite-scroll.error` | Text displayed on failure. |
| `retry_label` | string | `infinite-scroll.retry` | Text for the retry button. |
| `loading_slot` | slot | none | Custom slot to replace the default loading indicator. |
| `error_slot` | slot | none | Custom slot to replace the default error indicator. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `offset` | DU | `--op-infinite-scroll-offset` | `10 DU` |
| `pending_height` | DU | `--op-infinite-scroll-pending-height` | `12 DU` |

## Strings

| Key | Default (en) | Description |
| :--- | :--- | :--- |
| `infinite-scroll.loading` | `Loading more…` | Message shown in pending row. |
| `infinite-scroll.error` | `Failed to load` | Default error message. |
| `infinite-scroll.retry` | `Retry` | Button text for retry action. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `icons::ERROR` | Material error | Icon displayed next to error message. |
| `icons::REFRESH` | Material refresh | Icon shown inside the retry button. |

## Accessibility

* The loading indicator row uses `role="status"` and `aria-busy="true"`.
* The error message row uses `role="alert"`.
