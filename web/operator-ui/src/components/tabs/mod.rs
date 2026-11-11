use crate::components::primitives::Icon;
use crate::core::spawn_ripple;
use leptos::ev;
use leptos::prelude::*;
use operator_ui_tokens::dim;
use wasm_bindgen::JsCast;

pub(crate) const CSS: &str = include_str!("tabs.css");

/// One tab of [`Tabs`].
#[derive(Clone, Debug, PartialEq)]
pub struct TabItem {
    pub label: String,
    pub icon: Option<operator_ui_tokens::Icon>,
    /// Counter shown after the label; hidden at zero.
    pub count: Option<u32>,
}

impl TabItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            count: None,
        }
    }

    pub fn icon(mut self, icon: operator_ui_tokens::Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }
}

/// Visual style of [`Tabs`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsVariant {
    /// Folder tabs: the active tab rises above an accent baseline and opens into the content.
    #[default]
    Folder,
    /// Underline tabs: a sliding accent line under the active label.
    Line,
}

/// Switches between views of the same level.
#[component]
pub fn Tabs(
    items: Vec<TabItem>,
    selected: RwSignal<usize>,
    #[prop(optional)] variant: TabsVariant,
    /// Stretches the tabs to fill the width equally.
    #[prop(optional)]
    fill: bool,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let list = NodeRef::<leptos::html::Div>::new();
    let indicator = RwSignal::new(None::<(f64, f64)>);
    let count = items.len();

    let measure = move || {
        let Some(el) = list.try_get_untracked().flatten() else {
            return;
        };
        let Ok(tabs) = el.query_selector_all("[role=tab]") else {
            return;
        };
        let i = u32::try_from(selected.get_untracked()).unwrap_or(0);
        if let Some(tab) = tabs
            .item(i)
            .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
        {
            indicator.set(Some((
                f64::from(tab.offset_left()),
                f64::from(tab.offset_width()),
            )));
        }
    };
    Effect::new(move |_| {
        selected.track();
        request_animation_frame(measure);
    });
    let resize = window_event_listener(ev::resize, move |_| measure());
    on_cleanup(move || resize.remove());

    let focus_tab = move |i: usize| {
        selected.set(i);
        if let Some(t) = list.try_get_untracked().flatten().and_then(|el| {
            el.query_selector_all("[role=tab]")
                .ok()
                .and_then(|l| l.item(u32::try_from(i).unwrap_or(0)))
                .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
        }) {
            let _ = t.focus();
        }
    };

    let tabs = items
        .into_iter()
        .enumerate()
        .map(|(i, item)| {
            let active = move || selected.get() == i;
            view! {
                <button
                    type="button"
                    role="tab"
                    class="op-rippled op-tab"
                    aria-selected=move || active().to_string()
                    tabindex=move || if active() { "0" } else { "-1" }
                    data-active=move || active().then_some("")
                    on:pointerdown=move |ev| spawn_ripple(&ev)
                    on:click=move |_| selected.set(i)
                    on:keydown=move |ev| {
                        let n = count.max(1);
                        match ev.key().as_str() {
                            "ArrowRight" => focus_tab((i + 1) % n),
                            "ArrowLeft" => focus_tab((i + n - 1) % n),
                            "Home" => focus_tab(0),
                            "End" => focus_tab(n - 1),
                            _ => {}
                        }
                    }
                >
                    {item.icon.map(|ic| view! { <Icon icon=ic size=dim::ICON_SMALL/> })}
                    <span class="op-tab-label">{item.label}</span>
                    {item.count.filter(|c| *c > 0).map(|c| view! { <span class="op-tab-count">{c.to_string()}</span> })}
                </button>
            }
        })
        .collect_view();

    let variant_name = match variant {
        TabsVariant::Folder => "folder",
        TabsVariant::Line => "line",
    };
    view! {
        <div class="op-tabs" data-variant=variant_name data-fill=fill.then_some("")>
            <div node_ref=list class="op-tabs-list" role="tablist" aria-label=label>
                <span
                    class="op-tabs-indicator"
                    aria-hidden="true"
                    style:visibility=move || if indicator.get().is_some() { "visible" } else { "hidden" }
                    style:transform=move || indicator.get().map(|(l, _)| format!("translateX({l}px)"))
                    style:width=move || indicator.get().map(|(_, w)| format!("{w}px"))
                ></span>
                {tabs}
            </div>
        </div>
    }
}
