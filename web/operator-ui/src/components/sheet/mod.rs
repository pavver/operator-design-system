use crate::components::icon_button::IconButton;
use crate::components::scrollbar::ScrollView;
use crate::core::{Metric, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{Icon as IconData, icons};

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
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    #[prop(optional, into)] handle_width: MaybeProp<f32>,
    #[prop(optional, into)] handle_height: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] header_height: MaybeProp<f32>,
    #[prop(optional, into)] header_padding_start: MaybeProp<f32>,
    #[prop(optional, into)] title_font_size: MaybeProp<f32>,
    #[prop(optional, into)] title_line_height: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] padding: MaybeProp<f32>,
    #[prop(optional, into)] footer_padding: MaybeProp<f32>,
    #[prop(optional, into)] close_size: MaybeProp<f32>,
    #[prop(optional, into)] close_icon_size: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] actions: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] close_label: MaybeProp<String>,
    #[prop(optional)] close_icon: Option<IconData>,
    children: ChildrenFn,
) -> impl IntoView {
    let close_label = ui_text(close_label, "sheet.close", "Close");
    let close_icon = ui_icon(close_icon, "sheet.close", icons::CLOSE);
    let legacy_label = title.clone();
    let label = ui_text(
        Signal::derive(move || aria_label.get().or_else(|| legacy_label.clone())).into(),
        "sheet.label",
        "",
    );
    let header = StoredValue::new(header);
    let actions = StoredValue::new(actions);
    let footer = StoredValue::new(footer);
    let root_style = Signal::derive(root_style(
        vec![
            Metric::du("--op-sheet-size", size),
            Metric::du("--op-sheet-handle-width", handle_width),
            Metric::du("--op-sheet-handle-height", handle_height),
            Metric::du("--op-sheet-gap", gap),
            Metric::du("--op-sheet-header-height", header_height),
            Metric::du("--op-sheet-header-padding-start", header_padding_start),
            Metric::tu("--op-sheet-title-font-size", title_font_size),
            Metric::tu("--op-sheet-title-line-height", title_line_height),
            Metric::tu("--op-sheet-font-size", font_size),
            Metric::tu("--op-sheet-line-height", line_height),
            Metric::du("--op-sheet-padding", padding),
            Metric::du("--op-sheet-footer-padding", footer_padding),
            Metric::du("--op-sheet-close-size", close_size),
            Metric::du("--op-sheet-close-icon-size", close_icon_size),
        ],
        style,
    ));
    let children = StoredValue::new(children);
    let panel = NodeRef::<leptos::html::Div>::new();
    let side_name = match side {
        SheetSide::Left => "left",
        SheetSide::Right => "right",
        SheetSide::Bottom => "bottom",
    };
    crate::core::use_page_scroll_lock(open);
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
                class=root_class("op-sheet-scrim", class)
                style=move || root_style.get()
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
                    aria-label=move || label.get()
                    tabindex="-1"
                    data-side=side_name
                    on:click=|ev| ev.stop_propagation()
                >
                    {(side == SheetSide::Bottom).then(|| view! { <span class="op-sheet-handle" aria-hidden="true"></span> })}
                    {(title_view.is_some() || header.with_value(Slot::is_set)).then(|| {
                            view! {
                                <div class="op-sheet-head">
                                    <span class="op-sheet-title">{if header.with_value(Slot::is_set) { header.with_value(Slot::run).into_any() } else { title_view.clone().into_any() }}</span>
                                    {move || view! { <IconButton icon=close_icon.get() label=close_label on_click=Callback::new(move |()| open.set(false))/> }}
                                </div>
                            }
                        })}
                    <ScrollView class="op-sheet-body" viewport_class="op-scroll-contain">{move || children.with_value(|c| c())}</ScrollView>
                    {actions.with_value(Slot::is_set).then(|| view! { <div class="op-sheet-actions">{actions.with_value(Slot::run)}</div> })}
                    {footer.with_value(Slot::is_set).then(|| view! { <div class="op-sheet-footer">{footer.with_value(Slot::run)}</div> })}
                </div>
            </div>
        </Show>
    }
}
