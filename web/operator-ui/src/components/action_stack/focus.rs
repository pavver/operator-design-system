//! Keyboard focus inside the action stack: arrows move between the enabled actions and wrap.
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Which action takes focus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Target {
    First,
    Last,
    /// The next (`1`) or previous (`-1`) enabled action from the focused one, wrapping.
    Step(i32),
}

/// Index to focus among `count` actions, from the focused index if any.
pub(super) fn next_index(target: Target, current: Option<u32>, count: u32) -> Option<u32> {
    if count == 0 {
        return None;
    }
    let n = i64::from(count);
    let index = match (target, current) {
        (Target::First, _) => 0,
        (Target::Last, _) => n - 1,
        (Target::Step(delta), Some(i)) => (i64::from(i) + i64::from(delta)).rem_euclid(n),
        (Target::Step(delta), None) if delta > 0 => 0,
        (Target::Step(_), None) => n - 1,
    };
    u32::try_from(index).ok()
}

/// Moves focus to an enabled action of the stack layer.
pub(super) fn focus_action(layer: NodeRef<leptos::html::Div>, target: Target) {
    let Some(el) = layer.try_get_untracked().flatten() else {
        return;
    };
    let Ok(list) = el.query_selector_all("[role=menuitem]:not([aria-disabled])") else {
        return;
    };
    let active = document().active_element();
    let current = (0..list.length()).find(|&i| {
        list.item(i)
            .zip(active.as_ref())
            .is_some_and(|(node, a)| node.is_same_node(Some(a.as_ref())))
    });
    if let Some(item) = next_index(target, current, list.length())
        .and_then(|i| list.item(i))
        .and_then(|node| node.dyn_into::<web_sys::HtmlElement>().ok())
    {
        let _ = item.focus();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_wrap_around() {
        assert_eq!(next_index(Target::Step(1), Some(2), 3), Some(0));
        assert_eq!(next_index(Target::Step(-1), Some(0), 3), Some(2));
        assert_eq!(next_index(Target::Step(1), Some(0), 3), Some(1));
    }

    #[test]
    fn without_focus_down_takes_the_first_and_up_the_last() {
        assert_eq!(next_index(Target::Step(1), None, 3), Some(0));
        assert_eq!(next_index(Target::Step(-1), None, 3), Some(2));
    }

    #[test]
    fn home_and_end() {
        assert_eq!(next_index(Target::First, Some(1), 3), Some(0));
        assert_eq!(next_index(Target::Last, Some(1), 3), Some(2));
    }

    #[test]
    fn nothing_to_focus() {
        assert_eq!(next_index(Target::First, None, 0), None);
    }
}
