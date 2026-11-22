//! Live examples for swipe-item.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn swipe_item() -> AnyView {
    let status = RwSignal::new(String::from("Drag item horizontally"));

    let on_left = Callback::new(move |()| {
        status.set("Triggered left action (Archive)".to_string());
    });

    let on_right = Callback::new(move |()| {
        status.set("Triggered right action (Delete)".to_string());
    });

    view! {
        <Example title="Swipeable List Item" note="Swipe left or right to reveal contextual actions." block=true>
            <div style="max-width: calc(var(--op-du) * 125); width: 100%; display: flex; flex-direction: column; gap: calc(var(--op-du) * 3);">
                <SwipeItem
                    left=Slot::from(move || view! { <span>"ARCHIVE"</span> })
                    right=Slot::from(move || view! { <span>"DELETE"</span> })
                    on_left=on_left
                    on_right=on_right
                >
                    <div style="padding: calc(var(--op-du) * 4); display: flex; justify-content: space-between; align-items: center;">
                        <span style="font-weight: 500;">"Alert notification #402"</span>
                        <span style="font-size: var(--op-type-caption-size); color: var(--op-text-secondary);">"01:28"</span>
                    </div>
                </SwipeItem>
                <div style="font-size: var(--op-type-caption-size); color: var(--op-text-secondary);">
                    {move || status.get()}
                </div>
            </div>
        </Example>
    }
    .into_any()
}
