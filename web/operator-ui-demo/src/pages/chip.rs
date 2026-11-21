//! Live examples for chip.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

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
        <Example title="Customization" note="Metric props and slots customize this instance; its translated string follows the language selector.">
            <Chip text=crate::i18n::t("Selection") height=8.0 padding_x=3.0 gap=2.0
                leading=|| view! { <Icon icon=icons::SETTINGS size=4.0/> }
                selected=RwSignal::new(false)/>
        </Example>
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
