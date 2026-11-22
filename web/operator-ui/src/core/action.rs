//! Asynchronous control actions, job tokens, toggle flow, and clipboard helpers.

use super::ControlState;
use leptos::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Outcome of an asynchronous control action.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionResult {
    Success,
    Error(Option<String>),
}

impl ActionResult {
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }

    const fn to_state(&self) -> ControlState {
        match self {
            Self::Success => ControlState::Success,
            Self::Error(_) => ControlState::Error,
        }
    }
}

impl From<ActionResult> for ControlState {
    fn from(value: ActionResult) -> Self {
        value.to_state()
    }
}

type BoxedFuture = Pin<Box<dyn Future<Output = ActionResult>>>;

/// An asynchronous handler `Fn(A) -> Future<Output = ActionResult>`.
///
/// The handler itself is `Send + Sync` so it can live inside reactive closures,
/// while the returned future runs on the browser's single thread via `spawn_local`.
pub struct AsyncAction<A = ()>(Arc<dyn Fn(A) -> BoxedFuture + Send + Sync>);

impl<A> Clone for AsyncAction<A> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<A: 'static> AsyncAction<A> {
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: Fn(A) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ActionResult> + 'static,
    {
        Self(Arc::new(move |arg| Box::pin(f(arg))))
    }

    pub fn call(&self, arg: A) -> BoxedFuture {
        (self.0)(arg)
    }
}

impl<A: 'static, F, Fut> From<F> for AsyncAction<A>
where
    F: Fn(A) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ActionResult> + 'static,
{
    fn from(f: F) -> Self {
        Self::new(f)
    }
}

/// Suspends the current async task for `ms` milliseconds.
pub async fn sleep_ms(ms: u32) {
    gloo_timers::future::TimeoutFuture::new(ms).await;
}

/// Monotonic token used to drop results of superseded async jobs.
#[derive(Clone, Copy)]
pub struct JobToken(StoredValue<u64>);

impl JobToken {
    pub fn new() -> Self {
        Self(StoredValue::new(0))
    }

    /// Starts a new job and returns its id; previous jobs become stale.
    pub fn next(&self) -> u64 {
        self.0.update_value(|v| *v += 1);
        self.0.get_value()
    }

    pub fn is_current(&self, id: u64) -> bool {
        self.0.try_get_value() == Some(id)
    }
}

impl Default for JobToken {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared async toggle flow: Pending -> result -> hold -> Idle, with rollback on error.
#[cfg(any(feature = "checkbox", feature = "switch"))]
pub(crate) fn run_toggle(
    action: AsyncAction<bool>,
    checked: RwSignal<bool>,
    local: RwSignal<ControlState>,
    job: JobToken,
) {
    let id = job.next();
    leptos::task::spawn_local(async move {
        let previous = checked.get_untracked();
        let target = !previous;
        local.set(ControlState::Pending);
        let result = action.call(target).await;
        if !job.is_current(id) {
            return;
        }
        if result.is_success() {
            checked.set(target);
            local.set(ControlState::Success);
        } else {
            checked.set(previous);
            local.set(ControlState::Error);
        }
        sleep_ms(operator_ui_tokens::motion::RESULT_HOLD_MS).await;
        if job.is_current(id) {
            local.set(ControlState::Idle);
        }
    });
}

/// Local copy of an external boolean that resets whenever the external value changes.
#[cfg(any(feature = "checkbox", feature = "switch"))]
pub(crate) fn mirrored(source: Signal<bool>) -> RwSignal<bool> {
    let local = RwSignal::new(source.get_untracked());
    Effect::new(move |_| local.set(source.get()));
    local
}

/// Reactive controller for two-state toggles (AsyncCheckbox, AsyncSwitch).
#[cfg(any(feature = "checkbox", feature = "switch"))]
#[derive(Clone, Copy)]
pub(crate) struct ToggleControl {
    pub value: RwSignal<bool>,
    pub current: Signal<ControlState>,
    pub press: super::Press,
    local: RwSignal<ControlState>,
    job: JobToken,
    has_action: bool,
}

#[cfg(any(feature = "checkbox", feature = "switch"))]
impl ToggleControl {
    pub fn new(
        checked: Signal<bool>,
        state: Signal<Option<ControlState>>,
        enabled: Signal<bool>,
        has_action: bool,
    ) -> Self {
        let local = RwSignal::new(ControlState::Idle);
        let value = mirrored(checked);
        let job = JobToken::new();
        let press = super::Press::new();

        let current = Signal::derive(move || {
            let managed = state.get().unwrap_or_else(|| local.get());
            if !enabled.get() {
                ControlState::Disabled
            } else if managed != ControlState::Idle {
                managed
            } else if press.pressed.get() {
                ControlState::Pressed
            } else {
                ControlState::Idle
            }
        });

        Self {
            value,
            current,
            press,
            local,
            job,
            has_action,
        }
    }

    pub fn clickable(&self) -> bool {
        self.has_action
            && !matches!(
                self.current.get(),
                ControlState::Disabled | ControlState::Pending
            )
    }

    pub fn toggle(&self, action: Option<&AsyncAction<bool>>) {
        if let Some(act) = action {
            run_toggle(act.clone(), self.value, self.local, self.job);
        }
    }
}

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

#[cfg(any(feature = "code-block", feature = "property-list"))]
pub(crate) fn copy_action(text: Signal<String>) -> AsyncAction {
    AsyncAction::new(move |()| copy_text(text.get_untracked()))
}
