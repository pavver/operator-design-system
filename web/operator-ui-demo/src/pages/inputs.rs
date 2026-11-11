//! Examples for text and choice inputs.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn chip() -> AnyView {
    let online = RwSignal::new(true);
    let alarms = RwSignal::new(false);
    let indoor = RwSignal::new(false);
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
    }
    .into_any()
}
