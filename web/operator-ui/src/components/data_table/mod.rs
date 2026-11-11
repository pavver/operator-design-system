//! Sortable data table: DataTable, Column, TableRow, Align.

use crate::components::linear_progress::LinearProgress;
use crate::components::primitives::Icon;
use crate::components::scrollbar::{ScrollAxis, ScrollView};
use crate::core::{du, spawn_ripple};
use leptos::prelude::*;
use operator_ui_tokens::icons;
use std::cmp::Ordering;

pub(crate) const CSS: &str = include_str!("data_table.css");

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
