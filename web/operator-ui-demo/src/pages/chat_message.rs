//! Live examples for ChatMessage.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn chat_message() -> AnyView {
    view! {
        <Example
            title="Conversation thread"
            note="Sent and received messages with sender names and stamps."
            block=true
        >
            <div
                class="demo-stack demo-narrow"
                style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 3);width:100%;"
            >
                <ChatMessage
                    name="Operator Dispatch"
                    stamp="10:14 AM"
                    text="Telemetry check confirmed for sector 4."
                />
                <ChatMessage
                    sent=true
                    name="You"
                    stamp="10:15 AM"
                    text="Roger that. Starting system diagnostic routine now."
                />
                <ChatMessage
                    name="Operator Dispatch"
                    stamp="10:16 AM"
                >
                    <div>
                        "Parameters recorded."
                        <div style="margin-top:calc(var(--op-du) * 1);font-weight:600;">
                            "STATUS: NOMINAL"
                        </div>
                    </div>
                </ChatMessage>
            </div>
        </Example>
    }
    .into_any()
}
