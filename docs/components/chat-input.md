# ChatInput

High-density multi-line auto-growing chat input bar for conversational and dispatch feeds.
It features automatic vertical expansion, quoted message reply previews, attachment slots, send actions with Operator states, and media voice/video recording switches.

Status: **draft** on web.

## Use

Use `ChatInput` as the primary interaction control in conversational feeds, support channels, and tactical dispatch consoles.
When no text is entered, the action slot smoothly presents the [MediaRecordButton](media-record-button.md) for quick voice and video note capture.
When text is entered, it transitions immediately to the [ActionButton](action-button.md) style send button with Operator pending, success, and error states.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Standard border | `0.25 DU` standard container border. |
| Minimum height | `9 DU` (`--op-chat-input-min-height`). |
| Maximum height | `29 DU` (`--op-chat-input-max-height`). |
| Font size | `13 TU` (`--op-chat-input-font-size`). |

## Behavior

* The input textarea grows automatically as text wraps or line breaks are entered, between 9 DU and 29 DU.
* Pressing `Enter` without `Shift` dispatches the message via `on_send`.
* Pressing `Shift+Enter` inserts a new line.
* Quoting a reply or editing an existing message renders a preview bar above the input with a close button.
* When the input text is empty, the send button switches to the voice/video recording control.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `value` | `Signal<String>` | required | Reactive binding for the message draft text. |
| `on_input` | `Callback<String>` | required | Event on text content changes. |
| `on_send` | `Callback<()>` | required | Event when user submits the message. |
| `reply` | `Option<MessageReply>` | none | Quoted message reply reference. |
| `is_editing` | `bool` | `false` | True when editing a previously sent message. |
| `on_cancel_reply` | `Callback<()>` | none | Event when reply or edit preview is cancelled. |
| `on_attach` | `Callback<()>` | none | Event when attachments button is clicked. |
| `on_toggle_media` | `Callback<()>` | none | Event when media keyboard toggle is clicked. |
| `state` | `ControlState` | `Idle` | Operator async state for send action. |
| `media_mode` | `MediaRecordMode` | `Voice` | Active mode for the media record button. |
| `recording` | `bool` | `false` | True during active recording. |
| `recording_locked` | `bool` | `false` | True during locked hands-free recording. |
| `placeholder` | `string` | none | Input placeholder text. |
| `min_height` | `f32` | `9.0` | Minimum height in Design Units. |
| `max_height` | `f32` | `29.0` | Maximum height in Design Units. |
| `font_size` | `f32` | `13.0` | Font size in Type Units. |
| `leading` | `Slot` | none | Custom slot before the input. |
| `trailing` | `Slot` | none | Custom slot after the input. |
| `attachments` | `Slot` | none | Custom attachments button slot. |
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
| `--op-chat-input-bg` | `var(--op-color-surface)` | Input bar background. |
| `--op-chat-input-border` | `var(--op-color-border)` | Container outline border. |
| `--op-chat-input-text` | `var(--op-color-text)` | Input text color. |
| `--op-chat-input-placeholder` | `var(--op-color-text-secondary)` | Placeholder text color. |
| `--op-chat-input-accent` | `var(--op-color-accent)` | Focus border and send button color. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `chat-input.placeholder` | `"Type a message..."` | Textarea placeholder. |
| `chat-input.send` | `"Send"` | Accessible send label. |
| `chat-input.attach` | `"Attach"` | Accessible attachment label. |
| `chat-input.media` | `"Media"` | Accessible media keyboard toggle label. |
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
