# ChatMessage

Message bubble for chat, discussion threads, and communication streams.
It supports avatar pictures, sender attribution, timestamp stamps, and distinct sent/received geometry.

Status: **draft** on web.

## Use

Use `ChatMessage` in conversation interfaces, customer support panels, or activity threads.
Do not use `ChatMessage` for system alerts or inline help; use [Banner](banner.md) or [Toast](toast.md) instead.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (Operator square geometry). |
| Avatar size | `8 DU` square. |
| Border | Standard `0.25 DU` outline around the bubble. |
| Padding | `2 DU` inside the message bubble (`--op-chat-message-padding`). |

## Behavior

* Received messages align to the start of the container with subtle surface background.
* Sent messages align to the end with raised surface background.
* Avatar and sender name are placed adjacent to the bubble based on direction.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `sent` | bool | `false` | Distinguishes sent messages from received messages. |
| `name` | string | none | Sender's visible name. |
| `stamp` | string | none | Timestamp or delivery status stamp. |
| `avatar` | string | none | Avatar image URL. |
| `text` | string | none | Plain text message body. |
| `name_slot` | Slot | none | Custom sender name slot. |
| `stamp_slot` | Slot | none | Custom timestamp slot. |
| `avatar_slot` | Slot | none | Custom avatar slot. |
| `padding` | DU | `--op-chat-message-padding` | Inner bubble padding. |
| `gap` | DU | `--op-chat-message-gap` | Spacing between elements. |
| `font_size` | TU | `--op-chat-message-font-size` | Text size. |
| `class` | string | none | CSS classes on the root element. |
| `style` | string | none | Inline CSS on the root element. |

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `padding` | DU | `--op-chat-message-padding` | `2 DU` |
| `gap` | DU | `--op-chat-message-gap` | `1 DU` |
| `font_size` | TU | `--op-chat-message-font-size` | `inherit` |

| Color variable | Default |
| :--- | :--- |
| `--op-chat-message-border` | `var(--op-border)` |
| `--op-chat-message-sent-bg` | `var(--op-surface-raised)` |
| `--op-chat-message-sent-text` | `var(--op-text-primary)` |
| `--op-chat-message-received-bg` | `var(--op-surface-subtle)` |
| `--op-chat-message-received-text` | `var(--op-text-primary)` |
| `--op-chat-message-name-color` | `var(--op-text-secondary)` |
| `--op-chat-message-stamp-color` | `var(--op-text-secondary)` |

`class` and `style` apply to the root `div`.

### Strings

`ChatMessage` does not hardcode visible strings; texts are supplied by caller props.

### Icons

`ChatMessage` renders no icons directly.

## Accessibility

Avatars are marked as decorative with `aria-hidden="true"`.
The message container presents natural reading order for screen readers.
