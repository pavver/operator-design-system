//! Sortable data table: DataTable, Column, TableRow, Align.

use crate::components::primitives::Icon;
use crate::components::scrollbar::{ScrollAxis, ScrollView};
use crate::core::{
    Metric, ScopedSlot, Slot, du, root_class, root_style, spawn_ripple, ui_icon, ui_text,
};
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

/// Data passed to the scoped body-cell slot.
#[derive(Clone, Debug, PartialEq)]
pub struct TableCell {
    pub row: TableRow,
    pub column_index: usize,
    pub column: Option<Column>,
    pub value: String,
}

/// Data passed to the scoped header-cell slot.
#[derive(Clone, Debug, PartialEq)]
pub struct TableHeaderCell {
    pub column_index: usize,
    pub column: Column,
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
    empty: Slot,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] empty_text: MaybeProp<String>,
    #[prop(optional, into)] loading_label: MaybeProp<String>,
    #[prop(optional, into)] sort_ascending_label: MaybeProp<String>,
    #[prop(optional, into)] sort_descending_label: MaybeProp<String>,
    #[prop(optional, into)] sort_clear_label: MaybeProp<String>,
    #[prop(optional)] sort_ascending_icon: Option<operator_ui_tokens::Icon>,
    #[prop(optional)] sort_descending_icon: Option<operator_ui_tokens::Icon>,
    #[prop(optional)] row: Option<ScopedSlot<TableRow>>,
    #[prop(optional)] cell: Option<ScopedSlot<TableCell>>,
    #[prop(optional)] header_cell: Option<ScopedSlot<TableHeaderCell>>,
    #[prop(optional, into)] row_height: MaybeProp<f32>,
    #[prop(optional, into)] dense_row_height: MaybeProp<f32>,
    #[prop(optional, into)] header_height: MaybeProp<f32>,
    #[prop(optional, into)] padding_x: MaybeProp<f32>,
    #[prop(optional, into)] sort_gap: MaybeProp<f32>,
    #[prop(optional, into)] sort_padding: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] loading_thickness: MaybeProp<f32>,
    #[prop(optional, into)] empty_padding: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] header_font_size: MaybeProp<f32>,
    #[prop(optional, into)] header_letter_spacing: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let empty_text = ui_text(empty_text, "data-table.empty", "No data");
    let loading_label = ui_text(loading_label, "data-table.loading", "Loading rows");
    let label = ui_text(label, "data-table.label", "");
    let ascending_label = ui_text(
        sort_ascending_label,
        "data-table.sort-ascending",
        "Sort ascending",
    );
    let descending_label = ui_text(
        sort_descending_label,
        "data-table.sort-descending",
        "Sort descending",
    );
    let clear_label = ui_text(sort_clear_label, "data-table.sort-clear", "Clear sorting");
    let ascending_icon = ui_icon(
        sort_ascending_icon,
        "data-table.sort-ascending",
        icons::ARROW_UPWARD,
    );
    let descending_icon = ui_icon(
        sort_descending_icon,
        "data-table.sort-descending",
        icons::ARROW_DOWNWARD,
    );
    let row_slot = row;
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
            let sort_title = col.title.clone();
            let title = header_cell.map_or_else(
                || col.title.clone().into_any(),
                |slot| slot.run(TableHeaderCell { column_index: i, column: col.clone() }),
            );
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
                                aria-label=move || format!("{}: {}", sort_title, match dir() {
                                    None => ascending_label.get(),
                                    Some(SortDir::Asc) => descending_label.get(),
                                    Some(SortDir::Desc) => clear_label.get(),
                                })
                                on:pointerdown=move |ev| spawn_ripple(&ev)
                                on:click=move |_| cycle(i)
                            >
                                <span>{title}</span>
                                <span class="op-table-sort-icon">
                                    {move || {
                                        let icon = if dir() == Some(SortDir::Desc) { descending_icon.get() } else { ascending_icon.get() };
                                        view! { <Icon icon=icon style="--op-icon-size:var(--op-data-table-icon-size, var(--op-dim-icon-xsmall))"/> }
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
            let content = if empty.is_set() {
                empty.run().into_any()
            } else {
                view! { <span class="op-table-empty-text">{move || empty_text.get()}</span> }
                    .into_any()
            };
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
                let cells = if let Some(slot) = row_slot { slot.run(row.clone()) } else { row
                    .cells
                    .iter().cloned()
                    .enumerate()
                    .map(|(i, value)| {
                        let align = columns.with_value(|c| c.get(i).map_or(Align::Start, |c| c.align));
                        let content = cell.map_or_else(
                            || value.clone().into_any(),
                            |slot| slot.run(TableCell {
                                row: row.clone(), column_index: i,
                                column: columns.with_value(|c| c.get(i).cloned()), value: value.clone(),
                            }),
                        );
                        view! { <td data-align=align.as_str()>{content}</td> }
                    })
                    .collect_view().into_any() };
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

    let metrics = vec![
        Metric::du("--op-data-table-max-height", max_height.into()),
        Metric::du("--op-data-table-row-height", row_height),
        Metric::du("--op-data-table-dense-row-height", dense_row_height),
        Metric::du("--op-data-table-header-height", header_height),
        Metric::du("--op-data-table-padding-x", padding_x),
        Metric::du("--op-data-table-sort-gap", sort_gap),
        Metric::du("--op-data-table-sort-padding", sort_padding),
        Metric::du("--op-data-table-icon-size", icon_size),
        Metric::du("--op-data-table-loading-thickness", loading_thickness),
        Metric::du("--op-data-table-empty-padding", empty_padding),
        Metric::tu("--op-data-table-font-size", font_size),
        Metric::tu("--op-data-table-line-height", line_height),
        Metric::tu("--op-data-table-header-font-size", header_font_size),
        Metric::tu(
            "--op-data-table-header-letter-spacing",
            header_letter_spacing,
        ),
    ];
    view! {
        <div class=root_class("op-table-wrap", class) style=root_style(metrics, style) >
            {header.run()}
            <ScrollView axis=ScrollAxis::Both>
            <table class="op-table" data-dense=dense.then_some("") aria-label=move || { let name = label.get(); (!name.is_empty()).then_some(name) }>
                <thead>
                    <tr>{head}</tr>
                    <Show when=move || loading.get()>
                        <tr class="op-table-progress">
                            <td colspan=col_count.to_string()>
                                <div class="op-progress" style="--op-linear-progress-thickness:var(--op-data-table-loading-thickness, calc(var(--op-du) * 0.5));--op-linear-progress-bar:var(--op-data-table-loading, var(--op-pending));--op-linear-progress-track:var(--op-data-table-loading-track, color-mix(in srgb, var(--op-data-table-loading, var(--op-pending)) 20%, var(--op-surface-alt)))">
                                    <div class="op-progress-track" role="progressbar" aria-label=move || loading_label.get() aria-valuemin="0" aria-valuemax="1" data-tone="pending" data-indeterminate="">
                                        <div class="op-progress-fill"></div>
                                    </div>
                                </div>
                            </td>
                        </tr>
                    </Show>
                </thead>
                <tbody>
                    {before_options.is_set().then(|| view! { <tr><td colspan=col_count.to_string()>{before_options.run()}</td></tr> })}
                    {body}
                    {after_options.is_set().then(|| view! { <tr><td colspan=col_count.to_string()>{after_options.run()}</td></tr> })}
                </tbody>
            </table>
            </ScrollView>
            {footer.run()}
        </div>
    }
}
