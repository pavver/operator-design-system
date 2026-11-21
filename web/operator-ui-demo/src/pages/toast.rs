//! Live examples for toast.

use crate::kit::{Example, backend, instant};
use leptos::prelude::*;
use operator_ui::prelude::*;

const LONG_ERROR: &str = "Push preset failed: camera 03 (Gate North, 192.168.1.40) rejected the request with HTTP 409 Conflict because another operator is editing the same PTZ preset. The previous preset is still active. Close the editor on the other workstation or wait until the lock expires in 5 minutes, then try again. Request id 7f3c9a12-4b8e-4d21-9a6f-2c1e5b7d8f90.";
const LONG_WARNING: &str = "Recording sync is behind by 47 minutes on 6 of 12 cameras. The storage node at the north site answers slowly and the upload queue holds 1.8 GB. Recent footage stays on the cameras until the queue drains.";

pub fn toast() -> AnyView {
    let b = backend();
    let toaster = use_toaster();
    let show = move |tone: Tone, text: &'static str| {
        Callback::new(move |()| {
            if let Some(t) = toaster {
                t.message(tone, text);
            }
        })
    };
    let undo = Callback::new(move |()| {
        if let Some(t) = toaster {
            t.show(
                ToastSpec::new(Tone::Info, "Recording moved to trash")
                    .with_action("Undo", Callback::new(move |()| b.log("[UI] undo delete"))),
            );
        }
    });
    let failing = AsyncAction::new(move |()| async move {
        let r = b.call("push preset".into()).await;
        if let Some(t) = toaster {
            t.report(&r, "Push preset");
            if r.is_success() {
                t.message(Tone::Success, "Preset pushed to 3 cameras");
            }
        }
        r
    });
    // The stack position is global; restore the default when leaving the page.
    on_cleanup(move || {
        if let Some(t) = toaster {
            t.set_position(ToastPosition::default());
        }
    });
    let positions = ToastPosition::ALL
        .into_iter()
        .map(|p| {
            let place = instant(move || {
                if let Some(t) = toaster {
                    t.set_position(p);
                    t.message(Tone::Info, format!("Toast at {}", p.as_str()));
                }
            });
            view! { <ActionButton text=p.as_str() on_click=place/> }
        })
        .collect_view();
    view! {
        <Example title="Position" note="Set on ToastHost or at runtime with set_position. Top stacks put the newest toast nearest the edge.">
            {positions}
        </Example>
        <Example title="Tones">
            <ActionButton text="Info" on_click=instant(move || show(Tone::Info, "Firmware 4.2 is available").run(()))/>
            <ActionButton text="Success" on_click=instant(move || show(Tone::Success, "Settings saved").run(()))/>
            <ActionButton text="Warning" on_click=instant(move || show(Tone::Warning, "Storage is 92% full").run(()))/>
            <ActionButton text="Error" on_click=instant(move || show(Tone::Error, "Camera 03 did not respond").run(()))/>
        </Example>
        <Example title="Long text" note="Checks how wrapped text, the icon and the buttons behave in a tall toast.">
            <ActionButton text="Long error" on_click=instant(move || show(Tone::Error, LONG_ERROR).run(()))/>
            <ActionButton text="Long with action" on_click=instant(move || {
                if let Some(t) = toaster {
                    t.show(
                        ToastSpec::new(Tone::Warning, LONG_WARNING)
                            .with_action("Retry", Callback::new(move |()| b.log("[UI] retry sync"))),
                    );
                }
            })/>
        </Example>
        <Example title="With an action">
            <ActionButton text="Delete recording" dangerous=true on_click=instant(move || undo.run(()))/>
        </Example>
        <Example title="Reporting an async result" note="The control shows its own result; the toast carries the error message the control cannot show.">
            <ActionButton text="Push preset" on_click=failing/>
        </Example>
    }
    .into_any()
}
