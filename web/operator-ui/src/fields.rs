//! Text and choice inputs: TextArea, SearchField, RadioGroup, Combobox, Chip.

use crate::basic::{Icon, PendingIndicator, ProgressIndicator};
use crate::core::{
    AsyncAction, ControlState, JobToken, PendingRun, Press, du, sleep_ms, spawn_ripple,
    use_pending_indicator, use_state_visual,
};
use crate::overlay::Tooltip;
use crate::scroll::{ScrollAxis, ScrollView, Scrollbar};
use crate::toggles::GroupItem;
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, icons, motion};
use std::time::Duration;
use wasm_bindgen::JsCast;

fn field_state(
    enabled: Signal<bool>,
    state: Signal<Option<ControlState>>,
    focused: RwSignal<bool>,
) -> Signal<ControlState> {
    Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if let Some(s) = state.get() {
            s
        } else if focused.get() {
            ControlState::Focused
        } else {
            ControlState::Idle
        }
    })
}

// ---------------------------------------------------------------------------
// TextArea
// ---------------------------------------------------------------------------

/// Multi-line text input that grows with its content between `min_rows` and `max_rows`.
///
/// The user can also drag the corner grip to set the height between `min_rows` and
/// `resize_max_rows`; a double-click on the grip returns to content-driven height.
#[component]
pub fn TextArea(
    value: RwSignal<String>,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(default = 3)] min_rows: u32,
    #[prop(default = 8)] max_rows: u32,
    /// Shows the corner grip for manual resizing.
    #[prop(default = true)]
    resizable: bool,
    /// Largest height, in lines, reachable by manual resizing.
    #[prop(default = 20)]
    resize_max_rows: u32,
    /// Hard character limit with a counter under the field.
    #[prop(optional)]
    max_length: Option<usize>,
    #[prop(optional, into)] state: Signal<Option<ControlState>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
) -> impl IntoView {
    let area = NodeRef::<leptos::html::Textarea>::new();
    let focused = RwSignal::new(false);
    let visual = use_state_visual(field_state(enabled, state, focused));
    let max_rows = max_rows.max(min_rows);
    let resize_max_rows = resize_max_rows.max(max_rows);
    // Height set by the grip in CSS pixels; `None` means content-driven height.
    let manual = RwSignal::new(None::<f64>);
    let resizing = RwSignal::new(false);
    // Pointer y and textarea height at the start of a grip drag.
    let drag_from = StoredValue::new((0.0f64, 0.0f64));

    let fit = move || {
        let Some(el) = area.try_get_untracked().flatten() else {
            return;
        };
        let style = web_sys::HtmlElement::style(&el);
        if let Some(h) = manual.get_untracked() {
            let _ = style.set_property("height", &format!("{h}px"));
            return;
        }
        let _ = style.set_property("height", "auto");
        let _ = style.set_property("height", &format!("{}px", el.scroll_height()));
    };
    Effect::new(move |_| {
        value.track();
        manual.track();
        request_animation_frame(fit);
    });

    // Height limits of manual resizing, measured from the rendered line height and padding.
    let limits = move || -> Option<(f64, f64)> {
        let el = area.try_get_untracked().flatten()?;
        let style = web_sys::window()?.get_computed_style(&el).ok().flatten()?;
        let px = |name: &str| {
            style
                .get_property_value(name)
                .ok()
                .and_then(|v| v.trim_end_matches("px").parse::<f64>().ok())
                .unwrap_or(0.0)
        };
        let line = px("line-height");
        let pad = px("padding-top") + px("padding-bottom");
        Some((
            line * f64::from(min_rows) + pad,
            line * f64::from(resize_max_rows) + pad,
        ))
    };
    let on_grip_down = move |ev: web_sys::PointerEvent| {
        if ev.button() != 0 || !enabled.get_untracked() {
            return;
        }
        ev.prevent_default();
        let Some(el) = area.try_get_untracked().flatten() else {
            return;
        };
        if let Some(t) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            let _ = t.set_pointer_capture(ev.pointer_id());
        }
        drag_from.set_value((
            f64::from(ev.client_y()),
            el.get_bounding_client_rect().height(),
        ));
        resizing.set(true);
    };
    let on_grip_move = move |ev: web_sys::PointerEvent| {
        if !resizing.get_untracked() {
            return;
        }
        let Some((min, max)) = limits() else {
            return;
        };
        let (start_y, start_h) = drag_from.get_value();
        let h = (start_h + f64::from(ev.client_y()) - start_y).clamp(min, max);
        manual.set(Some(h.round()));
    };
    let on_grip_up = move |_| resizing.set(false);

    let count = move || value.with(|v| v.chars().count());
    let counter_tone = move || {
        max_length.map(|max| {
            if count() >= max {
                "error"
            } else if count() * 10 >= max * 9 {
                "warning"
            } else {
                "normal"
            }
        })
    };

    view! {
        <div class="op-textarea-wrap">
            <div
                class="op-sb op-textarea"
                data-state=move || visual.state.get()
                data-result=move || visual.result.get()
                data-fading=move || visual.fading.get().then_some("")
                style:--ta-min-rows=min_rows.to_string()
                style:--ta-max-rows=max_rows.to_string()
                style:--ta-resize-max-rows=resize_max_rows.to_string()
                data-manual=move || manual.get().is_some().then_some("")
                data-resizing=move || resizing.get().then_some("")
            >
                <textarea
                    node_ref=area
                    class="op-textarea-input op-scroll-native-off"
                    rows=min_rows
                    aria-label=label
                    placeholder=move || placeholder.get()
                    maxlength=max_length.map(|m| m.to_string())
                    prop:value=move || value.get()
                    disabled=move || !enabled.get()
                    on:input=move |ev| value.set(event_target_value(&ev))
                    on:focus=move |_| focused.set(true)
                    on:blur=move |_| focused.set(false)
                ></textarea>
                <Scrollbar target=area axis=ScrollAxis::Vertical corner=Signal::stored(resizable)/>
                {resizable
                    .then(|| {
                        view! {
                            <span class="op-textarea-grip-slot">
                            <Tooltip text="Drag to resize, double-click to fit the text">
                            <span
                                class="op-textarea-grip"
                                aria-hidden="true"
                                on:pointerdown=on_grip_down
                                on:pointermove=on_grip_move
                                on:pointerup=on_grip_up
                                on:pointercancel=on_grip_up
                                on:lostpointercapture=on_grip_up
                                on:dblclick=move |_| manual.set(None)
                            ></span>
                            </Tooltip>
                            </span>
                        }
                    })}
                <PendingRun/>
            </div>
            {max_length
                .map(|max| {
                    view! {
                        <div class="op-textarea-counter op-t-caption" data-tone=counter_tone>
                            {move || format!("{} / {max}", count())}
                        </div>
                    }
                })}
        </div>
    }
}

// ---------------------------------------------------------------------------
// SearchField
// ---------------------------------------------------------------------------

/// Search input with a clear button; reports the query after a short pause in typing.
#[component]
pub fn SearchField(
    value: RwSignal<String>,
    #[prop(default = "Search".into(), into)] placeholder: Signal<String>,
    /// Called with the query after `debounce_ms` without typing, on Enter, and on clear.
    #[prop(optional, into)]
    on_search: Option<Callback<String>>,
    #[prop(default = motion::SEARCH_DEBOUNCE_MS)] debounce_ms: u32,
    /// Shows a spinner in place of the search icon while results load.
    #[prop(optional, into)]
    pending: Signal<bool>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(default = "Search".into(), into)] label: String,
) -> impl IntoView {
    let input = NodeRef::<leptos::html::Input>::new();
    let focused = RwSignal::new(false);
    let visual = use_state_visual(field_state(enabled, Signal::stored(None), focused));
    let job = JobToken::new();

    let emit_now = move || {
        job.next();
        if let Some(cb) = on_search {
            cb.run(value.get_untracked());
        }
    };
    let emit_later = move || {
        let id = job.next();
        set_timeout(
            move || {
                if let Some(cb) = on_search.filter(|_| job.is_current(id)) {
                    cb.run(value.get_untracked());
                }
            },
            Duration::from_millis(u64::from(debounce_ms)),
        );
    };
    let clear = move || {
        value.set(String::new());
        emit_now();
        if let Some(el) = input.try_get_untracked().flatten() {
            let _ = el.focus();
        }
    };

    view! {
        <div
            class="op-sb op-field op-search"
            role="search"
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
            data-trailing=move || value.with(|v| !v.is_empty()).then_some("")
        >
            <span class="op-search-lead">
                {move || {
                    if pending.get() {
                        view! { <ProgressIndicator size=dim::ICON_MEDIUM stroke=dim::SPINNER_STROKE color="var(--op-pending)"/> }
                            .into_any()
                    } else {
                        view! { <Icon icon=icons::SEARCH size=dim::ICON_LARGE/> }.into_any()
                    }
                }}
            </span>
            <input
                node_ref=input
                class="op-field-input"
                type="search"
                aria-label=label
                placeholder=move || placeholder.get()
                prop:value=move || value.get()
                disabled=move || !enabled.get()
                on:input=move |ev| {
                    value.set(event_target_value(&ev));
                    emit_later();
                }
                on:keydown=move |ev| match ev.key().as_str() {
                    "Enter" => emit_now(),
                    "Escape" if !value.with_untracked(String::is_empty) => {
                        ev.prevent_default();
                        clear();
                    }
                    _ => {}
                }
                on:focus=move |_| focused.set(true)
                on:blur=move |_| focused.set(false)
            />
            <Show when=move || value.with(|v| !v.is_empty())>
                <button
                    type="button"
                    class="op-rippled op-field-reveal"
                    aria-label="Clear search"
                    disabled=move || !enabled.get()
                    on:mousedown=|ev| ev.prevent_default()
                    on:pointerdown=move |ev| spawn_ripple(&ev)
                    on:click=move |_| clear()
                >
                    <Icon icon=icons::CLOSE size=dim::ICON_SMALL/>
                </button>
            </Show>
            <PendingRun/>
        </div>
    }
}

// ---------------------------------------------------------------------------
// RadioGroup
// ---------------------------------------------------------------------------

/// Single choice from a short list; with `on_change` the selection commits only after success.
#[component]
pub fn RadioGroup(
    items: Vec<GroupItem>,
    selected: RwSignal<String>,
    /// Async commit; without it the selection changes immediately.
    #[prop(optional, into)]
    on_change: Option<AsyncAction<String>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
    #[prop(optional, into)] label: Option<String>,
    /// Lays the options out in a row.
    #[prop(optional)]
    horizontal: bool,
) -> impl IntoView {
    let target = RwSignal::new(None::<String>);
    let result = RwSignal::new(ControlState::Idle);
    let busy = move || target.with(Option::is_some);
    let on_change = StoredValue::new(on_change);
    let ids: Vec<String> = items.iter().map(|i| i.id.clone()).collect();
    let ids = StoredValue::new(ids);

    let choose = move |id: String| {
        if busy() || !enabled.get_untracked() || selected.with_untracked(|s| *s == id) {
            return;
        }
        let Some(action) = on_change.get_value() else {
            selected.set(id);
            return;
        };
        target.set(Some(id.clone()));
        result.set(ControlState::Pending);
        spawn_local(async move {
            let r = action.call(id.clone()).await;
            let ok = r.is_success();
            if ok {
                selected.set(id);
            }
            result.set(r.into());
            sleep_ms(motion::RESULT_HOLD_MS).await;
            result.set(ControlState::Idle);
            target.set(None);
        });
    };

    let move_by = move |from: &str, delta: i32| {
        let list = ids.get_value();
        let Some(i) = list.iter().position(|x| x == from) else {
            return;
        };
        let n = i32::try_from(list.len()).unwrap_or(1);
        let next = (i32::try_from(i).unwrap_or(0) + delta).rem_euclid(n.max(1));
        if let Some(id) = usize::try_from(next).ok().and_then(|n| list.get(n)) {
            choose(id.clone());
            focus_radio(id);
        }
    };

    let rows = items
        .into_iter()
        .map(|item| {
            let id = item.id.clone();
            let press = Press::new();
            let is_selected = {
                let id = id.clone();
                Signal::derive(move || selected.with(|s| *s == id))
            };
            let is_target = {
                let id = id.clone();
                Signal::derive(move || target.with(|t| t.as_deref() == Some(id.as_str())))
            };
            let show_spinner = use_pending_indicator(move || {
                is_target.get() && result.get() == ControlState::Pending
            });
            let mark_state = Signal::derive(move || {
                if !enabled.get() {
                    ControlState::Disabled
                } else if is_target.get() {
                    result.get()
                } else if press.pressed.get() {
                    ControlState::Pressed
                } else if is_selected.get() {
                    ControlState::Selected
                } else {
                    ControlState::Idle
                }
            });
            let visual = use_state_visual(mark_state);
            let click_id = id.clone();
            let key_id = id.clone();
            let is_first = ids.with_value(|l| l.first() == Some(&id));
            let tab_stop = move || {
                is_selected.get()
                    || (is_first && !selected.with(|s| ids.with_value(|l| l.contains(s))))
            };
            view! {
                <button
                    type="button"
                    role="radio"
                    class="op-rippled op-radio"
                    data-radio-id=id.clone()
                    aria-checked=move || is_selected.get().to_string()
                    aria-disabled=move || (!enabled.get()).then_some("true")
                    tabindex=move || if tab_stop() { "0" } else { "-1" }
                    data-checked=move || is_selected.get().then_some("")
                    on:pointerdown=move |ev| {
                        if enabled.get_untracked() && !busy() {
                            press.down();
                            spawn_ripple(&ev);
                        }
                    }
                    on:pointerup=move |_| press.up()
                    on:pointerleave=move |_| press.up()
                    on:pointercancel=move |_| press.up()
                    on:click=move |_| choose(click_id.clone())
                    on:keydown=move |ev| match ev.key().as_str() {
                        "ArrowDown" | "ArrowRight" => {
                            ev.prevent_default();
                            move_by(&key_id, 1);
                        }
                        "ArrowUp" | "ArrowLeft" => {
                            ev.prevent_default();
                            move_by(&key_id, -1);
                        }
                        _ => {}
                    }
                >
                    <span
                        class="op-sb op-radio-box"
                        data-state=move || visual.state.get()
                        data-result=move || visual.result.get()
                        data-fading=move || visual.fading.get().then_some("")
                    >
                        {move || {
                            if show_spinner.get() {
                                view! { <PendingIndicator size=3.0/> }.into_any()
                            } else {
                                view! { <span class="op-radio-dot"></span> }.into_any()
                            }
                        }}
                        <PendingRun/>
                    </span>
                    <span class="op-radio-label">{item.name}</span>
                </button>
            }
        })
        .collect_view();

    view! {
        <div
            class="op-radio-group"
            role="radiogroup"
            aria-label=label
            data-horizontal=horizontal.then_some("")
        >
            {rows}
        </div>
    }
}

fn focus_radio(id: &str) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let selector = format!("[data-radio-id=\"{}\"]", id.replace('"', "\\\""));
    if let Some(el) = doc
        .query_selector(&selector)
        .ok()
        .flatten()
        .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
    {
        let _ = el.focus();
    }
}

// ---------------------------------------------------------------------------
// Combobox
// ---------------------------------------------------------------------------

/// Text input with a filtered list of suggestions.
#[component]
pub fn Combobox(
    options: Vec<String>,
    /// Committed value.
    selected: RwSignal<String>,
    #[prop(optional, into)] placeholder: Signal<String>,
    #[prop(optional, into)] label: Option<String>,
    /// Accepts text that is not in `options`.
    #[prop(optional)]
    allow_custom: bool,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
) -> impl IntoView {
    let options = StoredValue::new(options);
    let query = RwSignal::new(selected.get_untracked());
    let open = RwSignal::new(false);
    let active = RwSignal::new(0usize);
    let focused = RwSignal::new(false);
    let visual = use_state_visual(field_state(enabled, Signal::stored(None), focused));

    Effect::new(move |_| {
        let v = selected.get();
        if !focused.get_untracked() {
            query.set(v);
        }
    });

    // The list shows every option when it opens; typing switches it to filtering.
    let filtering = RwSignal::new(false);
    let list_ref = NodeRef::<leptos::html::Div>::new();

    let filtered = Memo::new(move |_| {
        let q = if filtering.get() {
            query.get().to_lowercase()
        } else {
            String::new()
        };
        options.with_value(|o| {
            o.iter()
                .filter(|s| q.is_empty() || s.to_lowercase().contains(&q))
                .cloned()
                .collect::<Vec<_>>()
        })
    });

    // Scrolls the list so the active option is visible; `center` places it in the middle.
    let reveal_active = move |center: bool| {
        request_animation_frame(move || {
            let Some(view) = list_ref.get_untracked() else {
                return;
            };
            let Some(item) = view
                .query_selector(".op-dropdown-item[data-active]")
                .ok()
                .flatten()
            else {
                return;
            };
            let v = view.get_bounding_client_rect();
            let r = item.get_bounding_client_rect();
            let top = r.top() - v.top() + f64::from(view.scroll_top());
            let bottom = top + r.height();
            let height = f64::from(view.client_height());
            let current = f64::from(view.scroll_top());
            let next = if center {
                top - (height - r.height()) / 2.0
            } else if top < current {
                top
            } else if bottom > current + height {
                bottom - height
            } else {
                return;
            };
            #[allow(clippy::cast_possible_truncation)]
            view.set_scroll_top(next.max(0.0).round() as i32);
        });
    };

    // Opens the full list with the selected option active, or else the first option matching the text.
    let open_list = move || {
        if open.get_untracked() {
            return;
        }
        filtering.set(false);
        let current = selected.get_untracked();
        let q = query.get_untracked().to_lowercase();
        let index = options.with_value(|o| {
            o.iter()
                .position(|s| !current.is_empty() && *s == current)
                .or_else(|| {
                    (!q.is_empty())
                        .then(|| o.iter().position(|s| s.to_lowercase().contains(&q)))
                        .flatten()
                })
                .unwrap_or(0)
        });
        active.set(index);
        open.set(true);
        reveal_active(true);
    };

    let commit = move |v: String| {
        query.set(v.clone());
        selected.set(v.clone());
        open.set(false);
        filtering.set(false);
        if let Some(cb) = on_select {
            cb.run(v);
        }
    };
    let settle = move || {
        open.set(false);
        filtering.set(false);
        let q = query.get_untracked();
        let known = options.with_value(|o| o.contains(&q));
        if q == selected.get_untracked() {
            return;
        }
        if known || (allow_custom && !q.is_empty()) {
            commit(q);
        } else {
            query.set(selected.get_untracked());
        }
    };

    view! {
            <div class="op-dropdown op-combobox">
                {label.clone().map(|l| view! { <div class="op-dropdown-label">{l}</div> })}
                <div class="op-dropdown-anchor">
                    <div
                        class="op-sb op-field"
                        data-state=move || visual.state.get()
                        data-result=move || visual.result.get()
                        data-fading=move || visual.fading.get().then_some("")
                        data-trailing=""
                    >
                        <input
                            class="op-field-input"
                            role="combobox"
                            aria-label=label
                            aria-expanded=move || open.get().to_string()
                            aria-autocomplete="list"
                            placeholder=move || placeholder.get()
                            prop:value=move || query.get()
                            disabled=move || !enabled.get()
                            on:input=move |ev| {
                                query.set(event_target_value(&ev));
                                filtering.set(true);
                                active.set(0);
                                open.set(true);
                                reveal_active(false);
                            }
                            on:focus=move |_| {
                                focused.set(true);
                                open_list();
                            }
                            on:click=move |_| open_list()
                            on:blur=move |_| {
                                focused.set(false);
                                settle();
                            }
                            on:keydown=move |ev| {
                                let n = filtered.with_untracked(Vec::len);
                                match ev.key().as_str() {
                                    "ArrowDown" => {
                                        ev.prevent_default();
                                        if !open.get_untracked() {
                                            open_list();
                                        } else if n > 0 {
                                            active.update(|a| *a = (*a + 1) % n);
                                            reveal_active(false);
                                        }
                                    }
                                    "ArrowUp" => {
                                        ev.prevent_default();
                                        if n > 0 && open.get_untracked() {
                                            active.update(|a| *a = (*a + n - 1) % n);
                                            reveal_active(false);
                                        }
                                    }
                                    "Enter" => {
                                        ev.prevent_default();
                                        let pick = filtered.with_untracked(|f| f.get(active.get_untracked()).cloned());
                                        match pick {
                                            Some(v) if open.get_untracked() => commit(v),
                                            _ => settle(),
                                        }
                                    }
                                    "Escape" => {
                                        open.set(false);
                                        filtering.set(false);
                                        query.set(selected.get_untracked());
                                    }
                                    _ => {}
                                }
                            }
                        />
                        <span class="op-field-reveal op-combobox-arrow" aria-hidden="true">
                            <Icon icon=icons::ARROW_DROP_DOWN size=5.0/>
                        </span>
                        <PendingRun/>
                    </div>
                    <Show when=move || open.get() && enabled.get()>
                        <div class="op-dropdown-menu" role="listbox" on:mousedown=|ev| ev.prevent_default()>
                            <ScrollView node_ref=list_ref>
                            {move || {
                                let list = filtered.get();
                                if list.is_empty() {
                                    let text = if allow_custom { "Press Enter to use this value" } else { "No matches" };
                                    return view! { <div class="op-dropdown-item op-combobox-none">{text}</div> }.into_any();
                                }
                                let q = if filtering.get() { query.get() } else { String::new() };
                                list.into_iter()
                                    .enumerate()
                                    .map(|(i, option)| {
                                        let value = option.clone();
                                        let is_selected = selected.with(|s| *s == option);
                                        view! {
                                            <button
                                                type="button"
                                                role="option"
                                                tabindex="-1"
                                                class="op-dropdown-item"
                                                aria-selected=is_selected.to_string()
                                                data-active=move || (active.get() == i).then_some("")
                                                on:pointerenter=move |_| active.set(i)
                                                on:click=move |_| commit(value.clone())
                                            >
                                                {highlight(&option, &q)}
                                            </button>
                                        }
                                    })
                                    .collect_view()
                                    .into_any()
                            }}
    </ScrollView>
                        </div>
                    </Show>
                </div>
            </div>
        }
}

/// Splits `text` around the first case-insensitive match of `query` and marks it.
fn highlight(text: &str, query: &str) -> AnyView {
    let lower = text.to_lowercase();
    let q = query.to_lowercase();
    match (!q.is_empty())
        .then(|| lower.find(&q))
        .flatten()
        .filter(|_| lower.len() == text.len())
    {
        Some(start) => {
            let end = start + q.len();
            let (a, rest) = text.split_at(start);
            let (m, b) = rest.split_at(end - start);
            view! { <span>{a.to_string()}<mark class="op-match">{m.to_string()}</mark>{b.to_string()}</span> }
                .into_any()
        }
        None => view! { <span>{text.to_string()}</span> }.into_any(),
    }
}

// ---------------------------------------------------------------------------
// Chip
// ---------------------------------------------------------------------------

/// Compact token: a toggleable filter, a removable value, or a static tag.
#[component]
pub fn Chip(
    #[prop(into)] text: Signal<String>,
    #[prop(optional)] icon: Option<IconData>,
    /// Makes the chip a toggle (filter chip).
    #[prop(optional, into)]
    selected: Option<RwSignal<bool>>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    /// Shows a remove button (input chip).
    #[prop(optional, into)]
    on_remove: Option<Callback<()>>,
    #[prop(default = true.into(), into)] enabled: Signal<bool>,
) -> impl IntoView {
    let press = Press::new();
    let interactive = selected.is_some() || on_click.is_some();
    let is_selected = move || selected.is_some_and(|s| s.get());
    let state = Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if press.pressed.get() {
            ControlState::Pressed
        } else if is_selected() {
            ControlState::Selected
        } else {
            ControlState::Idle
        }
    });
    let visual = use_state_visual(state);
    let content = move || {
        view! {
            {(selected.is_some())
                .then(|| {
                    view! {
                        <Show when=is_selected>
                            <span class="op-chip-check"><Icon icon=icons::CHECK size=3.5/></span>
                        </Show>
                    }
                })}
            {icon.map(|i| view! { <Icon icon=i size=3.5/> })}
            <span class="op-chip-text">{move || text.get()}</span>
        }
    };
    let main = if interactive {
        view! {
            <button
                type="button"
                class="op-rippled op-chip-main"
                role=selected.is_some().then_some("checkbox")
                aria-checked=move || selected.map(|s| s.get().to_string())
                aria-disabled=move || (!enabled.get()).then_some("true")
                on:pointerdown=move |ev| {
                    if enabled.get_untracked() {
                        press.down();
                        spawn_ripple(&ev);
                    }
                }
                on:pointerup=move |_| press.up()
                on:pointerleave=move |_| press.up()
                on:pointercancel=move |_| press.up()
                on:click=move |_| {
                    if !enabled.get_untracked() {
                        return;
                    }
                    if let Some(s) = selected {
                        s.update(|v| *v = !*v);
                    }
                    if let Some(cb) = on_click {
                        cb.run(());
                    }
                }
            >
                {content()}
            </button>
        }
        .into_any()
    } else {
        view! { <span class="op-chip-main">{content()}</span> }.into_any()
    };
    view! {
        <span
            class="op-sb op-chip"
            data-state=move || visual.state.get()
            data-result=move || visual.result.get()
            data-fading=move || visual.fading.get().then_some("")
            data-selected=move || is_selected().then_some("")
        >
            {main}
            {on_remove
                .map(|cb| {
                    view! {
                        <button
                            type="button"
                            class="op-rippled op-chip-remove"
                            aria-label=move || format!("Remove {}", text.get())
                            disabled=move || !enabled.get()
                            on:pointerdown=move |ev| spawn_ripple(&ev)
                            on:click=move |_| cb.run(())
                        >
                            <Icon icon=icons::CLOSE size=3.5/>
                        </button>
                    }
                })}
        </span>
    }
}

/// Row of chips that wraps onto new lines.
#[component]
pub fn ChipSet(children: Children) -> impl IntoView {
    view! { <div class="op-chip-set" style:gap=du(dim::PAD_TIGHT)>{children()}</div> }
}
