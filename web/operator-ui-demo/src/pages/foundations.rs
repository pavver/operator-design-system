//! Live illustrations for the foundation pages.

use crate::kit::{Cell, Example, STATES};
use leptos::prelude::*;
use operator_ui::prelude::*;
use operator_ui::tokens::{Palette, Rgba, typography};

fn hex(c: Rgba) -> String {
    if c.a == 255 {
        format!("#{:02X}{:02X}{:02X}", c.r, c.g, c.b)
    } else {
        format!("#{:02X}{:02X}{:02X}{:02X}", c.r, c.g, c.b, c.a)
    }
}

fn palette_groups(p: &Palette) -> Vec<(&'static str, Vec<(&'static str, Rgba)>)> {
    vec![
        (
            "Surfaces",
            vec![
                ("background", p.background),
                ("surface", p.surface),
                ("surface_alt", p.surface_alt),
                ("hover", p.hover),
                ("pressed", p.pressed),
                ("selection", p.selection),
                ("disabled_surface", p.disabled_surface),
            ],
        ),
        (
            "Lines",
            vec![
                ("border", p.border),
                ("border_strong", p.border_strong),
                ("divider", p.divider),
                ("disabled_border", p.disabled_border),
            ],
        ),
        (
            "Content",
            vec![
                ("text_primary", p.text_primary),
                ("text_secondary", p.text_secondary),
                ("text_disabled", p.text_disabled),
                ("icon", p.icon),
                ("on_accent", p.on_accent),
            ],
        ),
        (
            "Signals",
            vec![
                ("accent", p.accent),
                ("accent_muted", p.accent_muted),
                ("pending", p.pending),
                ("success", p.success),
                ("warning", p.warning),
                ("error", p.error),
                ("info", p.info),
            ],
        ),
    ]
}

pub fn color() -> AnyView {
    let theme = OperatorTheme::current();
    let groups = move || {
        let palette = theme.theme.get().palette(theme.dark.get());
        palette_groups(palette)
            .into_iter()
            .map(|(group, entries)| {
                let swatches = entries
                    .into_iter()
                    .map(|(name, c)| {
                        view! {
                            <div class="demo-swatch">
                                <span class="demo-swatch-chip" style:background=hex(c)></span>
                                <span class="demo-swatch-text op-t-caption">
                                    <span>{name}</span>
                                    <span class="demo-muted">{hex(c)}</span>
                                </span>
                            </div>
                        }
                    })
                    .collect_view();
                view! {
                    <div class="demo-labelled">
                        <span class="op-t-caption demo-muted">{group}</span>
                        <div class="demo-swatches">{swatches}</div>
                    </div>
                }
            })
            .collect_view()
    };
    view! {
        <Example title="Current palette" note="Switch the mode in the top bar to see the other palette." block=true>
            <div class="demo-stack">{groups}</div>
        </Example>
    }
    .into_any()
}

pub fn typography() -> AnyView {
    let rows = [
        ("app_title", "op-t-app-title", typography::APP_TITLE),
        ("screen_title", "op-t-screen-title", typography::SCREEN_TITLE),
        ("section_title", "op-t-section-title", typography::SECTION_TITLE),
        ("row_title", "op-t-row-title", typography::ROW_TITLE),
        ("body", "op-t-body", typography::BODY),
        ("body_small", "op-t-body-small", typography::BODY_SMALL),
        ("caption", "op-t-caption", typography::CAPTION),
        ("console", "op-t-console", typography::CONSOLE),
    ]
    .into_iter()
    .map(|(name, class, t)| {
        view! {
            <div class="demo-type-row">
                <span class="demo-muted op-t-caption">{format!("{name} · {} / {} TU · {}", t.size, t.line, t.weight)}</span>
                <span class=class>"Gate camera 03 · North entrance · 0x4F2A"</span>
            </div>
        }
    })
    .collect_view();
    view! {
        <Example title="Type scale" block=true>
            <div class="demo-types">{rows}</div>
        </Example>
    }
    .into_any()
}

pub fn units() -> AnyView {
    let scale = [
        0.25f32, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 4.0, 6.0, 8.0, 10.0, 12.0,
    ]
    .into_iter()
    .map(|v| {
        view! {
            <div class="demo-du">
                <span class="demo-du-bar" style:width=du(v)></span>
                <span class="op-t-caption demo-muted">{format!("{v} DU")}</span>
            </div>
        }
    })
    .collect_view();
    view! {
        <Example title="DU scale" note="1 DU = 4 logical pixels on the web." block=true>
            <div class="demo-du-scale">{scale}</div>
        </Example>
    }
    .into_any()
}

pub fn iconography() -> AnyView {
    let grid = icons::ALL
        .iter()
        .map(|icon| {
            view! {
                <div class="demo-icon">
                    <Icon icon=*icon size=dim::ICON_LARGE/>
                    <span class="op-t-caption demo-muted">{icon.name}</span>
                </div>
            }
        })
        .collect_view();
    view! {
        <Example title="Icon set" note="Every icon in tokens/icons, rendered at 6 DU." block=true>
            <div class="demo-icons">{grid}</div>
        </Example>
    }
    .into_any()
}

pub fn motion() -> AnyView {
    let run = RwSignal::new(false);
    let rows = [
        ("state_color", motion::STATE_COLOR_MS),
        ("segment_slide", motion::SEGMENT_SLIDE_MS),
        ("dialog_enter", motion::DIALOG_ENTER_MS),
        ("expand", motion::EXPAND_MS),
        ("sheet_slide", motion::SHEET_SLIDE_MS),
        ("overscroll_release", motion::OVERSCROLL_RELEASE_MS),
        ("result_fade", motion::RESULT_FADE_MS),
    ]
    .into_iter()
    .map(|(name, ms)| {
        view! {
            <div class="demo-motion-row">
                <span class="op-t-caption demo-muted">{format!("{name} · {ms} ms")}</span>
                <span class="demo-motion-track">
                    <span
                        class="demo-motion-dot"
                        data-run=move || run.get().then_some("")
                        style:transition-duration=format!("{ms}ms")
                    ></span>
                </span>
            </div>
        }
    })
    .collect_view();
    view! {
        <Example title="Durations" note="Press Play to move each marker with its duration and the standard curve." block=true>
            <div class="demo-stack">
                <div>
                    <ActionButton
                        text="Play"
                        on_click=AsyncAction::new(move |()| async move {
                            run.update(|r| *r = !*r);
                            ActionResult::Success
                        })
                    />
                </div>
                {rows}
            </div>
        </Example>
    }
    .into_any()
}

pub fn states() -> AnyView {
    view! {
        <Example title="State border on one control" note="The same visual language applies to every control.">
            {STATES.into_iter().map(|(name, s)| view! { <Cell label=name><ActionButton text="Apply" state=s/></Cell> }).collect_view()}
            <Cell label="Disabled"><ActionButton text="Apply" enabled=false/></Cell>
        </Example>
    }
    .into_any()
}
