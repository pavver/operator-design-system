# VideoStream

Live camera and media stream playback shell around an HTML `<video>` element.

Status: **draft** on web.

## Use

Use `VideoStream` when displaying live camera feeds, WebRTC/MediaStream feeds, or surveillance telemetry.
Do not use `VideoStream` for static pre-recorded embedded clips with iframes; use [Video](video.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Default aspect ratio | `16:9` (`1.7778`). |
| Border | Standard `0.25 DU` inside the bounds. |

## Behavior

* Embeds a native `<video>` element with hardware-accelerated playback.
* Can bind directly to a streaming URL or an active browser `MediaStream`.
* Reflects connecting, playing, stalled, and error states automatically.
* Shows an interactive overlay with a pulsing live badge, mute toggle, and fullscreen button.
* Overlay fades out automatically during playback inactivity.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `src` | signal string | none | Video source URL. |
| `stream` | signal MediaStream | none | Live browser MediaStream object. |
| `state` | VideoStreamState | none | Explicit playback state override. |
| `title` | string | `"Live stream"` | Accessible title. |
| `ratio` | f32 | `1.7778` | Aspect ratio (width / height). |
| `max_width` | DU | none | Maximum width in DU. |
| `max_height` | DU | none | Maximum height in DU. |
| `muted` | bool | `true` | Mute state; defaults to true for autoplay compatibility. |
| `autoplay` | bool | `true` | Autoplay flag. |
| `live` | bool | `true` | Displays pulsing LIVE badge. |
| `on_error` | callback string | none | Callback when playback error occurs. |
| `header_slot` | Slot | none | Custom header overlay slot. |
| `controls_slot` | Slot | none | Custom bottom controls slot. |
| `overlay_slot` | Slot | none | Full-frame overlay slot (e.g. computer vision bounding boxes). |
| `connecting_slot` | Slot | none | Custom connecting state placeholder slot. |
| `error_slot` | Slot | none | Custom error state placeholder slot. |
| `connecting_label` | string | `"Connecting stream..."` | Connecting status text. |
| `stalled_label` | string | `"Stream stalled..."` | Stalled status text. |
| `error_label` | string | `"Video stream error"` | Error status text. |
| `load_error_label` | string | `"Failed to load video stream"` | Load failure error message. |
| `empty_label` | string | `"No active stream"` | Empty stream status message. |
| `live_label` | string | `"LIVE"` | Live badge text. |
| `mute_label` | string | `"Mute"` | Mute button label. |
| `fullscreen_label` | string | `"Fullscreen"` | Fullscreen button label. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `max_width` | DU | `--op-video-stream-max-width` | `100%` |
| `max_height` | DU | `--op-video-stream-max-height` | `100%` |
| `border_width` | DU | `--op-video-stream-border-width` | `0.25 DU` |

| Color variable | Default |
| :--- | :--- |
| `--op-video-stream-bg` | `#000000` |
| `--op-video-stream-border` | `var(--op-border)` |

`class` and `style` apply to the root container.

### Strings

| Key | Default |
| :--- | :--- |
| `video-stream.title` | `"Live stream"` |
| `video-stream.connecting` | `"Connecting stream..."` |
| `video-stream.stalled` | `"Stream stalled..."` |
| `video-stream.error` | `"Video stream error"` |
| `video-stream.load-error` | `"Failed to load video stream"` |
| `video-stream.empty` | `"No active stream"` |
| `video-stream.live` | `"LIVE"` |
| `video-stream.mute` | `"Mute"` |
| `video-stream.fullscreen` | `"Fullscreen"` |

### Icons

| Key | Default |
| :--- | :--- |
| Offline placeholder | `icons::VIDEOCAM` |
| Spinner indicator | `icons::REFRESH` |
| Error icon | `icons::ERROR` |
| Mute toggle | `icons::VISIBILITY` |
| Fullscreen toggle | `icons::OPEN_IN_NEW` |

## Accessibility

The video container is assigned `role="region"` and receives the accessible stream title.
Overlay buttons are fully keyboard accessible and include descriptive ARIA labels.
Playback state changes are communicated visually and semantically.
