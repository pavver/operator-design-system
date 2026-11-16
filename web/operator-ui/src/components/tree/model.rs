//! Tree data and flattening into visible rows (pure logic).

use operator_ui_tokens::Icon as IconData;
use std::collections::HashSet;

/// One node of a [`Tree`](super::Tree).
#[derive(Clone, Debug, PartialEq)]
pub struct TreeNode {
    /// Unique among all nodes of the tree.
    pub id: String,
    pub label: String,
    pub icon: Option<IconData>,
    /// Secondary text at the end of the row (a count, a state, a type).
    pub detail: Option<String>,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            detail: None,
            children: Vec::new(),
        }
    }

    pub fn icon(mut self, icon: IconData) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }
}

/// A visible row of a tree.
#[derive(Clone, Debug, PartialEq)]
pub struct TreeRow {
    pub id: String,
    pub label: String,
    pub icon: Option<IconData>,
    pub detail: Option<String>,
    pub depth: usize,
    pub has_children: bool,
    pub expanded: bool,
    pub parent: Option<String>,
}

/// Rows of the expanded part of the tree in display order.
pub fn flatten(nodes: &[TreeNode], expanded: &HashSet<String>) -> Vec<TreeRow> {
    let mut rows = Vec::new();
    push_rows(nodes, expanded, 0, None, &mut rows);
    rows
}

fn push_rows(
    nodes: &[TreeNode],
    expanded: &HashSet<String>,
    depth: usize,
    parent: Option<&str>,
    rows: &mut Vec<TreeRow>,
) {
    for node in nodes {
        let open = !node.children.is_empty() && expanded.contains(&node.id);
        rows.push(TreeRow {
            id: node.id.clone(),
            label: node.label.clone(),
            icon: node.icon,
            detail: node.detail.clone(),
            depth,
            has_children: !node.children.is_empty(),
            expanded: open,
            parent: parent.map(str::to_string),
        });
        if open {
            push_rows(&node.children, expanded, depth + 1, Some(&node.id), rows);
        }
    }
}

/// What a key does to the tree at the selected row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TreeMove {
    Select(String),
    Expand(String),
    Collapse(String),
    None,
}

/// The effect of `key` (`ArrowUp`, `ArrowDown`, `ArrowRight`, `ArrowLeft`, `Home`, `End`) at `selected`.
pub fn key_move(rows: &[TreeRow], selected: Option<&str>, key: &str) -> TreeMove {
    let at = selected.and_then(|s| rows.iter().position(|r| r.id == s));
    let select = |i: usize| {
        rows.get(i)
            .map_or(TreeMove::None, |r| TreeMove::Select(r.id.clone()))
    };
    let Some(i) = at else {
        return match key {
            "ArrowUp" | "End" => rows.len().checked_sub(1).map_or(TreeMove::None, select),
            "ArrowDown" | "Home" => select(0),
            _ => TreeMove::None,
        };
    };
    let row = &rows[i];
    match key {
        "ArrowDown" => select((i + 1).min(rows.len() - 1)),
        "ArrowUp" => select(i.saturating_sub(1)),
        "Home" => select(0),
        "End" => select(rows.len() - 1),
        "ArrowRight" if row.has_children && !row.expanded => TreeMove::Expand(row.id.clone()),
        "ArrowRight" if row.expanded => select(i + 1),
        "ArrowLeft" if row.expanded => TreeMove::Collapse(row.id.clone()),
        "ArrowLeft" => row.parent.clone().map_or(TreeMove::None, TreeMove::Select),
        _ => TreeMove::None,
    }
}
