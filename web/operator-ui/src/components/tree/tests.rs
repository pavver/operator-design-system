use super::model::*;
use std::collections::HashSet;

fn tree() -> Vec<TreeNode> {
    vec![
        TreeNode::new("lights", "Lights").children(vec![
            TreeNode::new("kitchen", "Kitchen"),
            TreeNode::new("hall", "Hall"),
        ]),
        TreeNode::new("sensors", "Sensors").children(vec![TreeNode::new("temp", "Temperature")]),
        TreeNode::new("plain", "Plain"),
    ]
}

fn ids(rows: &[TreeRow]) -> Vec<&str> {
    rows.iter().map(|r| r.id.as_str()).collect()
}

#[test]
fn collapsed_tree_shows_roots_only() {
    let rows = flatten(&tree(), &HashSet::new());
    assert_eq!(ids(&rows), ["lights", "sensors", "plain"]);
    assert!(rows[0].has_children && !rows[0].expanded);
    assert!(!rows[2].has_children);
}

#[test]
fn expanded_nodes_show_children_with_depth_and_parent() {
    let open: HashSet<String> = ["lights".to_string()].into();
    let rows = flatten(&tree(), &open);
    assert_eq!(
        ids(&rows),
        ["lights", "kitchen", "hall", "sensors", "plain"]
    );
    assert_eq!(rows[1].depth, 1);
    assert_eq!(rows[1].parent.as_deref(), Some("lights"));
}

#[test]
fn keys_follow_the_tree_pattern() {
    let open: HashSet<String> = ["lights".to_string()].into();
    let rows = flatten(&tree(), &open);
    assert_eq!(
        key_move(&rows, None, "ArrowDown"),
        TreeMove::Select("lights".into())
    );
    assert_eq!(
        key_move(&rows, Some("lights"), "ArrowRight"),
        TreeMove::Select("kitchen".into())
    );
    assert_eq!(
        key_move(&rows, Some("sensors"), "ArrowRight"),
        TreeMove::Expand("sensors".into())
    );
    assert_eq!(
        key_move(&rows, Some("hall"), "ArrowLeft"),
        TreeMove::Select("lights".into())
    );
    assert_eq!(
        key_move(&rows, Some("lights"), "ArrowLeft"),
        TreeMove::Collapse("lights".into())
    );
    assert_eq!(
        key_move(&rows, Some("plain"), "ArrowDown"),
        TreeMove::Select("plain".into())
    );
    assert_eq!(
        key_move(&rows, Some("lights"), "End"),
        TreeMove::Select("plain".into())
    );
    assert_eq!(key_move(&[], None, "ArrowDown"), TreeMove::None);
}
