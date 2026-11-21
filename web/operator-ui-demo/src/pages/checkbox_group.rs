//! Live examples for checkbox group.

use crate::kit::{Example, backend};
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn checkbox_group() -> AnyView {
    let b = backend();
    let selected = RwSignal::new(vec!["north".to_string()]);
    let items = vec![
        GroupItem::new("north", "North gate"),
        GroupItem::new("south", "South gate"),
        GroupItem::new("yard", "Yard perimeter"),
    ];
    view! {
        <Example title="Armed zones" note="Each tile runs its own request; several can be pending at once.">
            <div class="demo-narrow">
                <CheckboxGroup
                    items=items
                    selected=selected
                    on_change=AsyncAction::new(move |(id, on): (String, bool)| async move {
                        let r = b.call(format!("zone {id}={on}")).await;
                        if r.is_success() {
                            selected.update(|s| {
                                s.retain(|x| *x != id);
                                if on {
                                    s.push(id);
                                }
                            });
                        }
                        r
                    })
                />
            </div>
        </Example>
        {[("Small icons", GroupIconSize::Small), ("Medium icons", GroupIconSize::Medium), ("Large icons", GroupIconSize::Large)]
            .into_iter()
            .map(|(title, size)| {
                let icon_selected = RwSignal::new(vec!["camera".to_string()]);
                view! {
                    <Example title=title>
                        <div class="demo-narrow">
                            <CheckboxGroup
                                icon_size=size
                                items=vec![
                                    GroupItem::new("camera", "Camera alerts").with_icon(icons::VIDEOCAM),
                                    GroupItem::new("search", "Search results").with_icon(icons::SEARCH),
                                    GroupItem::new("settings", "Settings changes").with_icon(icons::SETTINGS),
                                ]
                                selected=icon_selected
                                on_change=AsyncAction::new(move |(id, on): (String, bool)| async move {
                                    let r = b.call(format!("notify {id}={on}")).await;
                                    if r.is_success() {
                                        icon_selected.update(|s| {
                                            s.retain(|x| *x != id);
                                            if on {
                                                s.push(id);
                                            }
                                        });
                                    }
                                    r
                                })
                            />
                        </div>
                    </Example>
                }
            })
            .collect_view()}
    }
    .into_any()
}
