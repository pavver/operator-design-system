# GalleryViewer

Full-screen media viewer dialog with multi-touch zoom, pan, thumbnail navigation, video playback, and action slots.

It ships in the media extension: the `gallery-viewer` feature is in the `media` and `extensions` groups, not in `full`.

Status: **draft** on web.

## Use

Use `GalleryViewer` when allowing users to inspect, zoom, navigate, and download photos and videos in full-screen immersion.
Do not use `GalleryViewer` for standard confirmation prompts; use [ConfirmDialog](confirm-dialog.md) or [Dialog](dialog.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Viewport | Full viewport modal (`100vw` by `100vh`). |
| Corner radius | `0 DU` (standard square geometry). |
| Thumbnail size | `12 DU` in carousel strip. |
| Close button | `9 DU`. |

## Behavior

* Opens over the entire screen with a dimmed backdrop.
* Locks background document scroll while active.
* Mouse wheel zooms in and out clamped between `1.0x` and `5.0x`.
* Double-clicking or double-tapping toggles between `3.0x` zoom and identity `1.0x`.
* Dragging pans the media across the viewport when zoomed.
* Arrow keys (`ArrowLeft`, `ArrowRight`) and edge clicks switch between gallery items when not zoomed.
* Center clicking toggles the visibility of the top header, counter, and bottom description.
* Video playback handles load failures by displaying an error state overlay over the poster instead of a black box.
* Closes on `Escape` key press or close button click.
* Traps keyboard focus within the dialog.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `open` | RwSignal bool | required | Visibility state. |
| `items` | signal Vec MediaItem | required | Collection of media items to display. |
| `current_index` | RwSignal usize | `0` | Active media item index. |
| `label` | string | `"Media gallery viewer"` | Accessible dialog label. |
| `close_label` | string | `"Close"` | Close button label. |
| `prev_label` | string | `"Previous item"` | Previous button label. |
| `next_label` | string | `"Next item"` | Next button label. |
| `empty_label` | string | `"Media not found"` | Translatable empty media message. |
| `video_error_label` | string | `"Video unavailable"` | Translatable video playback error message. |
| `download_slot` | Slot | none | Custom download action slot. |
| `share_slot` | Slot | none | Custom share action slot. |
| `actions_slot` | Slot | none | Extra header action buttons. |
| `thumbnail_size` | DU | `12.0` | Carousel thumbnail dimension in DU. |
| `on_change` | callback usize | none | Callback when active item index changes. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `thumbnail_size` | DU | `--op-gallery-viewer-thumb-size` | `12 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-gallery-viewer-bg` | `rgba(0, 0, 0, 0.92)` |

`class` and `style` apply to the full-screen dialog container.

### Strings

| Key | Default |
| :--- | :--- |
| `gallery-viewer.label` | `"Media gallery viewer"` |
| `gallery-viewer.close` | `"Close"` |
| `gallery-viewer.prev` | `"Previous item"` |
| `gallery-viewer.next` | `"Next item"` |
| `gallery-viewer.empty` | `"Media not found"` |
| `gallery-viewer.video-error` | `"Video unavailable"` |
| `gallery-viewer.thumbnails` | `"Media thumbnails"` |

### Icons

| Key | Default |
| :--- | :--- |
| Close icon | `icons::CLOSE` |
| Previous arrow | `icons::CHEVRON_LEFT` |
| Next arrow | `icons::CHEVRON_RIGHT` |
| Video error | `icons::ERROR` |

## Accessibility

The dialog assigns `role="dialog"` with `aria-modal="true"`.
Focus is trapped inside the viewer upon activation.
Pressing the `Escape` key closes the viewer and restores previous focus.
Media items include alternative text descriptions where provided.
