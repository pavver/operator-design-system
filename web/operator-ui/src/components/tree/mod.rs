//! Tree: an expandable hierarchy of rows, virtualized.

mod model;

pub use model::{TreeMove, TreeNode, TreeRow, flatten, key_move};

use crate::components::primitives::Icon;
use crate::components::virtual_list::{VirtualList, VirtualListHandle};
use crate::core::{Metric, ScopedSlot, Slot, root_class, root_style, ui_icon, ui_text};
use leptos::prelude::*;
use operator_ui_tokens::{DU_POINTS, dim, icons};
use send_wrapper::SendWrapper;
use std::collections::HashSet;
use wasm_bindgen::{JsCast, closure::Closure};

pub(crate) const CSS: &str = include_str!("tree.css");

/// Indentation of one tree level in DU.
pub const TREE_INDENT_DU: f32 = 4.0;

/// Expandable hierarchy with keyboard navigation; only the visible rows are rendered.
///
/// Arrow Up and Down move the selection, Right expands or enters a node, Left collapses or goes
/// to the parent, Home and End jump to the ends. A click selects, a click on the arrow or a double
/// click expands or collapses.
#[component]
pub fn Tree(
    #[prop(into)] nodes: Signal<Vec<TreeNode>>,
    /// Ids of expanded nodes.
    expanded: RwSignal<HashSet<String>>,
    selected: RwSignal<Option<String>>,
    #[prop(optional)] on_select: Option<Callback<String>>,
    /// Fixed height in DU.
    #[prop(into, default = None)]
    height: Option<f32>,
    /// Maximum height in DU.
    #[prop(into, default = None)]
    max_height: Option<f32>,
    #[prop(optional, into)] label: MaybeProp<String>,
    #[prop(optional)] option: Option<ScopedSlot<TreeRow>>,
    #[prop(optional)] chevron_icon: Option<operator_ui_tokens::Icon>,
    #[prop(optional, into)] empty: Slot,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] before_options: Slot,
    #[prop(optional, into)] after_options: Slot,
    #[prop(optional, into)] row_height: MaybeProp<f32>,
    #[prop(optional, into)] indent: MaybeProp<f32>,
    #[prop(optional, into)] padding_start: MaybeProp<f32>,
    #[prop(optional, into)] padding_end: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] detail_font_size: MaybeProp<f32>,
    #[prop(optional, into)] detail_line_height: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let rows = Memo::new(move |_| expanded.with(|open| nodes.with(|n| flatten(n, open))));
    let active_handle = StoredValue::new(VirtualListHandle::new(dim::ROW_COMPACT));
    let label = ui_text(label, "tree.label", "Tree");
    let chevron_icon = ui_icon(chevron_icon, "tree.chevron", icons::CHEVRON_RIGHT);
    let measured_row = RwSignal::new(row_height.get_untracked().unwrap_or(dim::ROW_COMPACT));
    let marker = NodeRef::<leptos::html::Div>::new();
    Effect::new(move |_| {
        let Some(el) = marker.get() else {
            return;
        };
        let refresh = move || {
            #[allow(clippy::cast_possible_truncation)]
            let height = marker
                .get_untracked()
                .map(|m| m.get_bounding_client_rect().height() as f32 / DU_POINTS);
            if let Some(height) = height.filter(|h| {
                h.is_finite()
                    && *h > 0.0
                    && (*h - measured_row.get_untracked()).abs() > f32::EPSILON
            }) {
                measured_row.set(height);
            }
        };
        refresh();
        let callback = Closure::<dyn FnMut()>::new(refresh);
        if let Ok(observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()) {
            observer.observe(&el);
            let keep = SendWrapper::new((observer, callback));
            on_cleanup(move || keep.0.disconnect());
        }
    });
    let select = move |id: String| {
        if let Some(i) = rows.with_untracked(|r| r.iter().position(|row| row.id == id)) {
            active_handle.get_value().reveal(i, false);
        }
        selected.set(Some(id.clone()));
        if let Some(cb) = on_select {
            cb.run(id);
        }
    };
    let toggle = move |id: String| {
        expanded.update(|open| {
            if !open.remove(&id) {
                open.insert(id);
            }
        });
    };
    let on_key = move |e: leptos::ev::KeyboardEvent| {
        let action = rows.with_untracked(|r| {
            selected.with_untracked(|s| key_move(r, s.as_deref(), e.key().as_str()))
        });
        if action != TreeMove::None {
            e.prevent_default();
        }
        match action {
            TreeMove::Select(id) => select(id),
            TreeMove::Expand(id) => {
                expanded.update(|open| {
                    open.insert(id);
                });
            }
            TreeMove::Collapse(id) => {
                expanded.update(|open| {
                    open.remove(&id);
                });
            }
            TreeMove::None => {}
        }
    };
    let metrics = vec![
        Metric::du("--op-tree-row-height", row_height),
        Metric::du("--op-tree-indent", indent),
        Metric::du("--op-tree-padding-start", padding_start),
        Metric::du("--op-tree-padding-end", padding_end),
        Metric::du("--op-tree-gap", gap),
        Metric::du("--op-tree-icon-size", icon_size),
        Metric::tu("--op-tree-font-size", font_size),
        Metric::tu("--op-tree-line-height", line_height),
        Metric::tu("--op-tree-detail-font-size", detail_font_size),
        Metric::tu("--op-tree-detail-line-height", detail_line_height),
        Metric::du("--op-tree-height", height.into()),
        Metric::du("--op-tree-max-height", max_height.into()),
    ];
    view! {
        <div class=root_class("op-tree", class) style=root_style(metrics, style) role="tree" aria-label=move || label.get() on:keydown=on_key>
            <div class="op-tree-measure-clip" aria-hidden="true"><div node_ref=marker class="op-tree-measure"></div></div>
            {header.run()}
            {before_options.run()}
            {move || if rows.with(Vec::is_empty) { empty.run() } else { None }}
            {move || {
            let handle = VirtualListHandle::new(measured_row.get());
            active_handle.set_value(handle);
            view! {
            <VirtualList
                items=Signal::derive(move || rows.get())
                handle=handle
                focusable=true
                key=|row: &TreeRow| (row.id.clone(), row.expanded)
                let:row
            >
                {
                    let r = row.1;
                    let id = r.id.clone();
                    let current = r.id.clone();
                    let toggle_id = r.id.clone();
                    let dbl_id = r.id.clone();
                    let has_children = r.has_children;
                    let indent = format!("padding-left:calc(var(--op-tree-indent, calc(var(--op-du) * 4)) * {} + var(--op-tree-padding-start, var(--op-du)));", r.depth);
                    let label = option.map_or_else(|| r.label.clone().into_any(), |slot| slot.run(r.clone()));
                    view! {
                        <div
                            class="op-tree-row"
                            role="treeitem"
                            attr:aria-level=(r.depth + 1).to_string()
                            aria-expanded=has_children.then(|| r.expanded.to_string())
                            aria-selected=move || selected.with(|s| s.as_deref() == Some(current.as_str())).to_string()
                            style=indent
                            on:click=move |_| select(id.clone())
                            on:dblclick=move |_| {
                                if has_children {
                                    toggle(dbl_id.clone());
                                }
                            }
                        >
                            <span
                                class="op-tree-arrow"
                                data-open=r.expanded.then_some("")
                                on:click=move |e| {
                                    if has_children {
                                        e.stop_propagation();
                                        toggle(toggle_id.clone());
                                    }
                                }
                            >
                                {has_children.then(|| view! { {move || view! { <Icon icon=chevron_icon.get() style="--op-icon-size:var(--op-tree-icon-size, var(--op-dim-icon-small))"/> }} })}
                            </span>
                            {r.icon.map(|i| view! { <span class="op-tree-icon"><Icon icon=i style="--op-icon-size:var(--op-tree-icon-size, var(--op-dim-icon-small))"/></span> })}
                            <span class="op-tree-label">{label}</span>
                            {r.detail.map(|d| view! { <span class="op-tree-detail op-t-caption">{d}</span> })}
                        </div>
                    }
                }
            </VirtualList>
            } }}
            {after_options.run()}
            {footer.run()}
        </div>
    }
}

#[cfg(test)]
mod tests;
