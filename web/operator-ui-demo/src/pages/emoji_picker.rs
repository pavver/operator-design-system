//! Live examples for EmojiPicker.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

const SVG_CUSTOM_STAR: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='32' height='32' viewBox='0 0 32 32'><polygon points='16,2 20,11 30,12 22,19 25,29 16,23 7,29 10,19 2,12 12,11' fill='%23eab308'/></svg>";
const SVG_CUSTOM_GEAR: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='32' height='32' viewBox='0 0 32 32'><circle cx='16' cy='16' r='10' fill='%2300a0ff'/></svg>";

pub fn sample_emoji_categories() -> Vec<EmojiCategory> {
    vec![
        EmojiCategory::new(
            "smileys",
            "Smileys",
            "😀",
            vec![
                EmojiItem::new("😀").with_name("grinning face"),
                EmojiItem::new("😃").with_name("grinning face with big eyes"),
                EmojiItem::new("😄").with_name("grinning face with smiling eyes"),
                EmojiItem::new("😁").with_name("beaming face with smiling eyes"),
                EmojiItem::new("😆").with_name("grinning squinting face"),
                EmojiItem::new("😅").with_name("grinning face with sweat"),
                EmojiItem::new("😂").with_name("face with tears of joy"),
                EmojiItem::new("🤣").with_name("rolling on the floor laughing"),
                EmojiItem::new("😊").with_name("smiling face with smiling eyes"),
                EmojiItem::new("😇").with_name("smiling face with halo"),
                EmojiItem::new("🙂").with_name("slightly smiling face"),
                EmojiItem::new("🙃").with_name("upside down face"),
                EmojiItem::new("😉").with_name("winking face"),
                EmojiItem::new("😌").with_name("relieved face"),
                EmojiItem::new("😍").with_name("smiling face with heart eyes"),
                EmojiItem::new("🥰").with_name("smiling face with hearts"),
                EmojiItem::new("😘").with_name("face blowing a kiss"),
                EmojiItem::new("😎").with_name("smiling face with sunglasses"),
                EmojiItem::new("🥳").with_name("partying face"),
                EmojiItem::new("😏").with_name("smirking face"),
                EmojiItem::new("🤔").with_name("thinking face"),
                EmojiItem::new("🤯").with_name("exploding head"),
                EmojiItem::new("😱").with_name("face screaming in fear"),
                EmojiItem::new("🔥").with_name("fire"),
            ],
        ),
        EmojiCategory::new(
            "gestures",
            "Gestures",
            "👍",
            vec![
                EmojiItem::new("👍").with_name("thumbs up"),
                EmojiItem::new("👎").with_name("thumbs down"),
                EmojiItem::new("👊").with_name("oncoming fist"),
                EmojiItem::new("✊").with_name("raised fist"),
                EmojiItem::new("👏").with_name("clapping hands"),
                EmojiItem::new("🙌").with_name("raising hands"),
                EmojiItem::new("🤝").with_name("handshake"),
                EmojiItem::new("🙏").with_name("folded hands"),
                EmojiItem::new("💪").with_name("flexed biceps"),
                EmojiItem::new("✍️").with_name("writing hand"),
                EmojiItem::new("👀").with_name("eyes"),
            ],
        ),
        EmojiCategory::new(
            "animals",
            "Animals & Nature",
            "🐻",
            vec![
                EmojiItem::new("🐶").with_name("dog face"),
                EmojiItem::new("🐱").with_name("cat face"),
                EmojiItem::new("🐭").with_name("mouse face"),
                EmojiItem::new("🐹").with_name("hamster"),
                EmojiItem::new("🐰").with_name("rabbit"),
                EmojiItem::new("🦊").with_name("fox"),
                EmojiItem::new("🐻").with_name("bear"),
                EmojiItem::new("🐼").with_name("panda"),
                EmojiItem::new("🐨").with_name("koala"),
                EmojiItem::new("🦁").with_name("lion"),
                EmojiItem::new("🐸").with_name("frog"),
            ],
        ),
        EmojiCategory::new(
            "objects",
            "Objects & Tactical",
            "💡",
            vec![
                EmojiItem::new("💡").with_name("light bulb"),
                EmojiItem::new("💻").with_name("laptop"),
                EmojiItem::new("📱").with_name("mobile phone"),
                EmojiItem::new("🎯").with_name("direct hit target"),
                EmojiItem::new("🚀").with_name("rocket"),
                EmojiItem::new("⚡").with_name("high voltage"),
                EmojiItem::new("💯").with_name("hundred points"),
                EmojiItem::new("📦").with_name("package box"),
                EmojiItem::new("🛡️").with_name("shield"),
                EmojiItem::new("🔧").with_name("wrench"),
            ],
        ),
    ]
}

pub fn sample_recent_emojis() -> Vec<EmojiItem> {
    vec![
        EmojiItem::new("👍").with_name("thumbs up"),
        EmojiItem::new("❤️").with_name("red heart"),
        EmojiItem::new("🔥").with_name("fire"),
        EmojiItem::new("🎉").with_name("party popper"),
        EmojiItem::new("👏").with_name("clapping hands"),
        EmojiItem::new("⚡").with_name("high voltage"),
        EmojiItem::new("😀").with_name("grinning face"),
        EmojiItem::new("🚀").with_name("rocket"),
    ]
}

pub fn emoji_picker() -> AnyView {
    let chosen_emoji = RwSignal::new(String::from("None"));

    let mut custom_map = CustomEmojiMap::new();
    custom_map.insert(":ods_star:", SVG_CUSTOM_STAR);
    custom_map.insert(":ods_gear:", SVG_CUSTOM_GEAR);

    view! {
        <div class="demo-stack" style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 4);width:100%;max-width:calc(var(--op-du) * 120);">
            <Example
                title="Pluggable Emoji Packs & Search"
                note="Category tabs from host application packs, live search filtering, and custom image emoji support."
                block=true
            >
                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 2);width:100%;max-width:calc(var(--op-du) * 90);margin:0 auto;">
                    <div style="font-family:monospace;font-size:calc(var(--op-tu) * 13);color:var(--op-text-primary);">
                        {move || format!("Selected: {}", chosen_emoji.get())}
                    </div>

                    <EmojiPicker
                        categories=sample_emoji_categories()
                        recent_emojis=sample_recent_emojis()
                        custom_emojis=custom_map
                        on_select=Callback::new(move |em| chosen_emoji.set(em))
                    />
                </div>
            </Example>
        </div>
    }.into_any()
}
