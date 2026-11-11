//! Shared interaction primitives: control states, async actions, state-border visuals, ripple and units.

pub mod overlay;
pub mod theme;

pub use overlay::*;
pub use theme::{OperatorTheme, Theme, ThemeRoot};

use leptos::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use wasm_bindgen::JsCast;

/// Visual and interaction state of every Operator UI control.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ControlState {
    #[default]
    Idle,
    Pressed,
    Pending,
    Success,
    Error,
    Disabled,
    Focused,
    Selected,
}

impl ControlState {
    /// Value written to the `data-state` attribute.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Pressed => "pressed",
            Self::Pending => "pending",
            Self::Success => "success",
            Self::Error => "error",
            Self::Disabled => "disabled",
            Self::Focused => "focused",
            Self::Selected => "selected",
        }
    }

    /// Success and Error carry the pulsing result animation.
    pub const fn is_result(self) -> bool {
        matches!(self, Self::Success | Self::Error)
    }
}

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

/// Converts Design Units into a CSS length.
pub fn du(value: f32) -> String {
    format!("calc(var(--op-du) * {value})")
}

/// Converts Type Units into a CSS length.
pub fn tu(value: f32) -> String {
    format!("calc(var(--op-tu) * {value})")
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

/// Reactive attributes that drive the CSS state border (`.op-sb`).
///
/// - `data-state` mirrors the current [`ControlState`].
/// - `data-result` keeps the last result (`success` / `error`) so the exit fade keeps its color.
/// - `data-fading` is present for the duration of the result exit fade.
#[derive(Clone, Copy)]
pub struct StateVisual {
    pub state: Signal<&'static str>,
    pub result: Signal<&'static str>,
    pub fading: Signal<bool>,
}

pub fn use_state_visual(state: Signal<ControlState>) -> StateVisual {
    let result = RwSignal::new(if state.get_untracked() == ControlState::Error {
        "error"
    } else {
        "success"
    });
    let fading = RwSignal::new(false);
    let previous = StoredValue::new(state.get_untracked());
    let fade_job = JobToken::new();
    let fade_ms = operator_ui_tokens::motion::RESULT_FADE_MS;

    Effect::new(move |_| {
        let current = state.get();
        let before = previous.get_value();
        if current.is_result() {
            result.set(if current == ControlState::Success {
                "success"
            } else {
                "error"
            });
            fading.set(false);
        } else if before.is_result() {
            fading.set(true);
            let id = fade_job.next();
            set_timeout(
                move || {
                    if fade_job.is_current(id) {
                        fading.set(false);
                    }
                },
                Duration::from_millis(u64::from(fade_ms)),
            );
        }
        previous.set_value(current);
    });

    StateVisual {
        state: Signal::derive(move || state.get().as_str()),
        result: result.into(),
        fading: fading.into(),
    }
}

/// Returns `true` once `pending` has stayed `true` for `motion.pending_indicator_delay`.
///
/// Fast requests therefore show only the running Pending border, never the spinner.
/// Controls render their regular content while the returned signal is `false`.
pub fn use_pending_indicator(pending: impl Fn() -> bool + Send + Sync + 'static) -> Signal<bool> {
    // A memo notifies only on real changes, so repeated `Pending` writes do not restart the delay.
    let pending = Memo::new(move |_| pending());
    let elapsed = RwSignal::new(false);
    let job = JobToken::new();
    let delay_ms = operator_ui_tokens::motion::PENDING_INDICATOR_DELAY_MS;

    Effect::new(move |_| {
        let id = job.next();
        elapsed.set(false);
        if pending.get() {
            set_timeout(
                move || {
                    if job.is_current(id) {
                        elapsed.set(true);
                    }
                },
                Duration::from_millis(u64::from(delay_ms)),
            );
        }
    });

    Signal::derive(move || pending.get() && elapsed.get())
}

/// Tracks pointer press state for a control.
#[derive(Clone, Copy)]
pub struct Press {
    pub pressed: RwSignal<bool>,
}

impl Press {
    pub fn new() -> Self {
        Self {
            pressed: RwSignal::new(false),
        }
    }

    pub fn down(&self) {
        self.pressed.set(true);
    }

    pub fn up(&self) {
        self.pressed.set(false);
    }
}

impl Default for Press {
    fn default() -> Self {
        Self::new()
    }
}

/// Spawns a bounded ripple inside the event's current target (host must be `.op-rippled`).
pub fn spawn_ripple(ev: &web_sys::PointerEvent) {
    let Some(host) = ev
        .current_target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
    else {
        return;
    };
    let Some(document) = host.owner_document() else {
        return;
    };
    let Ok(node) = document.create_element("span") else {
        return;
    };
    let Ok(ripple) = node.dyn_into::<web_sys::HtmlElement>() else {
        return;
    };
    let rect = host.get_bounding_client_rect();
    let x = f64::from(ev.client_x()) - rect.left();
    let y = f64::from(ev.client_y()) - rect.top();
    let radius = (rect.width().max(x).hypot(rect.height().max(y))).max(1.0);
    ripple.set_class_name("op-ripple");
    let style = ripple.style();
    let _ = style.set_property("left", &format!("{}px", x - radius));
    let _ = style.set_property("top", &format!("{}px", y - radius));
    let _ = style.set_property("width", &format!("{}px", radius * 2.0));
    let _ = style.set_property("height", &format!("{}px", radius * 2.0));
    if host.append_child(&ripple).is_err() {
        return;
    }
    hold_ripple(ripple);
}

/// Time the ripple takes to grow over the control, in milliseconds.
const RIPPLE_GROW_MS: u64 = 330;
/// Time the ripple takes to fade after the pointer is released, in milliseconds.
const RIPPLE_FADE_MS: u64 = 220;

/// Keeps a grown ripple on screen while the pointer is held and fades it after release.
///
/// The fade starts once both the growth animation has ended and the pointer is up,
/// so a long press holds the last frame of the ripple.
fn hold_ripple(ripple: web_sys::HtmlElement) {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use web_sys::js_sys::Function;

    const GROWN: u8 = 1;
    const RELEASED: u8 = 2;
    let flags = Rc::new(Cell::new(0u8));
    let finish = {
        let ripple = ripple.clone();
        let flags = flags.clone();
        move |flag: u8| {
            flags.set(flags.get() | flag);
            if flags.get() == GROWN | RELEASED {
                let _ = ripple.set_attribute("data-out", "");
                let ripple = ripple.clone();
                set_timeout(
                    move || ripple.remove(),
                    Duration::from_millis(RIPPLE_FADE_MS),
                );
            }
        }
    };

    // A timer rather than `animationend`: the event never fires when animations are disabled.
    {
        let finish = finish.clone();
        set_timeout(move || finish(GROWN), Duration::from_millis(RIPPLE_GROW_MS));
    }

    let Some(window) = web_sys::window() else {
        return;
    };
    // The release listener removes itself from both events; the cell breaks the self-reference.
    let slot: Rc<RefCell<Option<Function>>> = Rc::new(RefCell::new(None));
    let on_release = {
        let slot = slot.clone();
        let window = window.clone();
        wasm_bindgen::closure::Closure::once_into_js(move || {
            if let Some(f) = slot.borrow_mut().take() {
                let _ = window.remove_event_listener_with_callback("pointerup", &f);
                let _ = window.remove_event_listener_with_callback("pointercancel", &f);
            }
            finish(RELEASED);
        })
    };
    let f: Function = on_release.unchecked_into();
    let _ = window.add_event_listener_with_callback("pointerup", &f);
    let _ = window.add_event_listener_with_callback("pointercancel", &f);
    *slot.borrow_mut() = Some(f);
}

/// Running clockwise pending segment, shown by CSS only while the parent `.op-sb` is pending.
#[component]
pub fn PendingRun() -> impl IntoView {
    view! {
        <svg class="op-run" aria-hidden="true">
            <rect pathLength="100"></rect>
        </svg>
    }
}

/// Result icon shown briefly after an async action completes.
pub(crate) fn flash_icon(state: ControlState, size: f32) -> AnyView {
    if state == ControlState::Success {
        view! { <span class="op-c-success"><crate::components::primitives::Icon icon=operator_ui_tokens::icons::CHECK size=size label="Success"/></span> }.into_any()
    } else {
        view! { <span class="op-c-error"><crate::components::primitives::Icon icon=operator_ui_tokens::icons::CLOSE size=size label="Error"/></span> }
            .into_any()
    }
}

/// Shared async toggle flow: Pending -> result -> hold -> Idle, with rollback on error.
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
pub(crate) fn mirrored(source: Signal<bool>) -> RwSignal<bool> {
    let local = RwSignal::new(source.get_untracked());
    Effect::new(move |_| local.set(source.get()));
    local
}

/// Shared field state: disabled takes priority, then explicit state, then focused, else idle.
pub(crate) fn field_state(
    enabled: Signal<bool>,
    state: Signal<Option<ControlState>>,
    focused: RwSignal<bool>,
) -> Signal<ControlState> {
    Signal::derive(move || {
        if !enabled.get() {
            ControlState::Disabled
        } else if let Some(s) = state.get() {
            s
        } else if focused.get() {
            ControlState::Focused
        } else {
            ControlState::Idle
        }
    })
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

pub(crate) fn copy_action(text: Signal<String>) -> AsyncAction {
    AsyncAction::new(move |()| copy_text(text.get_untracked()))
}
