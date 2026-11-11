//! Data display: DataTable, Counter, PropertyList, Accordion, CodeBlock, Kbd.

use crate::basic::Icon;
use crate::buttons::IconButton;
use crate::core::{ActionResult, AsyncAction, du, spawn_ripple};
use crate::feedback::LinearProgress;
use crate::scroll::{ScrollAxis, ScrollView};
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};
use std::cmp::Ordering;
use std::sync::atomic::{AtomicU64, Ordering as AtomicOrdering};

/// Copies text to the system clipboard.
pub async fn copy_text(text: String) -> ActionResult {
    let Some(window) = web_sys::window() else {
        return ActionResult::Error(Some("no window".into()));
    };
    let promise = window.navigator().clipboard().write_text(&text);
    match wasm_bindgen_futures::JsFuture::from(promise).await {
        Ok(_) => ActionResult::Success,
        Err(_) => ActionResult::Error(Some("clipboard unavailable".into())),
    }
}

fn copy_action(text: Signal<String>) -> AsyncAction {
    AsyncAction::new(move |()| copy_text(text.get_untracked()))
}

// ---------------------------------------------------------------------------
// DataTable
// ---------------------------------------------------------------------------

/// Horizontal alignment of a table column.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

impl Align {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

/// Column of a [`DataTable`].
#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    pub title: String,
    /// Fixed width in DU; columns without a width share the rest.
    pub width: Option<f32>,
    pub align: Align,
    pub sortable: bool,
    /// Sorts by number and aligns to the end.
    pub numeric: bool,
}

impl Column {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            width: None,
            align: Align::Start,
            sortable: false,
            numeric: false,
        }
    }

    pub fn width(mut self, du: f32) -> Self {
        self.width = Some(du);
        self
    }

    pub fn sortable(mut self) -> Self {
        self.sortable = true;
        self
    }

    pub fn numeric(mut self) -> Self {
        self.numeric = true;
        self.align = Align::End;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
}

/// Row of a [`DataTable`] with a stable id.
#[derive(Clone, Debug, PartialEq)]
pub struct TableRow {
    pub id: String,
    pub cells: Vec<String>,
}

impl TableRow {
    pub fn new(id: impl Into<String>, cells: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            id: id.into(),
            cells: cells.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SortDir {
    Asc,
    Desc,
}

fn numeric_key(cell: &str) -> Option<f64> {
    let cleaned: String = cell
        .chars()
        .filter(|c| c.is_ascii_digit() || matches!(c, '.' | '-'))
        .collect();
    cleaned.parse().ok()
}

fn compare(a: &str, b: &str, numeric: bool) -> Ordering {
    if numeric {
        match (numeric_key(a), numeric_key(b)) {
            (Some(x), Some(y)) => return x.partial_cmp(&y).unwrap_or(Ordering::Equal),
            (Some(_), None) => return Ordering::Less,
            (None, Some(_)) => return Ordering::Greater,
            (None, None) => {}
        }
    }
    a.to_lowercase().cmp(&b.to_lowercase())
}

/// Table of text cells with sortable columns, a sticky header, and row selection.
#[component]
pub fn DataTable(
    columns: Vec<Column>,
    #[prop(into)] rows: Signal<Vec<TableRow>>,
    /// Enables single row selection.
    #[prop(optional, into)]
    selected: Option<RwSignal<Option<String>>>,
    #[prop(optional, into)] on_row_click: Option<Callback<String>>,
    /// Shows a progress bar under the header.
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// 8 DU rows instead of 10 DU.
    #[prop(optional)]
    dense: bool,
    /// Maximum height in DU; the body scrolls under the sticky header.
    #[prop(optional)]
    max_height: Option<f32>,
    /// Content for an empty table.
    #[prop(optional, into)]
    empty: Option<ViewFn>,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    let columns = StoredValue::new(columns);
    let sort = RwSignal::new(None::<(usize, SortDir)>);
    let col_count = columns.with_value(Vec::len);

    let sorted = Memo::new(move |_| {
        let mut list = rows.get();
        if let Some((col, dir)) = sort.get() {
            let numeric = columns.with_value(|c| c.get(col).is_some_and(|c| c.numeric));
            list.sort_by(|a, b| {
                let x = a.cells.get(col).map_or("", String::as_str);
                let y = b.cells.get(col).map_or("", String::as_str);
                let o = compare(x, y, numeric);
                if dir == SortDir::Asc { o } else { o.reverse() }
            });
        }
        list
    });

    let cycle = move |i: usize| {
        sort.update(|s| {
            *s = match *s {
                Some((c, SortDir::Asc)) if c == i => Some((i, SortDir::Desc)),
                Some((c, SortDir::Desc)) if c == i => None,
                _ => Some((i, SortDir::Asc)),
            }
        });
    };

    let head = columns
        .get_value()
        .into_iter()
        .enumerate()
        .map(|(i, col)| {
            let dir = move || sort.get().filter(|(c, _)| *c == i).map(|(_, d)| d);
            let aria_sort = move || match dir() {
                Some(SortDir::Asc) => "ascending",
                Some(SortDir::Desc) => "descending",
                None => "none",
            };
            let title = col.title.clone();
            view! {
                <th
                    scope="col"
                    data-align=col.align.as_str()
                    aria-sort=col.sortable.then_some(aria_sort)
                    style:width=col.width.map(du)
                >
                    {if col.sortable {
                        view! {
                            <button
                                type="button"
                                class="op-rippled op-table-sort"
                                data-sorted=move || dir().is_some().then_some("")
                                on:pointerdown=move |ev| spawn_ripple(&ev)
                                on:click=move |_| cycle(i)
                            >
                                <span>{title}</span>
                                <span class="op-table-sort-icon">
                                    {move || {
                                        let icon = if dir() == Some(SortDir::Desc) { icons::ARROW_DOWNWARD } else { icons::ARROW_UPWARD };
                                        view! { <Icon icon=icon size=3.5/> }
                                    }}
                                </span>
                            </button>
                        }
                            .into_any()
                    } else {
                        view! { <span class="op-table-th">{title}</span> }.into_any()
                    }}
                </th>
            }
        })
        .collect_view();

    let selectable = selected.is_some() || on_row_click.is_some();
    let body = move || {
        let list = sorted.get();
        if list.is_empty() {
            let content = empty.clone().map_or_else(
                || view! { <span class="op-table-empty-text">"No data"</span> }.into_any(),
                |e| e.run(),
            );
            return view! {
                <tr class="op-table-empty">
                    <td colspan=col_count.to_string()>{content}</td>
                </tr>
            }
            .into_any();
        }
        list.into_iter()
            .map(|row| {
                let id = row.id.clone();
                let is_selected = {
                    let id = id.clone();
                    Signal::derive(move || {
                        selected.is_some_and(|s| s.with(|v| v.as_deref() == Some(id.as_str())))
                    })
                };
                let click = move || {
                    if let Some(s) = selected {
                        s.set(Some(id.clone()));
                    }
                    if let Some(cb) = on_row_click {
                        cb.run(id.clone());
                    }
                };
                let click_key = click.clone();
                let cells = row
                    .cells
                    .into_iter()
                    .enumerate()
                    .map(|(i, cell)| {
                        let align = columns.with_value(|c| c.get(i).map_or(Align::Start, |c| c.align));
                        view! { <td data-align=align.as_str()>{cell}</td> }
                    })
                    .collect_view();
                view! {
                    <tr
                        data-selected=move || is_selected.get().then_some("")
                        aria-selected=selected.is_some().then_some(move || is_selected.get().to_string())
                        tabindex=selectable.then_some("0")
                        data-clickable=selectable.then_some("")
                        on:click=move |_| {
                            if selectable {
                                click();
                            }
                        }
                        on:keydown=move |ev| {
                            if selectable && (ev.key() == "Enter" || ev.key() == " ") {
                                ev.prevent_default();
                                click_key();
                            }
                        }
                    >
                        {cells}
                    </tr>
                }
            })
            .collect_view()
            .into_any()
    };

    view! {
        <div class="op-table-wrap" style:max-height=max_height.map(du)>
            <ScrollView axis=ScrollAxis::Both>
            <table class="op-table" data-dense=dense.then_some("") aria-label=label>
                <thead>
                    <tr>{head}</tr>
                    <Show when=move || loading.get()>
                        <tr class="op-table-progress">
                            <td colspan=col_count.to_string()>
                                <LinearProgress label="Loading rows" thickness=0.5/>
                            </td>
                        </tr>
                    </Show>
                </thead>
                <tbody>{body}</tbody>
            </table>
            </ScrollView>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Counter
// ---------------------------------------------------------------------------

/// Color of a [`Counter`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CounterTone {
    /// Unread or new items that need attention.
    #[default]
    Accent,
    /// Plain totals.
    Neutral,
    /// Failures or alarms.
    Error,
}

/// Compact number badge; values above `max` show as `max+`.
#[component]
pub fn Counter(
    #[prop(into)] count: Signal<u32>,
    #[prop(default = 99)] max: u32,
    #[prop(optional)] tone: CounterTone,
    /// Keeps the badge visible at zero.
    #[prop(optional)]
    show_zero: bool,
) -> impl IntoView {
    let tone = match tone {
        CounterTone::Accent => "accent",
        CounterTone::Neutral => "neutral",
        CounterTone::Error => "error",
    };
    let text = move || {
        let c = count.get();
        if c > max {
            format!("{max}+")
        } else {
            c.to_string()
        }
    };
    let visible = move || show_zero || count.get() > 0;
    view! {
        <Show when=visible>
            <span class="op-counter" data-tone=tone aria-label=move || format!("{} items", count.get())>
                {text}
            </span>
        </Show>
    }
}

/// Places a [`Counter`] on the top end corner of its child, for example an icon button.
#[component]
pub fn WithCounter(
    #[prop(into)] count: Signal<u32>,
    #[prop(default = 99)] max: u32,
    #[prop(optional)] tone: CounterTone,
    children: Children,
) -> impl IntoView {
    view! {
        <span class="op-with-counter">
            {children()}
            <span class="op-with-counter-badge">
                <Counter count=count max=max tone=tone/>
            </span>
        </span>
    }
}

// ---------------------------------------------------------------------------
// PropertyList
// ---------------------------------------------------------------------------

/// List of label/value pairs, for example device details.
#[component]
pub fn PropertyList(children: Children) -> impl IntoView {
    view! { <dl class="op-props">{children()}</dl> }
}

/// One row of a [`PropertyList`]; the value is `value` or custom children.
#[component]
pub fn Property(
    #[prop(into)] label: String,
    #[prop(optional, into)] value: Option<Signal<String>>,
    /// Adds a copy button for the value.
    #[prop(optional)]
    copyable: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let copy = value.filter(|_| copyable).map(copy_action);
    view! {
        <div class="op-prop">
            <dt class="op-prop-label">{label}</dt>
            <dd class="op-prop-value">
                <span class="op-prop-text">
                    {value.map(|v| move || v.get())}
                    {children.map(|c| c())}
                </span>
                {copy
                    .map(|action| {
                        view! {
                            <IconButton
                                icon=icons::CONTENT_COPY
                                label="Copy value"
                                size=dim::ROW_DENSE
                                icon_size=3.5
                                on_async_click=action
                            />
                        }
                    })}
            </dd>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Accordion / Collapsible
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct AccordionCtx {
    open: RwSignal<Option<u64>>,
}

static NEXT_SECTION: AtomicU64 = AtomicU64::new(1);

/// Group of [`Collapsible`] sections where only one is open at a time.
#[component]
pub fn Accordion(children: Children) -> impl IntoView {
    provide_context(AccordionCtx {
        open: RwSignal::new(None),
    });
    view! { <div class="op-accordion">{children()}</div> }
}

/// Section with a header row that shows or hides its content.
#[component]
pub fn Collapsible(
    #[prop(into)] title: Signal<String>,
    #[prop(optional, into)] subtitle: Option<Signal<String>>,
    /// External open state; inside an [`Accordion`] the group controls it.
    #[prop(optional, into)]
    open: Option<RwSignal<bool>>,
    #[prop(optional)] initially_open: bool,
    /// Extra content at the end of the header row, for example a counter.
    #[prop(optional, into)]
    trailing: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let id = NEXT_SECTION.fetch_add(1, AtomicOrdering::Relaxed);
    let group = use_context::<AccordionCtx>();
    let local = open.unwrap_or_else(|| RwSignal::new(initially_open));
    if let Some(g) = group.filter(|_| initially_open) {
        g.open.set(Some(id));
    }
    let is_open = Signal::derive(move || match group {
        Some(g) => g.open.get() == Some(id),
        None => local.get(),
    });
    let toggle = move || match group {
        Some(g) => g
            .open
            .update(|o| *o = if *o == Some(id) { None } else { Some(id) }),
        None => local.update(|o| *o = !*o),
    };
    let panel_id = format!("op-collapse-{id}");
    view! {
        <div class="op-collapse" data-open=move || is_open.get().then_some("")>
            <button
                type="button"
                class="op-rippled op-collapse-head"
                aria-expanded=move || is_open.get().to_string()
                aria-controls=panel_id.clone()
                on:pointerdown=move |ev| spawn_ripple(&ev)
                on:click=move |_| toggle()
            >
                <span class="op-collapse-chevron"><Icon icon=icons::CHEVRON_RIGHT size=dim::ICON_SMALL/></span>
                <span class="op-collapse-titles">
                    <span class="op-collapse-title">{move || title.get()}</span>
                    {subtitle.map(|s| view! { <span class="op-collapse-subtitle">{move || s.get()}</span> })}
                </span>
                {trailing.map(|t| view! { <span class="op-collapse-trailing">{t.run()}</span> })}
            </button>
            <div class="op-collapse-panel" id=panel_id role="region" inert=move || (!is_open.get()).then_some("")>
                <div class="op-collapse-clip">
                    <div class="op-collapse-content">{children()}</div>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// CodeBlock
// ---------------------------------------------------------------------------

/// Monospaced block of code or logs with a copy button.
#[component]
pub fn CodeBlock(
    #[prop(into)] code: Signal<String>,
    /// Label in the header, for example `rust` or `toml`.
    #[prop(optional, into)]
    language: Option<String>,
    #[prop(optional)] line_numbers: bool,
    /// Wraps long lines instead of scrolling horizontally.
    #[prop(optional)]
    wrap: bool,
    #[prop(default = true)] copy: bool,
    /// Maximum height in DU before the block scrolls.
    #[prop(optional)]
    max_height: Option<f32>,
) -> impl IntoView {
    let lines = move || {
        code.with(|c| {
            c.trim_end_matches('\n')
                .split('\n')
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
    };
    let has_head = language.is_some() || copy;
    view! {
        <div class="op-code" data-wrap=wrap.then_some("")>
            {has_head
                .then(|| {
                    view! {
                        <div class="op-code-head">
                            <span class="op-code-lang">{language.clone().unwrap_or_default()}</span>
                            {copy
                                .then(|| {
                                    view! {
                                        <IconButton
                                            icon=icons::CONTENT_COPY
                                            label="Copy code"
                                            size=dim::ROW_DENSE
                                            icon_size=3.5
                                            on_async_click=copy_action(code)
                                        />
                                    }
                                })}
                        </div>
                    }
                })}
            <ScrollView axis=ScrollAxis::Both max_height=max_height>
            <pre class="op-code-body">
                <code>
                    {move || {
                        lines()
                            .into_iter()
                            .enumerate()
                            .map(|(i, line)| {
                                view! {
                                    <span class="op-code-line">
                                        {line_numbers.then(|| view! { <span class="op-code-num" aria-hidden="true">{(i + 1).to_string()}</span> })}
                                        <span class="op-code-text">{line}</span>
                                    </span>
                                }
                            })
                            .collect_view()
                    }}
                </code>
            </pre>
            </ScrollView>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Kbd
// ---------------------------------------------------------------------------

/// Splits a shortcut such as `Ctrl+Shift+K` or `Ctrl++` into keys.
pub fn split_keys(keys: &str) -> Vec<String> {
    let mut out: Vec<String> = keys
        .split('+')
        .filter(|k| !k.trim().is_empty())
        .map(|k| k.trim().to_owned())
        .collect();
    if keys.ends_with("++") || keys == "+" {
        out.push("+".into());
    }
    out
}

/// Width class of a key cap, following a physical keyboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyWidth {
    Normal,
    /// Modifier and editing keys such as Shift, Enter, Tab, Backspace.
    Wide,
    /// The space bar.
    Space,
}

/// Returns the text printed on a key cap and its width.
pub fn key_cap(key: &str) -> (String, KeyWidth) {
    let lower = key.to_ascii_lowercase();
    let glyph = match lower.as_str() {
        "up" | "arrowup" => Some("\u{2191}"),
        "down" | "arrowdown" => Some("\u{2193}"),
        "left" | "arrowleft" => Some("\u{2190}"),
        "right" | "arrowright" => Some("\u{2192}"),
        _ => None,
    };
    if let Some(g) = glyph {
        return (g.to_owned(), KeyWidth::Normal);
    }
    let width = match lower.as_str() {
        "space" => KeyWidth::Space,
        "shift" | "enter" | "return" | "tab" | "backspace" | "caps" | "capslock" | "ctrl"
        | "control" | "alt" | "option" | "cmd" | "command" | "meta" | "win" | "super" => {
            KeyWidth::Wide
        }
        _ => KeyWidth::Normal,
    };
    (key.to_owned(), width)
}

/// Keyboard key or shortcut, for example `Ctrl+K`.
#[component]
pub fn Kbd(#[prop(into)] keys: String) -> impl IntoView {
    let parts = split_keys(&keys);
    let n = parts.len();
    view! {
        <span class="op-kbd-group" aria-label=keys.clone()>
            {parts
                .into_iter()
                .enumerate()
                .map(|(i, k)| {
                    let (cap, width) = key_cap(&k);
                    let size = match width {
                        KeyWidth::Normal => None,
                        KeyWidth::Wide => Some("wide"),
                        KeyWidth::Space => Some("space"),
                    };
                    view! {
                        <kbd class="op-kbd" data-size=size>
                            <span class="op-kbd-cap">{cap}</span>
                        </kbd>
                        {(i + 1 < n).then(|| view! { <span class="op-kbd-plus" aria-hidden="true">"+"</span> })}
                    }
                })
                .collect_view()}
        </span>
    }
}
