//! Live examples for EmojiPicker.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

const SVG_CUSTOM_STAR: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='32' height='32' viewBox='0 0 32 32'><polygon points='16,2 20,11 30,12 22,19 25,29 16,23 7,29 10,19 2,12 12,11' fill='%23eab308'/></svg>";
const SVG_CUSTOM_GEAR: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='32' height='32' viewBox='0 0 32 32'><circle cx='16' cy='16' r='10' fill='%2300a0ff'/></svg>";

pub fn emoji_picker() -> AnyView {
    let chosen_emoji = RwSignal::new(String::from("None"));

    let mut custom_map = CustomEmojiMap::new();
    custom_map.insert(":ods_star:", SVG_CUSTOM_STAR);
    custom_map.insert(":ods_gear:", SVG_CUSTOM_GEAR);

    view! {
        <div class="demo-stack" style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 4);width:100%;max-width:calc(var(--op-du) * 120);">
            <Example
                title="Standard Emoji Picker & Search"
                note="Category tabs, quick search filtering, and custom emoji support."
                block=true
            >
                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 2);width:100%;max-width:calc(var(--op-du) * 90);margin:0 auto;">
                    <div style="font-family:monospace;font-size:13px;color:var(--op-color-text);">
                        {move || format!("Selected: {}", chosen_emoji.get())}
                    </div>

                    <EmojiPicker
                        custom_emojis=custom_map
                        on_select=Callback::new(move |em| chosen_emoji.set(em))
                    />
                </div>
            </Example>
        </div>
    }.into_any()
}
