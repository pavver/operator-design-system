//! Text input with suggestions: Combobox, and the pointer-anchored ComboboxPopup.

mod input;
mod list;
mod options;
mod popup;

pub use options::{
    ComboOption, ComboOptions, ComboRow, display_text, filter_rows, find_exact, option_at,
    option_count, row_of, step,
};
pub use popup::ComboboxPopup;

use crate::components::dropdown_menu::FieldListHandle;
use crate::components::dropdown_menu::{anchor_signal, field_popup_label, field_popup_style};
use crate::components::field::{Field, FieldMetrics, FieldVariant};
use crate::components::popover::{Placement, Popover};
use crate::components::primitives::Icon;
use crate::core::{
    ControlState, Metric, ScopedSlot, Slot, field_state, root_class, root_style, ui_icon, ui_text,
};
use input::ComboboxInput;
use leptos::prelude::*;
pub(crate) use list::ComboList;
use operator_ui_tokens::dim;
use operator_ui_tokens::{Icon as IconData, icons};

pub(crate) const CSS: &str = include_str!("combobox.css");

/// Text input with a filtered list of suggestions.
#[component]
pub fn Combobox(
    /// Choices: plain strings or [`ComboOption`]s, fixed or as a signal.
    #[prop(into)]
    options: ComboOptions,
    /// Committed value.
    selected: RwSignal<String>,
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
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(optional)] icon: Option<IconData>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] row_height: MaybeProp<f32>,
    #[prop(optional, into)] max_height: MaybeProp<f32>,
    #[prop(optional, into)] option_gap: MaybeProp<f32>,
    #[prop(optional, into)] option_padding_x: MaybeProp<f32>,
    #[prop(optional, into)] option_icon_size: MaybeProp<f32>,
    #[prop(optional, into)] detail_line_height: MaybeProp<f32>,
    #[prop(optional, into)] letter_spacing: MaybeProp<f32>,
    #[prop(optional, into)] detail_font_size: MaybeProp<f32>,
    #[prop(optional, into)] section_padding_y: MaybeProp<f32>,
    #[prop(optional)] option: Option<ScopedSlot<ComboOption>>,
    #[prop(optional, into)] empty: Slot,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    /// Accepts text that is not an option.
    #[prop(optional)]
    allow_custom: bool,
    /// Turns typed text into a value, or rejects it with `None`; implies `allow_custom`.
    #[prop(optional)]
    parse_custom: Option<Callback<String, Option<String>>>,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    /// Row shown when nothing matches.
    #[prop(optional, into)]
    empty_text: MaybeProp<String>,
    /// Row shown when nothing matches but the typed text can be used.
    #[prop(optional, into)]
    custom_hint: MaybeProp<String>,
) -> impl IntoView {
    let placeholder = ui_text(placeholder, "combobox.placeholder", "");
    let name = ui_text(aria_label, "combobox.label", "Choose a value");
    let accessible_name = Signal::derive(move || label.get().unwrap_or_else(|| name.get()));
    let empty_text = ui_text(empty_text, "combobox.empty", "No matches");
    let custom_hint = ui_text(
        custom_hint,
        "combobox.custom-hint",
        "Press Enter to use this value",
    );
    let icon = ui_icon(icon, "combobox.expand", icons::ARROW_DROP_DOWN);
    let mut metrics = FieldMetrics {
        height,
        padding_x,
        slot_size,
        gap,
        font_size,
    }
    .vars();
    metrics.extend([
        Metric::du("--op-field-icon-size", icon_size),
        Metric::du("--op-field-row-height", row_height),
        Metric::du("--op-field-max-height", max_height),
        Metric::du("--op-field-option-gap", option_gap),
        Metric::du("--op-field-option-padding-x", option_padding_x),
        Metric::tu("--op-field-detail-font-size", detail_font_size),
        Metric::tu("--op-field-detail-line-height", detail_line_height),
        Metric::tu("--op-field-letter-spacing", letter_spacing),
        Metric::du("--op-field-option-icon-size", option_icon_size),
        Metric::du("--op-field-section-padding-y", section_padding_y),
    ]);
    let custom_style = Signal::derive(root_style(metrics, style));
    let empty_slot = empty;
    let options = options.0;
    let custom = allow_custom || parse_custom.is_some();
    let shown = move || options.with_untracked(|o| display_text(o, &selected.get_untracked()));
    let query = RwSignal::new(shown());
    let open = RwSignal::new(false);
    let active = RwSignal::new(0usize);
    let focused = RwSignal::new(false);
    let current = field_state(enabled, state, focused);
    let blocked = move || {
        matches!(
            current.get(),
            ControlState::Pending | ControlState::Disabled
        )
    };

    // An unfocused field shows the committed value, also when the options arrive later.
    Effect::new(move |_| {
        let v = selected.get();
        let text = options.with(|o| display_text(o, &v));
        if !focused.get_untracked() {
            query.set(text);
        }
    });

    // The list shows every option when it opens; typing switches it to filtering.
    let filtering = RwSignal::new(false);
    let list = FieldListHandle::new();
    let anchor_ref = NodeRef::<leptos::html::Div>::new();
    let anchor = anchor_signal(anchor_ref);
    let popup_style = field_popup_style(anchor_ref, open, custom_style);
    let popup_ref = NodeRef::<leptos::html::Div>::new();
    field_popup_label(popup_ref, accessible_name);
    // A disabled field never keeps its list open.
    Effect::new(move |_| {
        if blocked() {
            open.set(false);
        }
    });

    let rows = Memo::new(move |_| {
        let q = if filtering.get() {
            query.get()
        } else {
            String::new()
        };
        options.with(|o| filter_rows(o, &q))
    });
    let highlight = Signal::derive(move || {
        if filtering.get() {
            query.get()
        } else {
            String::new()
        }
    });

    // Scrolls the list so the active option is visible; `center` places it in the middle.
    let reveal_active = move |center: bool| {
        if let Some(row) = rows.with_untracked(|r| row_of(r, active.get_untracked())) {
            list.reveal(row, center);
        }
    };

    // Opens the full list with the selected option active, or else the first option matching the text.
    let open_list = move || {
        if blocked() || open.get_untracked() {
            return;
        }
        filtering.set(false);
        let current = selected.get_untracked();
        let q = query.get_untracked();
        let pos = options.with_untracked(|o| find_initial_active(o, &current, &q));
        active.set(pos);
        open.set(true);
        reveal_active(true);
    };

    let commit_value = move |value: String| {
        if blocked() {
            return;
        }
        query.set(options.with_untracked(|o| display_text(o, &value)));
        selected.set(value.clone());
        open.set(false);
        filtering.set(false);
        if let Some(cb) = on_select {
            cb.run(value);
        }
    };
    let pick = Callback::new(move |o: ComboOption| commit_value(o.value));
    let settle = move || {
        if blocked() {
            return;
        }
        open.set(false);
        filtering.set(false);
        let q = query.get_untracked();
        let sh = shown();
        let action = options.with_untracked(|o| resolve_settle(o, &q, &sh, custom, parse_custom));
        match action {
            SettleAction::Unchanged => {}
            SettleAction::Commit(v) => commit_value(v),
            SettleAction::ResetToShown => query.set(sh),
        }
    };
    let empty = Signal::derive(move || {
        Some(if custom && !query.with(String::is_empty) {
            custom_hint.get()
        } else {
            empty_text.get()
        })
    });

    let append = Slot::from(move || {
        view! {
            {append.run()}
            <span class="op-field-reveal op-combobox-arrow" aria-hidden="true">
                {move || view! { <Icon icon=icon.get() size=dim::ICON_FIELD attr:style="width:var(--op-field-icon-size, var(--op-dim-icon-field));height:var(--op-field-icon-size, var(--op-dim-icon-field));"/> }}
            </span>
        }
    });
    view! {
        <div class=root_class("op-dropdown op-combobox", class) style=move || custom_style.get()>
            <div class="op-dropdown-anchor" node_ref=anchor_ref>
                <Field state=current label=label hint=hint error=error variant=variant
                    prepend=prepend append=append before=before after=after label_slot=label_slot hint_slot=hint_slot
                    trailing=true pending_run=true
                >
                    <ComboboxInput
                        query=query
                        open=open
                        active=active
                        filtering=filtering
                        focused=focused
                        current=current
                        error=error
                        placeholder=placeholder
                        accessible_name=accessible_name
                        rows=rows
                        on_open_list=Callback::new(move |()| open_list())
                        on_reveal_active=Callback::new(reveal_active)
                        on_commit_value=Callback::new(commit_value)
                        on_settle=Callback::new(move |()| settle())
                        shown_text=Callback::new(move |()| shown())
                        blocked=Signal::derive(blocked)
                    />
                </Field>
                <Popover dismiss_on_scroll=true node_ref=popup_ref open=open anchor=anchor placement=Placement::BelowStretch class="op-dropdown-menu" role="listbox" restore_focus=false>
                    <ComboList
                        rows=rows
                        handle=list
                        active=active
                        selected=selected
                        highlight=highlight
                        empty=empty empty_slot=empty_slot.clone() option=option
                        header=header.clone() footer=footer.clone()
                        before_options=before_options.clone() after_options=after_options.clone()
                        style=popup_style
                        on_pick=pick
                    />
                </Popover>
            </div>
        </div>
    }
}

fn find_initial_active(options: &[ComboOption], current: &str, query: &str) -> usize {
    let q = query.to_lowercase();
    options
        .iter()
        .position(|s| !current.is_empty() && s.value == current)
        .or_else(|| {
            (!q.is_empty())
                .then(|| options.iter().position(|s| s.matches(&q)))
                .flatten()
        })
        .unwrap_or(0)
}

enum SettleAction {
    Unchanged,
    Commit(String),
    ResetToShown,
}

fn resolve_settle(
    options: &[ComboOption],
    query: &str,
    shown: &str,
    custom: bool,
    parse_custom: Option<Callback<String, Option<String>>>,
) -> SettleAction {
    if query == shown {
        return SettleAction::Unchanged;
    }
    if let Some(o) = find_exact(options, query) {
        return SettleAction::Commit(o.value);
    }
    let parsed = if !custom || query.is_empty() {
        None
    } else if let Some(parse) = parse_custom {
        parse.run(query.to_string())
    } else {
        Some(query.to_string())
    };
    match parsed {
        Some(v) => SettleAction::Commit(v),
        None => SettleAction::ResetToShown,
    }
}

#[cfg(test)]
mod tests;
