//! Live examples for MediaKeyboard (unified Emoji, Sticker, and GIF dispatching).

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

const SVG_STICKER_ODS: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='120' height='120' viewBox='0 0 120 120'><polygon points='60,10 75,45 115,50 85,80 95,115 60,95 25,115 35,80 5,50 45,45' fill='%23eab308'/><text x='60' y='70' fill='%23000' font-family='monospace' font-weight='bold' font-size='16' text-anchor='middle'>STAR</text></svg>";

pub fn media_keyboard() -> AnyView {
    let picked_summary = RwSignal::new(String::from("None"));

    let pack = StickerPack::new(
        "ods_demo",
        "ODS Pack",
        SVG_STICKER_ODS,
        StickerFormat::VectorSvg,
        vec![StickerItem::new(
            "st_star",
            "Star",
            SVG_STICKER_ODS,
            StickerFormat::VectorSvg,
            "⭐️",
        )],
    );

    view! {
        <div class="demo-stack" style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 4);width:100%;max-width:calc(var(--op-du) * 120);">
            <Example
                title="Unified Media Keyboard (Phone Docked Mode)"
                note="Switch between Emoji, Stickers, and GIFs with search and quick backspace action."
                block=true
            >
                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 2);width:100%;max-width:calc(var(--op-du) * 95);margin:0 auto;">
                    <div style="font-family:monospace;font-size:13px;color:var(--op-color-text);">
                        {move || format!("Last dispatched item: {}", picked_summary.get())}
                    </div>

                    <MediaKeyboard
                        desktop=false
                        sticker_packs=vec![pack]
                        on_select_emoji=Callback::new(move |em| picked_summary.set(format!("Emoji: {em}")))
                        on_select_sticker=Callback::new(move |st: StickerAttachment| picked_summary.set(format!("Sticker: {:?}", st.sticker_id)))
                        on_select_gif=Callback::new(move |g: GifItem| picked_summary.set(format!("GIF: {}", g.title)))
                        on_backspace=Callback::new(move |()| picked_summary.set("Backspace clicked".to_string()))
                    />
                </div>
            </Example>
        </div>
    }.into_any()
}
