use crate::components::icon_button::IconButton;
use crate::components::scrollbar::ScrollView;
use crate::core::du;
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("sheet.css");

/// Edge a [`Sheet`] slides in from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetSide {
    #[default]
    Left,
    Right,
    Bottom,
}

/// Panel that slides in from a screen edge over a scrim: navigation drawers and bottom sheets.
#[component]
pub fn Sheet(
    open: RwSignal<bool>,
    #[prop(optional)] side: SheetSide,
    #[prop(optional, into)] title: Option<String>,
    /// Width for side sheets or maximum height for bottom sheets, in DU.
    #[prop(default = 75.0)]
    size: f32,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let panel = NodeRef::<leptos::html::Div>::new();
    let side_name = match side {
        SheetSide::Left => "left",
        SheetSide::Right => "right",
        SheetSide::Bottom => "bottom",
    };
    Effect::new(move |_| {
        if open.get() {
            request_animation_frame(move || {
                if let Some(p) = panel.try_get_untracked().flatten() {
                    let _ = p.focus();
                }
            });
        }
    });
    let title_view = title.clone();
    view! {
        <Show when=move || open.get()>
            <div
                class="op-sheet-scrim"
                on:click=move |_| open.set(false)
                on:keydown=move |ev| {
                    if ev.key() == "Escape" {
                        open.set(false);
                    }
                }
            >
                <div
                    node_ref=panel
                    class="op-sheet"
                    role="dialog"
                    aria-modal="true"
                    aria-label=title.clone()
                    tabindex="-1"
                    data-side=side_name
                    style:--sheet-size=du(size)
                    on:click=|ev| ev.stop_propagation()
                >
                    {(side == SheetSide::Bottom).then(|| view! { <span class="op-sheet-handle" aria-hidden="true"></span> })}
                    {title_view
                        .clone()
                        .map(|t| {
                            view! {
                                <div class="op-sheet-head">
                                    <span class="op-sheet-title">{t}</span>
                                    <IconButton icon=icons::CLOSE label="Close" on_click=Callback::new(move |()| open.set(false))/>
                                </div>
                            }
                        })}
                    <ScrollView class="op-sheet-body">{move || children.with_value(|c| c())}</ScrollView>
                </div>
            </div>
        </Show>
    }
}
