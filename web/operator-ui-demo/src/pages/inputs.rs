//! Examples for text and choice inputs.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

const CAMERAS: [&str; 10] = [
    "Gate North",
    "Gate South",
    "Yard East",
    "Yard West",
    "Parking A",
    "Parking B",
    "Warehouse dock",
    "Server room",
    "Reception",
    "Roof",
];

pub fn text_area() -> AnyView {
    view! {
        <Example title="Auto-growing" note="Grows from 3 to 8 lines, then scrolls." block=true>
            <div class="demo-stack demo-narrow">
                <TextArea value=RwSignal::new(String::new()) placeholder="Describe the incident" label="Incident notes"/>
                <TextArea
                    value=RwSignal::new("Motion at the north gate after 22:00.".to_string())
                    placeholder="Message"
                    label="Message"
                    max_length=120
                    min_rows=2
                />
            </div>
        </Example>
        <Example title="States" block=true>
            <div class="demo-grid-2">
                <TextArea value=RwSignal::new("Saved".to_string()) state=ControlState::Success min_rows=2/>
                <TextArea value=RwSignal::new("Rejected".to_string()) state=ControlState::Error min_rows=2/>
                <TextArea value=RwSignal::new("Sending…".to_string()) state=ControlState::Pending min_rows=2/>
                <TextArea value=RwSignal::new("Read only".to_string()) enabled=false min_rows=2/>
            </div>
        </Example>
    }
    .into_any()
}

pub fn search_field() -> AnyView {
    let b = backend();
    let query = RwSignal::new(String::new());
    let pending = RwSignal::new(false);
    let results = RwSignal::new(CAMERAS.iter().map(|s| (*s).to_string()).collect::<Vec<_>>());
    let search = Callback::new(move |q: String| {
        pending.set(true);
        leptos::task::spawn_local(async move {
            sleep_ms(400).await;
            let q = q.to_lowercase();
            results.set(
                CAMERAS
                    .iter()
                    .filter(|c| c.to_lowercase().contains(&q))
                    .map(|s| (*s).to_string())
                    .collect(),
            );
            pending.set(false);
            b.log(format!("[UI] search \"{q}\""));
        });
    });
    view! {
        <Example title="Live search" note="The query is reported 300 ms after the last key press; the spinner shows while results load. Escape clears." block=true>
            <div class="demo-stack demo-narrow">
                <SearchField value=query placeholder="Search cameras" on_search=search pending=pending/>
                <div class="demo-list">
                    {move || {
                        let list = results.get();
                        if list.is_empty() {
                            view! { <EmptyState icon=icons::SEARCH title="No cameras match" compact=true/> }.into_any()
                        } else {
                            list.into_iter().map(|c| view! { <ListItem title=c/> }).collect_view().into_any()
                        }
                    }}
                </div>
            </div>
        </Example>
    }
    .into_any()
}

pub fn radio_group() -> AnyView {
    let b = backend();
    let mode = RwSignal::new("auto".to_string());
    let codec = RwSignal::new("h265".to_string());
    view! {
        <Example title="Async commit" note="The mark moves only after the request succeeds; the target shows a spinner while pending.">
            <RadioGroup
                label="Day/night mode"
                items=vec![GroupItem::new("auto", "Automatic"), GroupItem::new("day", "Always day"), GroupItem::new("night", "Always night")]
                selected=mode
                on_change=AsyncAction::new(move |v: String| b.call(format!("mode={v}")))
            />
        </Example>
        <Example title="Local, horizontal">
            <RadioGroup
                label="Codec"
                horizontal=true
                items=vec![GroupItem::new("h264", "H.264"), GroupItem::new("h265", "H.265"), GroupItem::new("mjpeg", "MJPEG")]
                selected=codec
            />
        </Example>
        <Example title="Disabled">
            <RadioGroup
                items=vec![GroupItem::new("a", "Option A"), GroupItem::new("b", "Option B")]
                selected=RwSignal::new("a".to_string())
                enabled=false
            />
        </Example>
    }
    .into_any()
}

pub fn combobox() -> AnyView {
    let b = backend();
    let camera = RwSignal::new("Gate North".to_string());
    let tag = RwSignal::new(String::new());
    let options: Vec<String> = CAMERAS.iter().map(|s| (*s).to_string()).collect();
    view! {
        <Example title="Pick from a list" note="Type to filter, use the arrow keys and Enter. Unknown text reverts on blur." block=true>
            <div class="demo-grid-2">
                <Combobox
                    label="Camera"
                    options=options.clone()
                    selected=camera
                    on_select=Callback::new(move |v: String| b.log(format!("[UI] camera={v}")))
                />
                <Combobox
                    label="Tag (custom values allowed)"
                    options=vec!["perimeter".into(), "indoor".into(), "parking".into(), "critical".into()]
                    selected=tag
                    placeholder="Add a tag"
                    allow_custom=true
                />
            </div>
            <span class="op-t-caption demo-muted">{move || format!("Selected camera: {}", camera.get())}</span>
        </Example>
    }
    .into_any()
}

pub fn chip() -> AnyView {
    let online = RwSignal::new(true);
    let alarms = RwSignal::new(false);
    let indoor = RwSignal::new(false);
    let colored_filter = RwSignal::new(true);
    let tags = RwSignal::new(vec![
        "perimeter".to_string(),
        "night".to_string(),
        "critical".to_string(),
    ]);
    view! {
        <Example title="Filter chips">
            <ChipSet>
                <Chip text="Online" selected=online/>
                <Chip text="With alarms" selected=alarms/>
                <Chip text="Indoor" icon=icons::VIDEOCAM selected=indoor/>
                <Chip text="Disabled" selected=RwSignal::new(false) enabled=false/>
            </ChipSet>
        </Example>
        <Example title="Input chips">
            <ChipSet>
                <For each=move || tags.get() key=|t| t.clone() let:tag>
                    {
                        let name = tag.clone();
                        view! {
                            <Chip text=tag on_remove=Callback::new(move |()| tags.update(|t| t.retain(|x| *x != name)))/>
                        }
                    }
                </For>
            </ChipSet>
        </Example>
        <Example title="Static tags">
            <ChipSet>
                <Chip text="H.265"/>
                <Chip text="4K"/>
                <Chip text="IR"/>
            </ChipSet>
        </Example>
        <Example title="Tones">
            <ChipSet>
                <Chip text="Neutral"/>
                <Chip text="Accent" color=ChipColor::Accent/>
                <Chip text="Online" icon=icons::CHECK_CIRCLE color=ChipColor::Success/>
                <Chip text="Degraded" icon=icons::WARNING color=ChipColor::Warning/>
                <Chip text="Alarm" icon=icons::ERROR color=ChipColor::Error/>
                <Chip text="Firmware 5.7" icon=icons::INFO color=ChipColor::Info/>
            </ChipSet>
        </Example>
        <Example title="Custom colors" note="User tags with their own colors; the text stays readable in both modes.">
            <ChipSet>
                <Chip text="perimeter" color=ChipColor::Custom("#3FA7D6".into()) on_remove=Callback::new(|()| {})/>
                <Chip text="night" color=ChipColor::Custom("#8E6CDB".into()) on_remove=Callback::new(|()| {})/>
                <Chip text="parking" color=ChipColor::Custom("#E07AB3".into()) on_remove=Callback::new(|()| {})/>
                <Chip text="yard" color=ChipColor::Custom("#2BB38A".into()) selected=colored_filter/>
                <Chip text="archived" color=ChipColor::Custom("#3FA7D6".into()) enabled=false/>
            </ChipSet>
        </Example>
    }
    .into_any()
}
