//! Shared pieces of the documentation pages: the simulated backend and example blocks.

use leptos::prelude::*;
use operator_ui::prelude::*;

// ---------------------------------------------------------------------------
// Simulated backend shared by every interactive example
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Success,
    Error,
    Alternate,
}

#[derive(Clone, Copy)]
pub struct Backend {
    pub latency_ms: RwSignal<u32>,
    pub outcome: RwSignal<Outcome>,
    flip: StoredValue<bool>,
    pub logs: RwSignal<Vec<String>>,
}

fn timestamp() -> String {
    let d = js_sys::Date::new_0();
    format!(
        "{:02}:{:02}:{:02}",
        d.get_hours(),
        d.get_minutes(),
        d.get_seconds()
    )
}

impl Backend {
    pub fn new() -> Self {
        Self {
            latency_ms: RwSignal::new(1200),
            outcome: RwSignal::new(Outcome::Alternate),
            flip: StoredValue::new(false),
            logs: RwSignal::new(vec![format!(
                "\x1b[2m{}\x1b[22m \x1b[34m[UI]\x1b[39m Operator UI documentation ready",
                timestamp()
            )]),
        }
    }

    pub fn log(self, line: impl Into<String>) {
        let line = format!("\x1b[2m{}\x1b[22m {}", timestamp(), line.into());
        self.logs.update(|l| {
            l.push(line);
            if l.len() > 200 {
                l.remove(0);
            }
        });
    }

    pub async fn call(self, what: String) -> ActionResult {
        self.log(format!("\x1b[36m[TX]\x1b[39m {what}"));
        sleep_ms(self.latency_ms.get_untracked()).await;
        let ok = match self.outcome.get_untracked() {
            Outcome::Success => true,
            Outcome::Error => false,
            Outcome::Alternate => {
                self.flip.update_value(|f| *f = !*f);
                self.flip.get_value()
            }
        };
        if ok {
            self.log(format!("\x1b[32m[OK]\x1b[39m {what}"));
            ActionResult::Success
        } else {
            self.log(format!(
                "\x1b[1;31m[ERROR]\x1b[22;39m {what}: \x1b[4mtimeout\x1b[24m"
            ));
            ActionResult::Error(Some("timeout".into()))
        }
    }

    pub fn action(self, what: &'static str) -> AsyncAction {
        AsyncAction::new(move |()| self.call(what.to_owned()))
    }
}

pub fn backend() -> Backend {
    use_context::<Backend>().unwrap_or_else(Backend::new)
}

/// Controls for the simulated backend, shown above interactive examples.
#[component]
pub fn SimBar() -> impl IntoView {
    let b = backend();
    let outcome_index = RwSignal::new(match b.outcome.get_untracked() {
        Outcome::Success => 0usize,
        Outcome::Error => 1,
        Outcome::Alternate => 2,
    });
    let latency = RwSignal::new(format!("{} ms", b.latency_ms.get_untracked()));
    Effect::new(move |_| {
        b.outcome.set(match outcome_index.get() {
            0 => Outcome::Success,
            1 => Outcome::Error,
            _ => Outcome::Alternate,
        });
    });
    Effect::new(move |_| {
        let ms = latency
            .get()
            .trim_end_matches(" ms")
            .parse()
            .unwrap_or(1200);
        b.latency_ms.set(ms);
    });
    view! {
        <div class="docs-sim">
            <div class="docs-sim-title op-t-caption">"Simulated backend"</div>
            <div class="docs-sim-controls">
                <div class="docs-sim-outcome">
                    <SegmentedControl
                        items=vec![SegmentItem::text("Success"), SegmentItem::text("Error"), SegmentItem::text("Alternate")]
                        selected=outcome_index
                    />
                </div>
                <div class="docs-sim-latency">
                    <DropdownMenu
                        options=vec!["150 ms".into(), "600 ms".into(), "1200 ms".into(), "3000 ms".into(), "8000 ms".into()]
                        selected=latency
                    />
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Example blocks
// ---------------------------------------------------------------------------

/// Titled live example in a bordered preview box.
#[component]
pub fn Example(
    #[prop(optional, into)] title: Option<&'static str>,
    #[prop(optional, into)] note: Option<&'static str>,
    /// Stretches the preview content to the full width.
    #[prop(optional)]
    block: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="docs-example-block">
            {title.map(|t| view! { <h3 class="docs-example-title">{t}</h3> })}
            {note.map(|n| view! { <p class="docs-example-note">{n}</p> })}
            <div class="docs-example" data-block=block.then_some("")>{children()}</div>
        </div>
    }
}

/// Labeled cell inside an example, used for static state grids.
#[component]
pub fn Cell(label: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="demo-cell">
            <div class="demo-cell-body">{children()}</div>
            <div class="demo-cell-label op-t-caption">{label}</div>
        </div>
    }
}

pub const STATES: [(&str, ControlState); 7] = [
    ("Idle", ControlState::Idle),
    ("Pressed", ControlState::Pressed),
    ("Pending", ControlState::Pending),
    ("Success", ControlState::Success),
    ("Error", ControlState::Error),
    ("Selected", ControlState::Selected),
    ("Focused", ControlState::Focused),
];

/// Wraps a synchronous handler as an action that succeeds at once.
pub fn instant(f: impl Fn() + Send + Sync + 'static) -> AsyncAction {
    AsyncAction::new(move |()| {
        f();
        async { ActionResult::Success }
    })
}
