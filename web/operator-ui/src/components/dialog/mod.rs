use crate::components::icon_button::IconButton;
use crate::components::scrollbar::ScrollView;
use crate::core::du;
use leptos::prelude::*;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("dialog.css");

/// Modal window for a focused task: a form, details, or a choice.
///
/// Width follows the clamp model `clamp(min_width, 90% viewport, max_width)`.
#[component]
pub fn Dialog(
    open: RwSignal<bool>,
    #[prop(into)] title: Signal<String>,
    /// Buttons at the bottom, right-aligned.
    #[prop(optional, into)]
    actions: Option<ViewFn>,
    /// Closes on Escape, scrim click, and the close button.
    #[prop(default = true.into(), into)]
    dismissible: Signal<bool>,
    #[prop(default = 75.0)] min_width: f32,
    #[prop(default = 140.0)] max_width: f32,
    children: ChildrenFn,
) -> impl IntoView {
    let panel = NodeRef::<leptos::html::Div>::new();
    let children = StoredValue::new(children);
    let dismiss = move || {
        if dismissible.get_untracked() {
            open.set(false);
        }
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
    view! {
        <Show when=move || open.get()>
            <div
                class="op-dialog-scrim"
                on:click=move |_| dismiss()
                on:keydown=move |ev| {
                    if ev.key() == "Escape" {
                        dismiss();
                    }
                }
            >
                <div
                    node_ref=panel
                    class="op-dialog op-dialog-generic"
                    role="dialog"
                    aria-modal="true"
                    aria-label=move || title.get()
                    tabindex="-1"
                    style:width=format!(
                        "clamp(min({}, 100%), 90vw, {})",
                        du(min_width),
                        du(max_width),
                    )
                    on:click=|ev| ev.stop_propagation()
                >
                    <div class="op-dialog-head">
                        <div class="op-dialog-title">{move || title.get()}</div>
                        <Show when=move || dismissible.get()>
                            <IconButton icon=icons::CLOSE label="Close" on_click=Callback::new(move |()| open.set(false))/>
                        </Show>
                    </div>
                    <ScrollView class="op-dialog-body" viewport_class="op-dialog-content">{move || children.with_value(|c| c())}</ScrollView>
                    {actions.clone().map(|a| view! { <div class="op-dialog-actions">{a.run()}</div> })}
                </div>
            </div>
        </Show>
    }
}
