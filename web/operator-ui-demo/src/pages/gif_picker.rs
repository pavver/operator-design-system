//! Live examples for GifPicker.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

const SVG_PREVIEW_1: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='200' height='150' viewBox='0 0 200 150'><rect width='200' height='150' fill='%231e293b'/><circle cx='100' cy='75' r='40' fill='%2300a0ff' opacity='0.3'/><text x='100' y='80' fill='%23fff' font-family='monospace' font-size='14' text-anchor='middle'>CLIP 1</text></svg>";
const SVG_PREVIEW_2: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='200' height='150' viewBox='0 0 200 150'><rect width='200' height='150' fill='%230f172a'/><polygon points='100,20 150,120 50,120' fill='%2322c55e' opacity='0.3'/><text x='100' y='90' fill='%23fff' font-family='monospace' font-size='14' text-anchor='middle'>CLIP 2</text></svg>";

pub fn gif_picker() -> AnyView {
    let chosen_gif = RwSignal::new(String::from("None"));

    let items = vec![
        GifItem::new(
            "gif1",
            "Radar Waveform",
            SVG_PREVIEW_1,
            "data:video/mp4;base64,AAAAHGZ0eXBtcDQyAAAAAG1wNDJpc29tYXZjMQAAADhtb292AAAAbG12aGQAAAAAAAAAAAAAAAAAAAPoAAAAAAABAAABAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAA",
            vec!["Trending".to_string(), "Reaction".to_string()],
            1.33,
        ),
        GifItem::new(
            "gif2",
            "Green Telemetry Pulse",
            SVG_PREVIEW_2,
            "data:video/mp4;base64,AAAAHGZ0eXBtcDQyAAAAAG1wNDJpc29tYXZjMQAAADhtb292AAAAbG12aGQAAAAAAAAAAAAAAAAAAAPoAAAAAAABAAABAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAA",
            vec!["Cats".to_string(), "Party".to_string()],
            1.33,
        ),
    ];

    view! {
        <div class="demo-stack" style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 4);width:100%;max-width:calc(var(--op-du) * 120);">
            <Example
                title="Looping Video Clips & IntersectionObserver"
                note="Plays video when visible in viewport, pauses automatically when scrolled out."
                block=true
            >
                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 2);width:100%;max-width:calc(var(--op-du) * 90);margin:0 auto;">
                    <div style="font-family:monospace;font-size:13px;color:var(--op-color-text);">
                        {move || format!("Selected: {}", chosen_gif.get())}
                    </div>

                    <GifPicker
                        gifs=items
                        on_select=Callback::new(move |g: GifItem| chosen_gif.set(g.title))
                    />
                </div>
            </Example>
        </div>
    }.into_any()
}
