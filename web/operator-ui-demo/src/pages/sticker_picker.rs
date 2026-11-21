//! Live examples for StickerPicker.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

const SVG_STICKER_1: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='120' height='120' viewBox='0 0 120 120'><polygon points='60,10 75,45 115,50 85,80 95,115 60,95 25,115 35,80 5,50 45,45' fill='%23eab308'/><text x='60' y='70' fill='%23000' font-family='monospace' font-weight='bold' font-size='16' text-anchor='middle'>STAR</text></svg>";
const SVG_STICKER_2: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='120' height='120' viewBox='0 0 120 120'><rect width='120' height='120' fill='%2300a0ff'/><text x='60' y='65' fill='%23fff' font-family='monospace' font-weight='bold' font-size='18' text-anchor='middle'>TACTICAL</text></svg>";
const SVG_STICKER_3: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='120' height='120' viewBox='0 0 120 120'><circle cx='60' cy='60' r='50' fill='%2322c55e'/><text x='60' y='67' fill='%23fff' font-family='monospace' font-weight='bold' font-size='18' text-anchor='middle'>READY</text></svg>";

pub fn sticker_picker() -> AnyView {
    let picked_info = RwSignal::new(String::from("None"));

    let pack = StickerPack::new(
        "ods_tactical",
        "ODS Tactical Pack",
        SVG_STICKER_1,
        StickerFormat::VectorSvg,
        vec![
            StickerItem::new(
                "st1",
                "Gold Star",
                SVG_STICKER_1,
                StickerFormat::VectorSvg,
                "⭐️",
            ),
            StickerItem::new(
                "st2",
                "Tactical Box",
                SVG_STICKER_2,
                StickerFormat::VectorSvg,
                "📦",
            ),
            StickerItem::new(
                "st3",
                "Ready Badge",
                SVG_STICKER_3,
                StickerFormat::VectorSvg,
                "✅",
            ),
        ],
    );

    view! {
        <div class="demo-stack" style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 4);width:100%;max-width:calc(var(--op-du) * 120);">
            <Example
                title="Offline Vector Sticker Packs"
                note="Pack navigation, search filtering, and enlarged preview on hover."
                block=true
            >
                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 2);width:100%;max-width:calc(var(--op-du) * 90);margin:0 auto;">
                    <div style="font-family:monospace;font-size:calc(var(--op-tu) * 13);color:var(--op-text-primary);">
                        {move || format!("Selected sticker: {}", picked_info.get())}
                    </div>

                    <StickerPicker
                        sticker_packs=vec![pack]
                        on_select=Callback::new(move |st: StickerAttachment| {
                            picked_info.set(format!("{:?}", st.sticker_id));
                        })
                    />
                </div>
            </Example>
        </div>
    }.into_any()
}
