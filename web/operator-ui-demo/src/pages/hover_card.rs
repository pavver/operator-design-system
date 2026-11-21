//! Live examples for hover card.

use crate::i18n::l;
use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn hover_card() -> AnyView {
    view! {
        <Example title="Node help" note="Hover the icon, or tap it on touch. The card stays open while the pointer is over it.">
            <div class="demo-row">
                <span class="op-t-body">Delay</span>
                <HoverCard
                    label="About Delay"
                    card=|| view! {
                        <span class="op-t-row-title">Delay</span>
                        <span class="op-t-body">"Passes each trigger on after the set duration. A new trigger while waiting restarts the wait."</span>
                        <span class="op-t-caption demo-muted">"Inputs: Exec, Duration. Output: Exec."</span>
                        <a class="op-t-body" href="#components/popover">Read more</a>
                    }
                >
                    <IconButton icon=operator_ui::tokens::icons::INFO label="About Delay" size=operator_ui::tokens::dim::ROW_DENSE on_click=Callback::new(|()| {})/>
                </HoverCard>
            </div>
        </Example>
        <Example title="Customization">
            <HoverCard width=65.0 padding=4.0 gap=2.0 offset=3.0 aria_label=l("Device help", "Довідка про пристрій")
                header=move || view! { <strong>{l("Device help", "Довідка про пристрій")}</strong> }
                footer=move || view! { <small>{l("Tap outside to close", "Натисніть поза карткою, щоб закрити")}</small> }
                card=move || view! { <span>{l("The card stays open while you use its content.", "Картка залишається відкритою під час роботи з її вмістом.")}</span> }
                style="--op-hover-card-background: var(--op-surface);"
            >
                <span tabindex="0" class="docs-dotted">{l("Device details", "Відомості про пристрій")}</span>
            </HoverCard>
        </Example>
    }
    .into_any()
}
