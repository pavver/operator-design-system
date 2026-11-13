//! Horizontal bar of commands that moves the commands that do not fit into an overflow menu.

use crate::components::icon_button::IconButton;
use crate::components::menu::{Menu, MenuAnchor, MenuItem};
use crate::components::tooltip::Tooltip;
use crate::core::{ActionResult, AsyncAction, ControlState, sleep_ms};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{Icon as IconData, dim, icons, motion};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;

pub(crate) const CSS: &str = include_str!("toolbar.css");

/// What a toolbar command does.
#[derive(Clone)]
enum Run {
    Click(Callback<()>),
    Async(AsyncAction),
}

/// One command of a [`Toolbar`]: an icon button in the bar, a menu row when it overflows.
#[derive(Clone)]
pub struct ToolbarItem {
    id: String,
    label: String,
    icon: IconData,
    run: Option<Run>,
    enabled: bool,
    danger: bool,
    separated: bool,
    overflow: bool,
}

impl ToolbarItem {
    /// Command with an id, a label (tooltip, accessible name, and menu text), and an icon.
    pub fn new(id: impl Into<String>, label: impl Into<String>, icon: IconData) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon,
            run: None,
            enabled: true,
            danger: false,
            separated: false,
            overflow: false,
        }
    }

    /// Runs `callback` when the command is chosen.
    pub fn on_click(mut self, callback: Callback<()>) -> Self {
        self.run = Some(Run::Click(callback));
        self
    }

    /// Runs an async action.
    /// In the bar the button shows its states; from the menu the overflow button shows them.
    pub fn on_async(mut self, action: AsyncAction) -> Self {
        self.run = Some(Run::Async(action));
        self
    }

    /// Disabled command.
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Destructive command, shown in the error color in the menu.
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    /// Starts a new group: a separator in the bar, a divider in the menu.
    pub fn separated(mut self) -> Self {
        self.separated = true;
        self
    }

    /// Always lives in the overflow menu, for rarely used commands.
    pub fn overflow(mut self) -> Self {
        self.overflow = true;
        self
    }

    fn menu_item(&self, separated: bool) -> MenuItem {
        let mut item = MenuItem::new(self.id.clone(), self.label.clone()).icon(self.icon);
        if self.danger {
            item = item.danger();
        }
        if !self.enabled {
            item = item.disabled();
        }
        if separated {
            item = item.separated();
        }
        item
    }
}

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

/// Horizontal bar of commands for the view below it.
///
/// Commands that do not fit move, from the end, into the overflow menu.
/// Arrow keys move focus between the visible controls.
#[component]
pub fn Toolbar(
    /// Commands in order.
    items: Vec<ToolbarItem>,
    #[prop(optional, into)] label: Option<String>,
    /// Title at the start of the bar.
    #[prop(optional, into)]
    title: Option<Signal<String>>,
    /// Controls at the end that never overflow, such as a search field.
    #[prop(optional, into)]
    end: Option<ViewFn>,
    /// 8 DU tall instead of 10 DU.
    #[prop(optional)]
    dense: bool,
) -> impl IntoView {
    let bar = NodeRef::<leptos::html::Div>::new();
    let row = NodeRef::<leptos::html::Div>::new();
    let more_host = NodeRef::<leptos::html::Span>::new();
    let button_size = if dense {
        dim::ROW_DENSE
    } else {
        dim::ROW_COMPACT
    };

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
        let mut out: Vec<MenuItem> = Vec::new();
        in_bar.with_value(|items| {
            for item in items.iter().skip(shown) {
                out.push(item.menu_item(item.separated && !out.is_empty()));
            }
        });
        let had_bar_items = !out.is_empty();
        in_menu.with_value(|items| {
            for (i, item) in items.iter().enumerate() {
                let separated = (i == 0 && had_bar_items) || (item.separated && !out.is_empty());
                out.push(item.menu_item(separated));
            }
        });
        out
    });
    let has_more = move || always_more || visible.get() < bar_len;

    // Arrow keys move focus between visible, enabled controls.
    let step = move |delta: i64| {
        let Some(el) = bar.try_get_untracked().flatten() else {
            return;
        };
        let Ok(list) = el.query_selector_all(
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
    };

    let bar_buttons = in_bar
        .get_value()
        .into_iter()
        .enumerate()
        .map(|(i, item)| {
            let hidden = move || visible.get() <= i;
            let sep = (item.separated && i > 0).then(|| {
                view! { <span class="op-toolbar-sep" role="separator" aria-orientation="vertical"></span> }
            });
            let button = match item.run.clone() {
                Some(Run::Async(action)) => view! {
                    <IconButton icon=item.icon label=item.label.clone() size=button_size enabled=item.enabled on_async_click=action/>
                }
                .into_any(),
                Some(Run::Click(callback)) => view! {
                    <IconButton icon=item.icon label=item.label.clone() size=button_size enabled=item.enabled on_click=callback/>
                }
                .into_any(),
                None => view! {
                    <IconButton icon=item.icon label=item.label.clone() size=button_size enabled=false/>
                }
                .into_any(),
            };
            view! {
                <span class="op-toolbar-item" hidden=hidden>
                    {sep}
                    <Tooltip text=item.label.clone()>{button}</Tooltip>
                </span>
            }
        })
        .collect_view();

    view! {
        <div
            node_ref=bar
            class="op-toolbar"
            role="toolbar"
            aria-label=label
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
                        step(1);
                    }
                    "ArrowLeft" => {
                        ev.prevent_default();
                        step(-1);
                    }
                    _ => {}
                }
            }
        >
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
                    <IconButton icon=icons::MORE_VERT label="More commands" size=button_size state=more_state on_click=toggle/>
                </span>
            </div>
            {end.map(|e| view! { <div class="op-toolbar-end">{e.run()}</div> })}
        </div>
        {move || view! {
            <Menu open=open anchor=anchor.into() items=menu_items.get() on_select=on_select label="More commands"/>
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
