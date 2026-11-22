# ChatInput

High-density chat input bar with multi-line auto-growing textarea, quoted reply/edit header, and media record button switcher.
It integrates smoothly with the Operator async state machine (`Idle`, `Pending`, `Success`, `Error`).

It ships in the media extension: the `chat-input` feature is in the `media` and `extensions` groups, not in `full`.

Status: **draft** on web.

## Use

Use `ChatInput` as the primary composer bar in instant messaging layouts, collaborative channels, and task comments.
The textarea expands vertically as content is typed and smoothly scrolls beyond the maximum height limit.
When the input is empty, the trailing action displays `MediaRecordButton`; typing any character instantly reveals the send button.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Standard border | `0.25 DU` container outline border. |
| Minimum height | `9 DU` (`--op-chat-input-min-height`). |
| Maximum height | `29 DU` (`--op-chat-input-max-height`). |
| Font size | `13 TU` (`--op-chat-input-font-size`). |

## Behavior

* Typing multi-line text dynamically auto-grows the textarea height.
* Pressing `Enter` dispatches `on_send`; pressing `Shift+Enter` inserts a newline.
* When text is empty, the control renders the hold-to-record `MediaRecordButton`.
* Quoted messages and active edits display a top banner with a dismiss action.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | `Signal<String>` | required | Controlled input text signal. |
| `on_input` | `Callback<String>` | required | Callback when input text changes. |
| `on_send` | `Callback<()>` | required | Callback when send action is triggered. |
| `reply` | `Option<MessageReply>` | none | Quoted message reply reference. |
| `is_editing` | `bool` | `false` | Whether currently editing an existing message. |
| `on_cancel_reply` | `Option<Callback<()>>` | none | Action when reply/edit banner is dismissed. |
| `on_attach` | `Option<Callback<()>>` | none | Callback for attachments button. |
| `on_toggle_media` | `Option<Callback<()>>` | none | Callback for media keyboard toggle. |
| `state` | `ControlState` | `Idle` | Send button state (`Idle`, `Pending`, `Success`, `Error`). |
| `placeholder` | `string` | `"Type a message..."` | Textarea placeholder text. |
| `min_height` | `f32` | `9.0` | Minimum bar height in Design Units. |
| `max_height` | `f32` | `29.0` | Maximum expanded height in Design Units. |
| `font_size` | `f32` | `13.0` | Textarea font size in Type Units. |
| `leading` | `Slot` | none | Leading slot before input box. |
| `trailing` | `Slot` | none | Trailing slot before send/record button. |
| `attachments` | `Slot` | none | Custom slot replacing attachments button. |
| `class` | `string` | none | Custom CSS classes. |
| `style` | `string` | none | Custom inline style overrides. |

## Customization

### Metric props

| Prop | CSS variable | Default | Unit |
| :--- | :--- | :--- | :--- |
| `min_height` | `--op-chat-input-min-height` | `9.0` | DU |
| `max_height` | `--op-chat-input-max-height` | `29.0` | DU |
| `font_size` | `--op-chat-input-font-size` | `13.0` | TU |

### Color variables

| Variable | Default | Role |
| :--- | :--- | :--- |
| `--op-chat-input-bg` | `var(--op-surface)` | Input bar background. |
| `--op-chat-input-border` | `var(--op-border)` | Container outline border. |
| `--op-chat-input-text` | `var(--op-text-primary)` | Input text color. |
| `--op-chat-input-placeholder` | `var(--op-text-secondary)` | Placeholder text color. |
| `--op-chat-input-accent` | `var(--op-accent)` | Focus border and send button color. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `chat-input.placeholder` | `"Type a message..."` | Textarea placeholder. |
| `chat-input.send` | `"Send"` | Accessible send label. |
| `chat-input.attach` | `"Attach"` | Accessible attachment label. |
| `chat-input.media` | `"Media"` | Accessible media keyboard toggle label. |
| `chat-input.cancel` | `"Cancel"` | Accessible cancel label. |
| `chat-input.editing` | `"Editing message"` | Edit bar preview title. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `chat-input.send-icon` | `icons::ARROW_UPWARD` | Send button arrow icon. |
| `chat-input.attach-icon` | `icons::ADD` | Attachments icon. |
| `chat-input.media-icon` | `icons::SETTINGS` | Media keyboard toggle icon. |
| `chat-input.close-icon` | `icons::CLOSE` | Cancel preview icon. |

## Accessibility

* Auto-growing textarea maintains keyboard accessibility and standard copy-paste bindings.
* Reply and edit preview headers carry descriptive text with cancel actions.
* Send button reflects `aria-busy` and async Operator states.
