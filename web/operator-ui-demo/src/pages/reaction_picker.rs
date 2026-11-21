//! Live examples for ReactionPicker.

use crate::kit::Example;
use leptos::prelude::*;
use operator_ui::prelude::*;

pub fn sample_reaction_categories() -> Vec<ReactionSection> {
    vec![
        ReactionSection::new(
            "popular",
            "TOP",
            vec![
                "👍".to_string(),
                "👎".to_string(),
                "❤️".to_string(),
                "🔥".to_string(),
                "👏".to_string(),
                "🎉".to_string(),
                "⚡".to_string(),
                "🤔".to_string(),
                "🤯".to_string(),
                "🙏".to_string(),
                "💯".to_string(),
                "🎯".to_string(),
                "🚀".to_string(),
            ],
        ),
        ReactionSection::new(
            "smileys",
            "SMILEYS",
            vec![
                "😀".to_string(),
                "😃".to_string(),
                "😄".to_string(),
                "😁".to_string(),
                "😆".to_string(),
                "😅".to_string(),
                "😂".to_string(),
                "🤣".to_string(),
                "😊".to_string(),
                "😇".to_string(),
                "🙂".to_string(),
                "😉".to_string(),
                "😍".to_string(),
                "🥰".to_string(),
                "😘".to_string(),
                "😎".to_string(),
                "🥳".to_string(),
                "😏".to_string(),
                "😢".to_string(),
                "😭".to_string(),
                "😡".to_string(),
            ],
        ),
        ReactionSection::new(
            "gestures",
            "GESTURES",
            vec![
                "👍".to_string(),
                "👎".to_string(),
                "👊".to_string(),
                "✊".to_string(),
                "👏".to_string(),
                "🙌".to_string(),
                "🤝".to_string(),
                "🙏".to_string(),
                "💪".to_string(),
                "✍️".to_string(),
                "👀".to_string(),
            ],
        ),
        ReactionSection::new(
            "tactical",
            "TACTICAL",
            vec![
                "🎯".to_string(),
                "🚀".to_string(),
                "🛡️".to_string(),
                "⚔️".to_string(),
                "📡".to_string(),
                "📻".to_string(),
                "💻".to_string(),
                "💡".to_string(),
                "🔧".to_string(),
                "📦".to_string(),
                "🏆".to_string(),
            ],
        ),
    ]
}

pub fn sample_quick_reactions() -> Vec<String> {
    vec![
        "👍".to_string(),
        "👎".to_string(),
        "❤️".to_string(),
        "🔥".to_string(),
        "👏".to_string(),
        "🎉".to_string(),
        "⚡".to_string(),
    ]
}

pub fn reaction_picker() -> AnyView {
    let picked_reaction = RwSignal::new(String::from("None"));
    let is_dialog_open = RwSignal::new(false);

    let on_select = Callback::new(move |em: String| {
        picked_reaction.set(em);
    });

    view! {
        <div class="demo-stack" style="display:flex;flex-direction:column;gap:calc(var(--op-du) * 4);width:100%;max-width:calc(var(--op-du) * 120);">
            <Example
                title="Quick Reaction Strip & Full Selector"
                note="Click any quick reaction or press '+' to open the comprehensive categorized picker."
                block=true
            >
                <div style="display:flex;flex-direction:column;align-items:center;gap:calc(var(--op-du) * 3);padding:calc(var(--op-du) * 4);width:100%;">
                    <div style="font-family:monospace;font-size:calc(var(--op-tu) * 13);color:var(--op-text-primary);">
                        {move || format!("Selected reaction: {}", picked_reaction.get())}
                    </div>

                    <ReactionPicker
                        quick_reactions=sample_quick_reactions()
                        categories=sample_reaction_categories()
                        on_select=on_select
                    />
                </div>
            </Example>

            <Example
                title="Standalone Category Picker Dialog"
                note="Full modal category selector embedded directly."
                block=true
            >
                <div style="display:flex;flex-direction:column;align-items:center;gap:calc(var(--op-du) * 2);padding:calc(var(--op-du) * 2);width:100%;">
                    <button
                        type="button"
                        class="docs-cta"
                        on:click=move |_| is_dialog_open.set(true)
                    >
                        "Open Full Reaction Dialog"
                    </button>

                    <ReactionPicker
                        show_quick_bar=false
                        open=is_dialog_open
                        categories=sample_reaction_categories()
                        on_close=Callback::new(move |()| is_dialog_open.set(false))
                        on_select=on_select
                    />
                </div>
            </Example>
        </div>
    }.into_any()
}
