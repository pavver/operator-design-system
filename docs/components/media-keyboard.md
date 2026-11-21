# MediaKeyboard

Generic media keyboard with unified Emoji, Stickers, and GIF dispatching.
It is compatible with custom emoji packs, sticker sets, and animated clips (including Telegram packs), with search filtering, phone docked replacement, and desktop anchored popover support.

Status: **draft** on web.

## Use

Use `MediaKeyboard` in chat inputs, instant messaging layouts, and comment composers.
On desktop, it floats anchored above or below the input field as a Popover.
On phones, it docks to the bottom edge and replaces the software virtual keyboard area.

## Geometry

| Property | Value |
| :--- | :--- |
| Corner radius | `0 DU` (standard square geometry). |
| Standard border | `0.25 DU` container outline border. |
| Desktop width | `85 DU` (`--op-media-keyboard-width`). |
| Height | `70 DU` (`--op-media-keyboard-height`). |

## Behavior

* The bottom navigation bar switches seamlessly between Emoji, Stickers, and GIF tabs.
* On desktop, the component opens anchored to the provided popover target.
* On mobile, it stretches across 100% width docked to the screen bottom.
* Dispatches respective item payloads via `on_select_emoji`, `on_select_sticker`, and `on_select_gif`.
* An optional backspace action button can delete the preceding character from the input.

## API

| Property | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `on_select_emoji` | `Callback<String>` | none | Event reporting chosen emoji. |
| `on_select_sticker` | `Callback<StickerAttachment>` | none | Event reporting chosen sticker. |
| `on_select_gif` | `Callback<GifItem>` | none | Event reporting chosen GIF. |
| `on_backspace` | `Option<Callback<()>>` | none | Action when backspace button is clicked. |
| `open` | `bool` | `true` | Visibility control. |
| `on_close` | `Option<Callback<()>>` | none | Close event callback. |
| `desktop` | `bool` | `true` | Desktop popover vs phone docked panel mode. |
| `anchor` | `Option<Signal<Option<PopoverAnchor>>>` | none | Target anchor for popover placement. |
| `initial_tab` | `Option<MediaKeyboardTab>` | `Emoji` | Active initial tab. |
| `emoji_categories` | `Option<Vec<EmojiCategory>>` | none | Pluggable emoji categories provided by host app. |
| `recent_emojis` | `Option<Vec<EmojiItem>>` | none | Recently used emojis provided by host app. |
| `custom_emojis` | `Option<CustomEmojiMap>` | none | Custom emoji registry. |
| `sticker_packs` | `Option<Vec<StickerPack>>` | none | Sticker packs list. |
| `gifs` | `Option<Vec<GifItem>>` | none | Curated GIF list. |
| `trending_tags` | `Option<Vec<String>>` | none | Trending GIF tags list. |
| `width` | `f32` | `85.0` | Desktop width in Design Units. |
| `height` | `f32` | `70.0` | Container height in Design Units. |
| `class` | `string` | none | Custom CSS classes. |
| `style` | `string` | none | Custom inline style overrides. |

## Customization

### Metric props

| Prop | CSS variable | Default | Unit |
| :--- | :--- | :--- | :--- |
| `width` | `--op-media-keyboard-width` | `85.0` | DU |
| `height` | `--op-media-keyboard-height` | `70.0` | DU |

### Color variables

| Variable | Default | Role |
| :--- | :--- | :--- |
| `--op-media-keyboard-bg` | `var(--op-surface)` | Keyboard background. |
| `--op-media-keyboard-border` | `var(--op-border)` | Keyboard outline border. |
| `--op-media-keyboard-accent` | `var(--op-accent)` | Active tab underline and focus border. |
| `--op-media-keyboard-text` | `var(--op-text-primary)` | Keyboard text color. |

## Strings

| Key | Default English | Description |
| :--- | :--- | :--- |
| `media-keyboard.emoji` | `"Emoji"` | Emoji tab button label. |
| `media-keyboard.stickers` | `"Stickers"` | Stickers tab button label. |
| `media-keyboard.gifs` | `"GIF"` | GIF tab button label. |
| `media-keyboard.backspace` | `"Backspace"` | Accessible label for backspace button. |

## Icons

| Key | Default | Description |
| :--- | :--- | :--- |
| `media-keyboard.backspace-icon` | `icons::CLOSE` | Backspace action icon. |

## Accessibility

* Tab buttons carry `role="tab"` and `aria-selected` status.
* The container is marked as a landmark region with `role="region"`.
