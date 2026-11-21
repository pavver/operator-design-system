//! Live examples for range slider.
use crate::i18n::l;
use crate::kit::Example;
use crate::kit::backend;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn range_slider() -> AnyView {
    let b = backend();
    let range = RwSignal::new((20.0f32, 70.0f32));
    let commit_action =
        AsyncAction::new(move |v: (f32, f32)| async move { b.call(format!("range={v:?}")).await });
    view! {
        <Example title="Interactive" block=true>
            <RangeSlider value=range label="Allowed interval" on_commit=commit_action/>
            <p>{move || format!("{} – {}", range.get().0, range.get().1)}</p>
        </Example>
        <Example title="States" block=true>
            <RangeSlider value=RwSignal::new((20.0, 70.0)) enabled=false/>
            <RangeSlider value=RwSignal::new((20.0, 70.0)) state=Some(ControlState::Pending)/>
        </Example>
        <Example title="Customization" block=true>
            <RangeSlider value=RwSignal::new((10.0, 80.0)) height=14.0 thumb_size=8.0 rail_inset=4.0 track_height=1.0 label=l("Interval", "Інтервал")/></Example>
    }.into_any()
}
