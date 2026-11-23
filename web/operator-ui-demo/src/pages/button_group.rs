//! Live examples for ButtonGroup.

use crate::i18n::l;
use crate::kit::{Example, backend, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn button_group() -> AnyView {
    let b = backend();
    let enabled = RwSignal::new(true);
    view! {
        <Example
            title="Standard joined buttons"
            note="Adjacent buttons share borders. While one action runs, the others wait."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <ButtonGroup>
                    <ActionButton text="Copy" icon=icons::CONTENT_COPY on_click=b.action("copy")/>
                    <ActionButton text="Cut" on_click=b.action("cut")/>
                    <ActionButton text="Paste" on_click=b.action("paste")/>
                </ButtonGroup>
            </div>
        </Example>
        <Example
            title="Icon buttons group"
            note="Joined icon actions with their own pending and result feedback."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <ButtonGroup>
                    <IconButton icon=icons::EDIT label="Edit" on_async_click=b.action("edit")/>
                    <IconButton icon=icons::REFRESH label="Refresh" on_async_click=b.action("refresh")/>
                    <IconButton icon=icons::DELETE label="Delete" on_async_click=b.action("delete")/>
                </ButtonGroup>
            </div>
        </Example>
        <Example
            title="Whole group switch"
            note="One enabled prop turns every button of the group on or off."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <ActionButton
                    text=Signal::derive(move || if enabled.get() { l("Disable the group", "Вимкнути групу").get() } else { l("Enable the group", "Увімкнути групу").get() })
                    on_click=instant(move || enabled.update(|v| *v = !*v))
                />
                <ButtonGroup enabled=enabled>
                    <ActionButton text="Start" on_click=b.action("start")/>
                    <ActionButton text="Pause" on_click=b.action("pause")/>
                    <ActionButton text="Stop" on_click=b.action("stop")/>
                </ButtonGroup>
            </div>
        </Example>
        <Example
            title="Parallel actions"
            note="With exclusive off, every button runs independently."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <ButtonGroup exclusive=false>
                    <ActionButton text="Camera 1" on_click=b.action("camera 1")/>
                    <ActionButton text="Camera 2" on_click=b.action("camera 2")/>
                    <ActionButton text="Camera 3" on_click=b.action("camera 3")/>
                </ButtonGroup>
            </div>
        </Example>
        <Example
            title="Spread mode"
            note="Stretches buttons to fill available width."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <ButtonGroup spread=true>
                    <ActionButton text="Left" on_click=b.action("align left")/>
                    <ActionButton text="Center" on_click=b.action("align center")/>
                    <ActionButton text="Right" on_click=b.action("align right")/>
                </ButtonGroup>
            </div>
        </Example>
        <Example
            title="Vertical mode"
            note="Vertical stack sharing horizontal borders."
            block=true
        >
            <div class="demo-stack demo-narrow">
                <ButtonGroup vertical=true>
                    <ActionButton text="Option A" on_click=b.action("option a")/>
                    <ActionButton text="Option B" on_click=b.action("option b")/>
                    <ActionButton text="Option C" on_click=b.action("option c")/>
                </ButtonGroup>
            </div>
        </Example>
    }
    .into_any()
}
