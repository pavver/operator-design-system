//! ComboboxPopup: a search field and an option list opened at a point (quick add, wire menus).

use super::list::ComboList;
use super::options::{
    ComboOption, ComboOptions, filter_rows, option_at, option_count, row_of, step,
};
use crate::components::dropdown_menu::{FieldListHandle, field_popup_label};
use crate::components::field::{Field, FieldMetrics, FieldVariant};
use crate::components::popover::{Placement, Popover, PopoverAnchor};
use crate::core::{ControlState, Metric, ScopedSlot, Slot, root_class, root_style, ui_text};
use leptos::prelude::*;

/// Search field with a filtered option list in a floating layer, without a field on the page.
///
/// Typing filters, arrows move the active option, Enter or a click picks it and closes the layer.
/// Enter with no matching option passes the typed text to `on_custom` when it is set.
#[component]
pub fn ComboboxPopup(
    open: RwSignal<bool>,
    #[prop(into)] anchor: Signal<PopoverAnchor>,
    #[prop(into)] options: ComboOptions,
    on_select: Callback<ComboOption>,
    #[prop(optional)] on_custom: Option<Callback<String>>,
    #[prop(default = Placement::AtPoint)] placement: Placement,
    /// Layer width in DU.
    #[prop(default = 80.0)]
    width: f32,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] hint: MaybeProp<String>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] variant: Signal<FieldVariant>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    #[prop(optional, into)] prepend: Slot,
    #[prop(optional, into)] append: Slot,
    #[prop(optional, into)] before: Slot,
    #[prop(optional, into)] after: Slot,
    #[prop(optional, into)] label_slot: Slot,
    #[prop(optional, into)] hint_slot: Slot,
    #[prop(optional, into)] height: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] slot_size: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,

    #[prop(optional, into)] empty_text: MaybeProp<String>,
    #[prop(optional, into)] custom_hint: MaybeProp<String>,
    #[prop(optional, into)] row_height: MaybeProp<f32>,
    #[prop(optional, into)] max_height: MaybeProp<f32>,
    #[prop(optional, into)] option_gap: MaybeProp<f32>,
    #[prop(optional, into)] option_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] option_icon_size: MaybeProp<f32>,
    #[prop(optional, into)] detail_line_height: MaybeProp<f32>,
    #[prop(optional, into)] letter_spacing: MaybeProp<f32>,
    #[prop(optional, into)] detail_font_size: MaybeProp<f32>,
    #[prop(optional, into)] section_padding_y: MaybeProp<f32>,
    #[prop(optional, into)] screen_margin: MaybeProp<f32>,
    #[prop(optional)] option: Option<ScopedSlot<ComboOption>>,
    #[prop(optional, into)] empty: Slot,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
) -> impl IntoView {
    let placeholder = ui_text(placeholder, "combobox-popup.placeholder", "");
    let name = ui_text(aria_label, "combobox-popup.label", "Search");
    let accessible_name = Signal::derive(move || label.get().unwrap_or_else(|| name.get()));
    let empty_text = ui_text(empty_text, "combobox.empty", "No matches");
    let custom_hint = ui_text(
        custom_hint,
        "combobox.custom-hint",
        "Press Enter to use this value",
    );
    let empty_slot = empty;
    let mut metrics = FieldMetrics {
        height,
        padding_x,
        slot_size,
        gap,
        font_size,
    }
    .vars();
    metrics.extend([
        Metric::du("--op-field-popup-width", width.into()),
        Metric::du("--op-field-row-height", row_height),
        Metric::du("--op-field-max-height", max_height),
        Metric::du("--op-field-option-gap", option_gap),
        Metric::du("--op-field-option-padding-x", option_padding_x),
        Metric::tu("--op-field-detail-font-size", detail_font_size),
        Metric::tu("--op-field-detail-line-height", detail_line_height),
        Metric::tu("--op-field-letter-spacing", letter_spacing),
        Metric::du("--op-field-option-icon-size", option_icon_size),
        Metric::du("--op-field-section-padding-y", section_padding_y),
        Metric::du("--op-field-screen-margin", screen_margin),
    ]);
    let custom_style = Signal::derive(root_style(metrics, style));
    let options = options.0;
    let query = RwSignal::new(String::new());
    let popup_ref = NodeRef::<leptos::html::Div>::new();
    field_popup_label(popup_ref, accessible_name);
    let active = RwSignal::new(0usize);
    let input = NodeRef::<leptos::html::Input>::new();
    let list = FieldListHandle::new();

    // Every opening starts with an empty search.
    Effect::new(move |_| {
        if open.get() {
            query.set(String::new());
            active.set(0);
        }
    });
    let rows = Memo::new(move |_| options.with(|o| filter_rows(o, &query.get())));
    let reveal_active = move || {
        if let Some(row) = rows.with_untracked(|r| row_of(r, active.get_untracked())) {
            list.reveal(row, false);
        }
    };
    let pick = Callback::new(move |o: ComboOption| {
        open.set(false);
        on_select.run(o);
    });
    let focus_input = Callback::new(move |()| {
        if let Some(el) = input.get_untracked() {
            let _ = el.focus();
        }
    });
    let empty = Signal::derive(move || {
        Some(if on_custom.is_some() && !query.with(String::is_empty) {
            custom_hint.get()
        } else {
            empty_text.get()
        })
    });

    view! {
        <Popover node_ref=popup_ref
            open=open
            anchor=anchor
            placement=placement
            class="op-dropdown-menu op-combo-popup"
            role="dialog"
            label=accessible_name.get()
            on_placed=focus_input
        >
            <div class=root_class("op-combo-popup-body", class) style=move || custom_style.get()>
                <Field state=ControlState::Focused label=label hint=hint error=error variant=variant
                    prepend=prepend.clone() append=append.clone() before=before.clone() after=after.clone() label_slot=label_slot.clone() hint_slot=hint_slot.clone()
                    box_class="op-combo-popup-field"
                >
                <input
                    node_ref=input
                    class="op-field-input"
                    role="combobox"
                    aria-label=move || accessible_name.get()
                    aria-expanded="true"
                    aria-autocomplete="list"
                    placeholder=move || placeholder.get()
                    prop:value=move || query.get()
                    on:input=move |ev| {
                        query.set(event_target_value(&ev));
                        active.set(0);
                        reveal_active();
                    }
                    on:keydown=move |ev| {
                        let n = rows.with_untracked(|r| option_count(r));
                        match ev.key().as_str() {
                            "ArrowDown" => {
                                ev.prevent_default();
                                active.update(|a| *a = step(*a, n, 1));
                                reveal_active();
                            }
                            "ArrowUp" => {
                                ev.prevent_default();
                                active.update(|a| *a = step(*a, n, -1));
                                reveal_active();
                            }
                            "Enter" => {
                                ev.prevent_default();
                                match rows.with_untracked(|r| option_at(r, active.get_untracked())) {
                                    Some(o) => pick.run(o),
                                    None => {
                                        let q = query.get_untracked();
                                        if let Some(cb) = on_custom.filter(|_| !q.is_empty()) {
                                            open.set(false);
                                            cb.run(q);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                />
                </Field>
                <ComboList
                    rows=rows
                    handle=list
                    active=active
                    selected=Signal::stored(String::new())
                    highlight=query
                    empty=empty empty_slot=empty_slot.clone() option=option
                    header=header.clone() footer=footer.clone()
                    before_options=before_options.clone() after_options=after_options.clone()
                    on_pick=pick
                />
            </div>
        </Popover>
    }
}
