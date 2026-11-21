//! Live examples for common.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui::prelude::*;

pub const SEARCH_CAMERAS: [&str; 10] = [
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

/// Cameras returned by the simulated load.
const CAMERAS: [(&str, &str, &str); 3] = [
    ("cam-3", "Gate camera", "Online · 12 ms · H.265"),
    ("cam-5", "Yard camera", "Online · 31 ms · H.264"),
    ("cam-8", "Garage", "Offline since 09:14"),
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum LoadPhase {
    Loaded,
    Empty,
    Failed,
}

/// Full load cycle against the simulated backend: Skeleton while loading,
/// then the list, an EmptyState, or a compact error EmptyState with Retry.
#[component]
fn LoadCycle() -> impl IntoView {
    let b = backend();
    let phase = RwSignal::new(LoadPhase::Loaded);
    let skeleton = RwSignal::new(false);
    let empty_data = RwSignal::new(0_usize);
    let generation = StoredValue::new(0_u32);

    let load = move || async move {
        generation.update_value(|g| *g += 1);
        let current = generation.get_value();
        // Waits under 300 ms show no skeleton, so fast answers do not flicker.
        spawn_local(async move {
            sleep_ms(300).await;
            if generation.get_value() == current {
                skeleton.set(true);
            }
        });
        let result = b.call("load cameras".to_owned()).await;
        if generation.get_value() == current {
            generation.update_value(|g| *g += 1);
            skeleton.set(false);
            phase.set(match (&result, empty_data.get_untracked() == 1) {
                (ActionResult::Success, false) => LoadPhase::Loaded,
                (ActionResult::Success, true) => LoadPhase::Empty,
                _ => LoadPhase::Failed,
            });
        }
        result
    };

    let content = move || {
        if skeleton.get() {
            return (0..CAMERAS.len())
                .map(|_| {
                    view! {
                        <div class="demo-skeleton-row">
                            <Skeleton circle=true width=dim::AVATAR_HEADER height=dim::AVATAR_HEADER/>
                            <div class="demo-skeleton-text"><Skeleton lines=2/></div>
                        </div>
                    }
                })
                .collect_view()
                .into_any();
        }
        match phase.get() {
            LoadPhase::Loaded => CAMERAS
                .iter()
                .map(|(id, name, info)| {
                    let status = if info.starts_with("Offline") {
                        PresenceStatus::Offline
                    } else {
                        PresenceStatus::Online
                    };
                    view! {
                        <ListItem
                            title=*name
                            subtitle=*info
                            leading=ViewFn::from(move || view! { <Avatar id=*id name=*name status=status/> })
                        />
                    }
                })
                .collect_view()
                .into_any(),
            LoadPhase::Empty => view! {
                <EmptyState
                    icon=icons::VIDEOCAM
                    title="No cameras yet"
                    description="Cameras appear here after you pair them on the local network."
                    actions=ViewFn::from(move || view! { <ActionButton text="Pair camera" on_click=b.action("pair camera")/> })
                />
            }
            .into_any(),
            LoadPhase::Failed => view! {
                <EmptyState
                    icon=icons::ERROR
                    compact=true
                    title="Could not load cameras"
                    description="The device did not answer in time."
                    actions=ViewFn::from(move || view! {
                        <ActionButton text="Retry" icon=icons::REFRESH on_click=AsyncAction::new(move |()| load())/>
                    })
                />
            }
            .into_any(),
        }
    };

    view! {
        <div class="demo-load-controls">
            <ActionButton text="Reload" icon=icons::REFRESH on_click=AsyncAction::new(move |()| load())/>
            <div class="demo-load-mode">
                <SegmentedControl
                    items=vec![SegmentItem::text("Cameras"), SegmentItem::text("Empty")]
                    selected=empty_data
                />
            </div>
        </div>
        <div class="demo-list">{content}</div>
    }
}

pub fn load_cycle_example() -> AnyView {
    view! {
        <Example
            title="Load cycle"
            note="Reload asks the simulated backend. Success shows the list, or EmptyState when Empty is selected; an error shows the compact error state with Retry. Answers under 300 ms show no skeleton."
            block=true
        >
            <LoadCycle/>
        </Example>
    }
    .into_any()
}
