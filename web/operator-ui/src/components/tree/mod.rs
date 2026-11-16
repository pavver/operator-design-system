//! Tree: an expandable hierarchy of rows, virtualized.

mod model;

pub use model::{TreeMove, TreeNode, TreeRow, flatten, key_move};

use crate::components::primitives::Icon;
use crate::components::virtual_list::{VirtualList, VirtualListHandle};
use crate::core::du;
use leptos::prelude::*;
use operator_ui_tokens::{dim, icons};
use std::collections::HashSet;

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
    #[prop(default = "Tree".into(), into)] label: String,
) -> impl IntoView {
    let rows = Memo::new(move |_| expanded.with(|open| nodes.with(|n| flatten(n, open))));
    let handle = VirtualListHandle::new(dim::ROW_COMPACT);
    let select = move |id: String| {
        if let Some(i) = rows.with_untracked(|r| r.iter().position(|row| row.id == id)) {
            handle.reveal(i, false);
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
    view! {
        <div class="op-tree" role="tree" aria-label=label on:keydown=on_key>
            <VirtualList
                items=Signal::derive(move || rows.get())
                handle=handle
                height=height
                max_height=max_height
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
                    #[allow(clippy::cast_precision_loss)]
                    let indent = du(TREE_INDENT_DU * r.depth as f32 + 1.0);
                    view! {
                        <div
                            class="op-tree-row"
                            role="treeitem"
                            attr:aria-level=(r.depth + 1).to_string()
                            aria-expanded=has_children.then(|| r.expanded.to_string())
                            aria-selected=move || selected.with(|s| s.as_deref() == Some(current.as_str())).to_string()
                            style:padding-left=indent
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
                                {has_children.then(|| view! { <Icon icon=icons::CHEVRON_RIGHT size=dim::ICON_SMALL/> })}
                            </span>
                            {r.icon.map(|i| view! { <span class="op-tree-icon"><Icon icon=i size=dim::ICON_SMALL/></span> })}
                            <span class="op-tree-label">{r.label}</span>
                            {r.detail.map(|d| view! { <span class="op-tree-detail op-t-caption">{d}</span> })}
                        </div>
                    }
                }
            </VirtualList>
        </div>
    }
}

#[cfg(test)]
mod tests;
