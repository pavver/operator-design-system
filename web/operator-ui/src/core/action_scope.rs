//! Shared state of the buttons inside a group: one switch for all, and one action at a time.
use super::AsyncAction;
use leptos::prelude::*;

/// Context a group provides to the buttons inside it.
#[derive(Clone, Copy)]
pub struct ActionScope {
    enabled: Signal<bool>,
    exclusive: bool,
    running: RwSignal<Option<u64>>,
    next: StoredValue<u64>,
}

impl ActionScope {
    /// Provides a scope to the children of the calling component.
    ///
    /// `enabled` switches every member; with `exclusive`, members are blocked while another
    /// member's action runs.
    pub fn provide(enabled: Signal<bool>, exclusive: bool) -> Self {
        let scope = Self {
            enabled,
            exclusive,
            running: RwSignal::new(None),
            next: StoredValue::new(0),
        };
        provide_context(scope);
        scope
    }

    /// True while a member's action runs.
    pub fn busy(self) -> Signal<bool> {
        Signal::derive(move || self.running.get().is_some())
    }
}

/// Joins the surrounding [`ActionScope`], if any.
///
/// Returns the member's effective `enabled` signal and its action wrapped to mark the scope busy
/// while it runs; outside a scope both come back unchanged.
pub fn join_action_scope(
    enabled: Signal<bool>,
    action: Option<AsyncAction>,
) -> (Signal<bool>, Option<AsyncAction>) {
    let Some(scope) = use_context::<ActionScope>() else {
        return (enabled, action);
    };
    let id = scope.next.get_value();
    scope.next.set_value(id + 1);
    let gated = Signal::derive(move || {
        enabled.get()
            && scope.enabled.get()
            && (!scope.exclusive || scope.running.get().is_none_or(|r| r == id))
    });
    let action = action.map(|action| {
        AsyncAction::new(move |()| {
            let action = action.clone();
            async move {
                scope.running.try_set(Some(id));
                let result = action.call(()).await;
                if scope.running.try_get_untracked().flatten() == Some(id) {
                    scope.running.try_set(None);
                }
                result
            }
        })
    });
    (gated, action)
}
