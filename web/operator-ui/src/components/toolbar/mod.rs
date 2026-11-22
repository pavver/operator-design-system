//! Horizontal bar of commands that moves the commands that do not fit into an overflow menu.

mod item;

use crate::components::icon_button::IconButton;
use crate::components::menu::{Menu, MenuAnchor, MenuItem};
use crate::components::tooltip::Tooltip;
use crate::core::{
    ActionResult, ControlState, Metric, Slot, root_class, root_style, sleep_ms, ui_icon, ui_text,
};
use item::Run;
pub use item::ToolbarItem;
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, icons, motion};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

pub(crate) const CSS: &str = include_str!("toolbar.css");

/// How many bar commands fit into a row `available` pixels wide.
///
/// `widths` are the command widths and `gap` the space between neighbours.
/// When not everything fits, or when `always_more` is set, the overflow button (`more` wide) takes space after the commands.
fn fit_count(widths: &[f64], gap: f64, available: f64, more: f64, always_more: bool) -> usize {
    let total: f64 = widths.iter().map(|w| w + gap).sum();
    if !always_more && total <= available + gap {
        return widths.len();
    }
    let limit = available - more;
    let mut used = 0.0;
    for (i, w) in widths.iter().enumerate() {
        used += w + gap;
        if used > limit {
            return i;
        }
    }
    widths.len()
}

fn px(value: &str) -> f64 {
    value.trim().trim_end_matches("px").parse().unwrap_or(0.0)
}

fn build_menu_items(
    shown: usize,
    in_bar: &[ToolbarItem],
    in_menu: &[ToolbarItem],
) -> Vec<MenuItem> {
    let mut out: Vec<MenuItem> = Vec::new();
    for item in in_bar.iter().skip(shown) {
        out.push(item.menu_item(item.separated && !out.is_empty()));
    }
    let had_bar_items = !out.is_empty();
    for (i, item) in in_menu.iter().enumerate() {
        let separated = (i == 0 && had_bar_items) || (item.separated && !out.is_empty());
        out.push(item.menu_item(separated));
    }
    out
}

fn step_focus(bar: &web_sys::Element, delta: i64) {
    let Ok(list) = bar.query_selector_all(
        "button:not([disabled]):not([aria-disabled=true]), input:not([disabled]), [tabindex='0']",
    ) else {
        return;
    };
    let targets: Vec<web_sys::HtmlElement> = (0..list.length())
        .filter_map(|i| list.item(i))
        .filter_map(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
        .filter(|e| e.closest("[hidden]").ok().flatten().is_none())
        .collect();
    let Ok(len) = i64::try_from(targets.len()) else {
        return;
    };
    if len == 0 {
        return;
    }
    let active = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.active_element());
    let current = targets
        .iter()
        .rposition(|t| {
            active
                .as_ref()
                .is_some_and(|a| t.contains(Some(a.as_ref())))
        })
        .and_then(|i| i64::try_from(i).ok())
        .unwrap_or(0);
    let next = usize::try_from((current + delta).rem_euclid(len)).unwrap_or(0);
    if let Some(target) = targets.get(next) {
        let _ = target.focus();
    }
}

#[component]
fn ToolbarItemView(
    item: ToolbarItem,
    index: usize,
    visible: RwSignal<usize>,
    button_sz: f32,
) -> impl IntoView {
    let hidden = move || visible.get() <= index;
    let sep = (item.separated && index > 0).then(|| {
        view! { <span class="op-toolbar-sep" role="separator" aria-orientation="vertical"></span> }
    });
    let button = match item.run.clone() {
        Some(Run::Async(action)) => view! {
            <IconButton icon=item.icon label=item.label.clone() size=button_sz enabled=item.enabled on_async_click=action/>
        }
        .into_any(),
        Some(Run::Click(callback)) => view! {
            <IconButton icon=item.icon label=item.label.clone() size=button_sz enabled=item.enabled on_click=callback/>
        }
        .into_any(),
        None => view! {
            <IconButton icon=item.icon label=item.label.clone() size=button_sz enabled=false/>
        }
        .into_any(),
    };
    view! {
        <span class="op-toolbar-item" hidden=hidden>
            {sep}
            <Tooltip text=item.label.clone()>{button}</Tooltip>
        </span>
    }
}

/// Horizontal bar of commands for the view below it.
///
/// Commands that do not fit move, from the end, into the overflow menu.
/// Arrow keys move focus between the visible controls.
#[component]
pub fn Toolbar(
    /// Commands in order.
    items: Vec<ToolbarItem>,
    #[prop(optional, into)] label: MaybeProp<String>,
    /// Title at the start of the bar.
    #[prop(optional, into)]
    title: Option<Signal<String>>,
    /// Controls at the start of the bar before the title.
    #[prop(optional, into)]
    start: Slot,
    /// Controls at the end that never overflow, such as a search field.
    #[prop(optional, into)]
    end: Slot,
    /// 8 DU tall instead of 10 DU.
    #[prop(optional)]
    dense: bool,
    /// Bar height in DU (`--op-toolbar-height`).
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Dense bar height in DU (`--op-toolbar-dense-height`).
    #[prop(optional, into)]
    dense_height: MaybeProp<f32>,
    /// Horizontal padding in DU (`--op-toolbar-padding-x`).
    #[prop(optional, into)]
    padding_x: MaybeProp<f32>,
    /// Items gap in DU (`--op-toolbar-gap`).
    #[prop(optional, into)]
    gap: MaybeProp<f32>,
    /// Dense items gap in DU (`--op-toolbar-dense-gap`).
    #[prop(optional, into)]
    dense_gap: MaybeProp<f32>,
    /// Title horizontal padding in DU (`--op-toolbar-title-padding-x`).
    #[prop(optional, into)]
    title_padding_x: MaybeProp<f32>,
    /// Title font size in TU (`--op-toolbar-title-font-size`).
    #[prop(optional, into)]
    title_font_size: MaybeProp<f32>,
    /// Separator line thickness in DU (`--op-toolbar-separator-width`).
    #[prop(optional, into)]
    separator_width: MaybeProp<f32>,
    /// Separator line height in DU (`--op-toolbar-separator-height`).
    #[prop(optional, into)]
    separator_height: MaybeProp<f32>,
    /// Separator horizontal margins in DU (`--op-toolbar-separator-margin-x`).
    #[prop(optional, into)]
    separator_margin_x: MaybeProp<f32>,
    /// Command button size in DU (`--op-toolbar-button-size`).
    #[prop(optional, into)]
    button_size: MaybeProp<f32>,
    /// Accessible name for the overflow button; defaults to `toolbar.more`.
    #[prop(optional, into)]
    more_label: MaybeProp<String>,
    /// Icon for the overflow button.
    #[prop(optional)]
    more_icon: Option<IconData>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let label = ui_text(label, "toolbar.label", "Toolbar");
    let more_label = ui_text(more_label, "toolbar.more", "More commands");
    let more_icon = ui_icon(more_icon, "toolbar.more", icons::MORE_VERT);

    let bar = NodeRef::<leptos::html::Div>::new();
    let row = NodeRef::<leptos::html::Div>::new();
    let more_host = NodeRef::<leptos::html::Span>::new();
    let default_button_size = if dense {
        dim::ROW_DENSE
    } else {
        dim::ROW_COMPACT
    };
    let button_sz = button_size.get().unwrap_or(default_button_size);

    let in_bar: Vec<ToolbarItem> = items.iter().filter(|i| !i.overflow).cloned().collect();
    let in_menu: Vec<ToolbarItem> = items.iter().filter(|i| i.overflow).cloned().collect();
    let bar_len = in_bar.len();
    let always_more = !in_menu.is_empty();
    let all = StoredValue::new(items);
    let in_bar = StoredValue::new(in_bar);
    let in_menu = StoredValue::new(in_menu);

    let visible = RwSignal::new(bar_len);
    let widths = StoredValue::new(vec![0.0_f64; bar_len]);
    let more_width = StoredValue::new(0.0_f64);

    let measure = move || {
        let (Some(row_el), Some(more_el)) = (
            row.try_get_untracked().flatten(),
            more_host.try_get_untracked().flatten(),
        ) else {
            return;
        };
        let Ok(children) = row_el.query_selector_all(":scope > .op-toolbar-item") else {
            return;
        };
        // Widths of hidden commands stay as they were last measured.
        widths.update_value(|w| {
            for (i, slot) in w.iter_mut().enumerate() {
                let Some(el) = u32::try_from(i)
                    .ok()
                    .and_then(|i| children.item(i))
                    .and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok())
                else {
                    continue;
                };
                if !el.hidden() {
                    *slot = el.get_bounding_client_rect().width();
                }
            }
        });
        let gap = web_sys::window()
            .and_then(|win| win.get_computed_style(&row_el).ok().flatten())
            .and_then(|s| s.get_property_value("column-gap").ok())
            .map_or(0.0, |v| px(&v));
        if !more_el.hidden() {
            more_width.set_value(more_el.get_bounding_client_rect().width());
        }
        // Before the overflow button has been shown, assume it is as wide as the widest command.
        let more = match more_width.get_value() {
            w if w > 0.0 => w,
            _ => widths.with_value(|w| w.iter().copied().fold(0.0, f64::max)),
        };
        // The overflow button sits inside the row, right after the last visible command.
        let available = row_el.get_bounding_client_rect().width();
        let count = widths.with_value(|w| fit_count(w, gap, available, more, always_more));
        if visible.get_untracked() != count {
            visible.set(count);
        }
    };

    Effect::new(move |_| {
        let Some(el) = bar.get() else {
            return;
        };
        request_animation_frame(measure);
        let callback = Closure::<dyn FnMut()>::new(measure);
        let Ok(observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()) else {
            return;
        };
        observer.observe(&el);
        let keep = SendWrapper::new((observer, callback));
        on_cleanup(move || keep.0.disconnect());
    });

    // Overflow menu.
    let open = RwSignal::new(false);
    let anchor = RwSignal::new(MenuAnchor::Point(0.0, 0.0));
    // The window listener closes an open menu on the same press, so remember the state at press time.
    let open_at_press = StoredValue::new(false);
    let run_state = RwSignal::new(None::<ControlState>);
    let run_job = StoredValue::new(0_u32);
    let toggle = Callback::new(move |()| {
        if let Some(h) = more_host.try_get_untracked().flatten() {
            let r = h.get_bounding_client_rect();
            anchor.set(MenuAnchor::Below(r.left(), r.top(), r.width(), r.height()));
        }
        open.set(!open_at_press.get_value());
        open_at_press.set_value(false);
    });
    let on_select = Callback::new(move |id: String| {
        let Some(run) = all.with_value(|items| {
            items
                .iter()
                .find(|i| i.id == id)
                .and_then(|i| i.run.clone())
        }) else {
            return;
        };
        match run {
            Run::Click(callback) => callback.run(()),
            Run::Async(action) => {
                run_job.update_value(|j| *j += 1);
                let job = run_job.get_value();
                run_state.set(Some(ControlState::Pending));
                spawn_local(async move {
                    let result = action.call(()).await;
                    if run_job.get_value() != job {
                        return;
                    }
                    run_state.set(Some(match result {
                        ActionResult::Success => ControlState::Success,
                        ActionResult::Error(_) => ControlState::Error,
                    }));
                    sleep_ms(motion::RESULT_HOLD_MS).await;
                    if run_job.get_value() == job {
                        run_state.set(None);
                    }
                });
            }
        }
    });
    let more_state = Signal::derive(move || {
        if open.get() {
            Some(ControlState::Selected)
        } else {
            run_state.get()
        }
    });
    let menu_items = Memo::new(move |_| {
        let shown = visible.get();
        in_bar.with_value(|bar_items| {
            in_menu.with_value(|menu_items| build_menu_items(shown, bar_items, menu_items))
        })
    });
    let has_more = move || always_more || visible.get() < bar_len;

    let bar_buttons = in_bar
        .get_value()
        .into_iter()
        .enumerate()
        .map(|(i, item)| {
            view! {
                <ToolbarItemView item=item index=i visible=visible button_sz=button_sz />
            }
        })
        .collect_view();

    let root_metrics = vec![
        Metric::du("--op-toolbar-height", height),
        Metric::du("--op-toolbar-dense-height", dense_height),
        Metric::du("--op-toolbar-padding-x", padding_x),
        Metric::du("--op-toolbar-gap", gap),
        Metric::du("--op-toolbar-dense-gap", dense_gap),
        Metric::du("--op-toolbar-title-padding-x", title_padding_x),
        Metric::tu("--op-toolbar-title-font-size", title_font_size),
        Metric::du("--op-toolbar-separator-width", separator_width),
        Metric::du("--op-toolbar-separator-height", separator_height),
        Metric::du("--op-toolbar-separator-margin-x", separator_margin_x),
        Metric::du("--op-toolbar-button-size", button_size),
    ];

    view! {
        <div
            node_ref=bar
            class=root_class("op-toolbar", class)
            style=root_style(root_metrics, style)
            role="toolbar"
            aria-label=move || label.get()
            data-dense=dense.then_some("")
            on:keydown=move |ev| {
                let in_input = ev
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    .is_some();
                if in_input {
                    return;
                }
                match ev.key().as_str() {
                    "ArrowRight" => {
                        ev.prevent_default();
                        if let Some(el) = bar.try_get_untracked().flatten() {
                            step_focus(&el, 1);
                        }
                    }
                    "ArrowLeft" => {
                        ev.prevent_default();
                        if let Some(el) = bar.try_get_untracked().flatten() {
                            step_focus(&el, -1);
                        }
                    }
                    _ => {}
                }
            }
        >
            {start.run()}
            {title.map(|t| view! {
                <span class="op-toolbar-title">{move || t.get()}</span>
                <span class="op-toolbar-sep" role="separator" aria-orientation="vertical"></span>
            })}
            <div node_ref=row class="op-toolbar-row">
                {bar_buttons}
                <span
                    node_ref=more_host
                    class="op-toolbar-more op-menu-anchor"
                    hidden=move || !has_more()
                    aria-haspopup="menu"
                    aria-expanded=move || open.get().to_string()
                    on:pointerdown=move |_| open_at_press.set_value(open.get_untracked())
                >
                    <IconButton icon=more_icon.get() label=more_label size=button_sz state=more_state on_click=toggle/>
                </span>
            </div>
            {end.run().map(|e| view! { <div class="op-toolbar-end">{e}</div> })}
        </div>
        {move || {
            let lbl = more_label.get();
            view! {
                <Menu open=open anchor=anchor.into() items=menu_items.get() on_select=on_select label=lbl/>
            }
        }}
    }
}

#[cfg(test)]
mod tests {
    use super::fit_count;

    #[test]
    fn everything_fits_without_more() {
        // 32 + 4 + 32 + 4 + 32 = 104.
        assert_eq!(fit_count(&[32.0, 32.0, 32.0], 4.0, 104.0, 32.0, false), 3);
    }

    #[test]
    fn overflow_reserves_the_more_button() {
        // Two commands and the more button need 32 + 4 + 32 + 4 + 32 = 104 > 100.
        assert_eq!(fit_count(&[32.0, 32.0, 32.0], 4.0, 100.0, 32.0, false), 1);
        assert_eq!(fit_count(&[32.0, 32.0, 32.0], 4.0, 110.0, 32.0, true), 2);
    }

    #[test]
    fn nothing_fits() {
        assert_eq!(fit_count(&[32.0, 32.0], 4.0, 20.0, 32.0, false), 0);
    }
}
