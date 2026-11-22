# ChatMessage

Message bubble for chat, discussion threads, and tactical communication streams.
It supports avatar pictures, sender attribution and roles, timestamp stamps, delivery confirmation, TTL self-destruct timers, quoted replies, forwarded attribution, reactions, and polymorphic attachments (media grids, voice waveforms, audio, circular video notes, stickers, contacts, polls, web page previews, invoices, and stories).

It ships in the media extension: the `chat-message` feature is in the `media` and `extensions` groups, not in `full`.

Status: **draft** on web.

## Use

Use `ChatMessage` in conversational interfaces, telemetry channels, customer support feeds, and activity streams.
Do not use `ChatMessage` for global system alarms; use [Banner](banner.md) or [Toast](toast.md) instead.
When displaying standalone 1 to 3 emojis, `ChatMessage` automatically switches to the large isolated emoji presentation without a bubble container.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (Operator square geometry; circular exception for video notes and avatars). |
| Standard border | Standard `0.25 DU` outline around the bubble and attachment cards. |
| Emphasis border | `0.5 DU` on active borders, video note circular borders, and stop signs. |
| Padding | `2 DU` inside the message bubble (`--op-chat-message-padding`). |
| Avatar size | `8 DU` square (`--op-avatar-default`). |
| Video note size | `40 DU` circular viewfinder diameter. |
| Sticker size | `35 DU` standalone bounding box. |
| Stop sign size | `3.75 DU` symmetrical regular octagon with centered exclamation mark. |

## Behavior

* Received messages align to the start of the container with subtle surface background.
* Sent messages align to the end with raised surface background.
* Consecutive messages from the same author group tightly with reduced spacing.
* Grouped messages omit duplicate sender name and avatar headers until the boundary message.
* Media items support spoiler overlays that unmask upon click.
* Expiring self-destruct messages display a live countdown badge that pulses when ten seconds or less remain.
* Pure 1 to 3 emoji messages render borderless and enlarged without a surrounding bubble.
* Interactive polls update vote totals and bar percentages upon option selection.
* Syntax-tagged code blocks include an interactive one-click clipboard copy button.
* Quoted replies display an accent left indicator bar and author excerpt.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `message` | `Option<OperatorMessage>` | `None` | Complete structured message data model. |
| `sent` | `bool` | `false` | True if sent by the local operator; false if received. |
| `name` | `string` | none | Visible sender name. |
| `role` | `string` | none | Tactical role tag badge (e.g. `OPERATOR`, `DISPATCHER`). |
| `stamp` | `string` | none | Timestamp or delivery time string. |
| `avatar` | `string` | none | Avatar image URL. |
| `text` | `string` | none | Plain text or Markdown message body. |
| `content` | `Option<MessageContent>` | none | Polymorphic content payload variant. |
| `delivery_status` | `Option<DeliveryStatus>` | none | Delivery confirmation state (`Sending`, `Sent`, `Delivered`, `Read`, `Failed`). |
| `reply_to` | `Option<MessageReply>` | none | Quoted message reply reference. |
| `forward_info` | `Option<ForwardHeader>` | none | Forwarded message origin attribution. |
| `is_edited` | `bool` | `false` | Edited content mark. |
| `ttl` | `Option<MessageTtlInfo>` | none | Self-destruct countdown timer details. |
| `grouping` | `Option<MessageGrouping>` | `Single` | Layout grouping (`Single`, `First`, `Middle`, `Last`). |
| `reactions` | `Option<Vec<MessageReaction>>` | none | List of emoji reaction counters. |
| `inline_keyboard` | `Option<Vec<Vec<InlineKeyboardButton>>>` | none | Grid of interactive bot action buttons. |
| `on_action` | `Callback<String>` | none | Event callback when an inline keyboard action button is clicked. |
| `on_reaction_click` | `Callback<String>` | none | Event callback when an emoji reaction chip is clicked. |
| `on_add_reaction_click` | `Callback<()>` | none | Event callback when the add reaction button is clicked. |
| `name_slot` | `Slot` | none | Custom sender name slot. |
| `stamp_slot` | `Slot` | none | Custom timestamp slot. |
| `avatar_slot` | `Slot` | none | Custom avatar slot. |
| `padding` | `DU` | `--op-chat-message-padding` | Inner bubble padding. |
| `gap` | `DU` | `--op-chat-message-gap` | Spacing between elements. |
| `font_size` | `TU` | `--op-chat-message-font-size` | Text font size. |
| `class` | `string` | none | CSS classes on the root element. |
| `style` | `string` | none | Inline CSS on the root element. |

## Subcomponents

* `DeliveryStatus`: Renders micro-spinner (`Sending`), single check (`Sent`), double check (`Delivered`), accent double check (`Read`), or octagon stop sign (`Failed`).
* `ChatStatusBadge`: Renders conversation thread state badges including `@` personal mention badges, unread count pills, rotating sync indicators, and pinned/muted marks.
* `TtlIndicator`: Renders self-destruct timers with encryption locks, duration text, and urgent pulse styling.
* `ReactionsRow`: Renders reaction chips with counts and selection states, plus an optional add reaction button.
* `EmojiText`: Renders text replacing custom emoji shortcodes with app-provided high-resolution graphics.
* `IsolatedEmojiView`: Renders 1 to 3 isolated emojis in large display size with floating timestamp.

## Customization

| Metric prop | Unit | Variable | Default |
| :--- | :--- | :--- | :--- |
| `padding` | `DU` | `--op-chat-message-padding` | `2 DU` |
| `gap` | `DU` | `--op-chat-message-gap` | `1 DU` |
| `font_size` | `TU` | `--op-chat-message-font-size` | `inherit` |

| Color variable | Default |
| :--- | :--- |
| `--op-chat-message-border` | `var(--op-border)` |
| `--op-chat-message-sent-bg` | `var(--op-surface-raised)` |
| `--op-chat-message-sent-text` | `var(--op-text-primary)` |
| `--op-chat-message-received-bg` | `var(--op-surface-subtle)` |
| `--op-chat-message-received-text` | `var(--op-text-primary)` |
| `--op-chat-message-name-color` | `var(--op-text-secondary)` |
| `--op-chat-message-stamp-color` | `var(--op-text-secondary)` |
| `--op-chat-accent` | `var(--op-accent)` |

`class` and `style` apply to the root element.

### Strings

`ChatMessage` does not hardcode user visible copy.
All text labels, headers, and excerpt texts are provided via props.
Built-in utility labels (`edited`, `COPIED`, `SPOILER`, `PAID`) can be tailored through application props or customized slots.

### Icons

`ChatMessage` utilizes Material Symbols Sharp from `operator_ui_tokens::icons`:
* `CHECK` for sent delivery status, read double checks, and code block copied confirmation.
* `LOCK` for encrypted self-destruct TTL badges.
* `REFRESH` for conversation sync indicators.
* `ADD` for adding emoji reactions.
* `CONTENT_COPY` for code block copy actions.
* `OPEN_IN_NEW` for external web page links.

Missing glyphs in token set:
Play/pause media controls, audio waveforms, stop sign polygon, and file/contact badges are rendered via tactical geometric SVG and Unicode symbols.

## Accessibility

Avatars and decorative icons have `aria-hidden="true"`.
Interactive reaction chips, poll options, and inline action buttons are accessible via keyboard with proper button roles and ARIA labels.
Delivery status indicators and TTL timers feature explicit `role="status"` and localized `aria-label` descriptors.
Spoiler text spans provide accessible button semantics and can be toggled via pointer click or keyboard activation.
