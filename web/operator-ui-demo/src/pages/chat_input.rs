//! Live examples for ChatInput.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn chat_input() -> AnyView {
    let input_text = RwSignal::new(String::new());
    let sent_log = RwSignal::new(Vec::<String>::new());
    let input_state = RwSignal::new(ControlState::Idle);

    let reply_msg = RwSignal::new(Some(MessageReply::new(
        "msg_104",
        "Operator Alpha",
        "Coordinates validated for primary sector entrance.",
    )));

    let on_send = Callback::new(move |()| {
        let txt = input_text.get();
        if !txt.trim().is_empty() {
            input_state.set(ControlState::Pending);
            sent_log.update(|log| log.push(txt));
            input_text.set(String::new());
            reply_msg.set(None);
            input_state.set(ControlState::Success);
            set_timeout(
                move || {
                    input_state.set(ControlState::Idle);
                },
                std::time::Duration::from_millis(800),
            );
        }
    });

    view! {
        <div class="demo-stack" style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 4);width:100%;max-width:calc(var(--op-du) * 140);">
            <Example
                title="Auto-Growing Chat Input with Reply & Media Recording"
                note="Type multi-line messages, quote replies, or switch to media record button when empty."
                block=true
            >
                <div style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 3);width:100%;padding:calc(var(--op-du) * 2);">
                    <div style="background:var(--op-surface-raised);padding:calc(var(--op-du) * 2);min-height:calc(var(--op-du) * 15);font-family:monospace;font-size:calc(var(--op-tu) * 12);">
                        <div style="color:var(--op-text-secondary);margin-bottom:calc(var(--op-du) * 1);">"Transmitted messages:"</div>
                        {move || {
                            let msgs = sent_log.get();
                            if msgs.is_empty() {
                                view! { <div style="color:var(--op-text-secondary);font-style:italic;">"(None yet)"</div> }.into_any()
                            } else {
                                msgs.into_iter().map(|m| view! { <div>{format!("> {m}")}</div> }).collect::<Vec<_>>().into_any()
                            }
                        }}
                    </div>

                    <ChatInput
                        value=input_text
                        on_input=Callback::new(move |t| input_text.set(t))
                        on_send=on_send
                        reply=reply_msg
                        state=input_state
                        on_cancel_reply=Callback::new(move |()| reply_msg.set(None))
                        on_attach=Callback::new(move |()| sent_log.update(|l| l.push("[Attachment clicked]".to_string())))
                    />
                </div>
            </Example>
        </div>
    }.into_any()
}
