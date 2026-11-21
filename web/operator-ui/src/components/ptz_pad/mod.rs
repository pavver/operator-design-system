//! PTZ (Pan/Tilt/Zoom) camera control pad: PtzPad.

mod types;
pub use types::*;

use crate::components::icon_button::IconButton;
use crate::components::primitives::Icon;
use crate::core::{
    AsyncAction, ControlState, JobToken, Metric, Slot, root_class, root_style, sleep_ms, ui_text,
    use_state_visual,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::{icons, motion};

pub(crate) const CSS: &str = include_str!("ptz_pad.css");

/// Camera pan/tilt/zoom controller with press-and-hold gestures, keyboard navigation,
/// and Operator state feedback.
///
/// Features a circular joystick pad (a documented round geometry exception) with
/// 8-way directional control, auto-centering, and optional integrated zoom buttons.
#[component]
pub fn PtzPad(
    /// Callback invoked when a pan/tilt direction and speed `[0.0, 1.0]` is triggered.
    #[prop(optional)]
    on_move: Option<Callback<(PtzDirection, f32)>>,
    /// Callback invoked when a zoom step is triggered.
    #[prop(optional)]
    on_zoom: Option<Callback<PtzZoomDirection>>,
    /// Asynchronous command action handling move and zoom requests.
    #[prop(optional, into)]
    on_async_command: Option<AsyncAction<PtzCommand>>,
    /// Explicit control state override.
    #[prop(optional, into)]
    state: Signal<Option<ControlState>>,
    /// Enabled flag; disabled pads block all interaction.
    #[prop(default = true.into(), into)]
    enabled: Signal<bool>,
    /// Whether to render diagonal directional arrows (default `true`).
    #[prop(default = true)]
    show_diagonals: bool,
    /// Whether to render integrated zoom controls underneath the pad (default `true`).
    #[prop(default = true)]
    show_zoom: bool,
    /// Pad outer diameter in DU (`--op-ptz-pad-size`, default 40 DU).
    #[prop(optional, into)]
    size: MaybeProp<f32>,
    /// Thumb diameter in DU (`--op-ptz-pad-thumb-size`, default 15 DU).
    #[prop(optional, into)]
    thumb_size: MaybeProp<f32>,
    /// Border stroke width in DU (`--op-ptz-pad-border-width`, default 0.25 DU).
    #[prop(optional, into)]
    border_width: MaybeProp<f32>,
    /// Translatable controller label.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Translatable zoom in button label.
    #[prop(optional, into)]
    zoom_in_label: MaybeProp<String>,
    /// Translatable zoom out button label.
    #[prop(optional, into)]
    zoom_out_label: MaybeProp<String>,
    /// Slot to replace the central thumb dot.
    #[prop(optional, into)]
    center_slot: Slot,
    /// Slot to replace the zoom in button.
    #[prop(optional, into)]
    zoom_in_slot: Slot,
    /// Slot to replace the zoom out button.
    #[prop(optional, into)]
    zoom_out_slot: Slot,
    /// Slot for extra controls beneath the pad.
    #[prop(optional, into)]
    extra_controls: Slot,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
) -> impl IntoView {
    let resolved_label = ui_text(label, "ptz-pad.label", "PTZ Camera Control");
    let resolved_zoom_in = ui_text(zoom_in_label, "ptz-pad.zoom-in", "Zoom In");
    let resolved_zoom_out = ui_text(zoom_out_label, "ptz-pad.zoom-out", "Zoom Out");

    let pad_ref = NodeRef::<leptos::html::Div>::new();
    let is_dragging = RwSignal::new(false);
    let offset_x = RwSignal::new(0.0f32);
    let offset_y = RwSignal::new(0.0f32);
    let current_dir = RwSignal::new(PtzDirection::None);
    let local_state = RwSignal::new(ControlState::Idle);
    let job = JobToken::new();

    let root_metrics = vec![
        Metric::du("--op-ptz-pad-size", size),
        Metric::du("--op-ptz-pad-thumb-size", thumb_size),
        Metric::du("--op-ptz-pad-border-width", border_width),
    ];
    let full_style = root_style(root_metrics, style);

    let current = Signal::derive(move || {
        if let Some(s) = state.get() {
            return s;
        }
        if !enabled.get() {
            return ControlState::Disabled;
        }
        let managed = local_state.get();
        if managed != ControlState::Idle {
            return managed;
        }
        ControlState::Idle
    });
    let visual = use_state_visual(current);

    let dispatch_command = Callback::new(move |cmd: PtzCommand| {
        match cmd {
            PtzCommand::Move { direction, speed } => {
                if let Some(cb) = on_move {
                    cb.run((direction, speed));
                }
            }
            PtzCommand::Zoom(z) => {
                if let Some(cb) = on_zoom {
                    cb.run(z);
                }
            }
            PtzCommand::Stop => {
                if let Some(cb) = on_move {
                    cb.run((PtzDirection::None, 0.0));
                }
            }
        }
        if let Some(ref action) = on_async_command {
            let action = action.clone();
            let id = job.next();
            spawn_local(async move {
                local_state.set(ControlState::Pending);
                let result = action.call(cmd).await;
                if !job.is_current(id) {
                    return;
                }
                local_state.set(result.into());
                sleep_ms(motion::RESULT_HOLD_MS).await;
                if job.is_current(id) {
                    local_state.set(ControlState::Idle);
                }
            });
        }
    });

    let reset_thumb = Callback::new(move |()| {
        is_dragging.set(false);
        offset_x.set(0.0);
        offset_y.set(0.0);
        if current_dir.get_untracked().is_active() {
            current_dir.set(PtzDirection::None);
            dispatch_command.run(PtzCommand::Stop);
        }
    });

    let update_pointer = move |client_x: f64, client_y: f64| {
        let Some(pad) = pad_ref.get_untracked() else {
            return;
        };
        let rect = pad.get_bounding_client_rect();
        let center_x = rect.left() + rect.width() / 2.0;
        let center_y = rect.top() + rect.height() / 2.0;

        let dx = (client_x - center_x) as f32;
        let dy = (client_y - center_y) as f32;

        let pad_radius = (rect.width() as f32) / 2.0;
        let thumb_radius = pad_radius * 0.375;
        let max_offset = (pad_radius - thumb_radius).max(1.0);

        let dist = (dx * dx + dy * dy).sqrt();
        let (clamped_x, clamped_y) = if dist <= max_offset {
            (dx, dy)
        } else {
            let factor = max_offset / dist;
            (dx * factor, dy * factor)
        };

        offset_x.set(clamped_x);
        offset_y.set(clamped_y);

        let speed = (dist / max_offset).clamp(0.0, 1.0);
        let deadzone = max_offset * 0.2;

        if dist > deadzone {
            let angle_rad = dy.atan2(dx);
            let angle_deg = angle_rad.to_degrees();

            let new_dir = if (-22.5..22.5).contains(&angle_deg) {
                PtzDirection::Right
            } else if (22.5..67.5).contains(&angle_deg) {
                if show_diagonals {
                    PtzDirection::DownRight
                } else {
                    PtzDirection::Down
                }
            } else if (67.5..112.5).contains(&angle_deg) {
                PtzDirection::Down
            } else if (112.5..157.5).contains(&angle_deg) {
                if show_diagonals {
                    PtzDirection::DownLeft
                } else {
                    PtzDirection::Down
                }
            } else if (-67.5..-22.5).contains(&angle_deg) {
                if show_diagonals {
                    PtzDirection::UpRight
                } else {
                    PtzDirection::Up
                }
            } else if (-112.5..-67.5).contains(&angle_deg) {
                PtzDirection::Up
            } else if (-157.5..-112.5).contains(&angle_deg) {
                if show_diagonals {
                    PtzDirection::UpLeft
                } else {
                    PtzDirection::Up
                }
            } else {
                PtzDirection::Left
            };

            if new_dir != current_dir.get_untracked() {
                current_dir.set(new_dir);
                dispatch_command.run(PtzCommand::Move {
                    direction: new_dir,
                    speed,
                });
            }
        } else if current_dir.get_untracked().is_active() {
            current_dir.set(PtzDirection::None);
            dispatch_command.run(PtzCommand::Stop);
        }
    };

    let thumb_transform = move || {
        let x = offset_x.get();
        let y = offset_y.get();
        format!("transform: translate({x:.1}px, {y:.1}px);")
    };

    let trigger_zoom_in = Callback::new(move |()| {
        if enabled.get() {
            dispatch_command.run(PtzCommand::Zoom(PtzZoomDirection::In));
        }
    });

    let trigger_zoom_out = Callback::new(move |()| {
        if enabled.get() {
            dispatch_command.run(PtzCommand::Zoom(PtzZoomDirection::Out));
        }
    });

    view! {
        <div class="op-ptz-wrapper">
            <div
                node_ref=pad_ref
                class=root_class("op-ptz-pad", class)
                style=full_style
                role="region"
                aria-label=move || resolved_label.get()
                tabindex=move || if enabled.get() { "0" } else { "-1" }
                data-state=move || visual.state.get()
                data-result=move || visual.result.get()
                on:pointerdown=move |ev| {
                    if enabled.get() {
                        is_dragging.set(true);
                        update_pointer(ev.client_x() as f64, ev.client_y() as f64);
                    }
                }
                on:pointermove=move |ev| {
                    if is_dragging.get() {
                        update_pointer(ev.client_x() as f64, ev.client_y() as f64);
                    }
                }
                on:pointerup=move |_| reset_thumb.run(())
                on:pointercancel=move |_| reset_thumb.run(())
                on:pointerleave=move |_| {
                    if is_dragging.get() {
                        reset_thumb.run(());
                    }
                }
                on:keydown=move |ev| {
                    if !enabled.get() {
                        return;
                    }
                    match ev.key().as_str() {
                        "ArrowUp" => {
                            ev.prevent_default();
                            current_dir.set(PtzDirection::Up);
                            offset_y.set(-30.0);
                            offset_x.set(0.0);
                            dispatch_command.run(PtzCommand::Move { direction: PtzDirection::Up, speed: 1.0 });
                        }
                        "ArrowDown" => {
                            ev.prevent_default();
                            current_dir.set(PtzDirection::Down);
                            offset_y.set(30.0);
                            offset_x.set(0.0);
                            dispatch_command.run(PtzCommand::Move { direction: PtzDirection::Down, speed: 1.0 });
                        }
                        "ArrowLeft" => {
                            ev.prevent_default();
                            current_dir.set(PtzDirection::Left);
                            offset_x.set(-30.0);
                            offset_y.set(0.0);
                            dispatch_command.run(PtzCommand::Move { direction: PtzDirection::Left, speed: 1.0 });
                        }
                        "ArrowRight" => {
                            ev.prevent_default();
                            current_dir.set(PtzDirection::Right);
                            offset_x.set(30.0);
                            offset_y.set(0.0);
                            dispatch_command.run(PtzCommand::Move { direction: PtzDirection::Right, speed: 1.0 });
                        }
                        "+" | "=" => {
                            ev.prevent_default();
                            trigger_zoom_in.run(());
                        }
                        "-" | "_" => {
                            ev.prevent_default();
                            trigger_zoom_out.run(());
                        }
                        _ => {}
                    }
                }
                on:keyup=move |ev| {
                    match ev.key().as_str() {
                        "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" => {
                            reset_thumb.run(());
                        }
                        _ => {}
                    }
                }
            >
                // Cardinal arrows
                <div class="op-ptz-arrow op-ptz-arrow-up" data-active=move || current_dir.get().is_up().to_string()>
                    <Icon icon=icons::ARROW_UPWARD size=5.0 />
                </div>
                <div class="op-ptz-arrow op-ptz-arrow-down" data-active=move || current_dir.get().is_down().to_string()>
                    <Icon icon=icons::ARROW_DOWNWARD size=5.0 />
                </div>
                <div class="op-ptz-arrow op-ptz-arrow-left" data-active=move || current_dir.get().is_left().to_string()>
                    <Icon icon=icons::ARROW_BACK size=5.0 />
                </div>
                <div class="op-ptz-arrow op-ptz-arrow-right" data-active=move || current_dir.get().is_right().to_string()>
                    <Icon icon=icons::CHEVRON_RIGHT size=5.0 />
                </div>

                // Diagonal arrows
                {show_diagonals.then(|| view! {
                    <div class="op-ptz-arrow op-ptz-arrow-up-left" data-active=move || (current_dir.get() == PtzDirection::UpLeft).to_string()>
                        <Icon icon=icons::ARROW_UPWARD size=3.5 />
                    </div>
                    <div class="op-ptz-arrow op-ptz-arrow-up-right" data-active=move || (current_dir.get() == PtzDirection::UpRight).to_string()>
                        <Icon icon=icons::ARROW_UPWARD size=3.5 />
                    </div>
                    <div class="op-ptz-arrow op-ptz-arrow-down-left" data-active=move || (current_dir.get() == PtzDirection::DownLeft).to_string()>
                        <Icon icon=icons::ARROW_UPWARD size=3.5 />
                    </div>
                    <div class="op-ptz-arrow op-ptz-arrow-down-right" data-active=move || (current_dir.get() == PtzDirection::DownRight).to_string()>
                        <Icon icon=icons::ARROW_UPWARD size=3.5 />
                    </div>
                })}

                // Thumb
                <div class="op-ptz-thumb" style=thumb_transform>
                    {if center_slot.is_set() {
                        center_slot.run()
                    } else {
                        Some(view! { <div class="op-ptz-dot"></div> }.into_any())
                    }}
                </div>
            </div>

            // Zoom controls
            {show_zoom.then(|| view! {
                <div class="op-ptz-zoom-bar">
                    {if zoom_out_slot.is_set() {
                        zoom_out_slot.run()
                    } else {
                        Some(view! {
                            <IconButton
                                icon=icons::REMOVE
                                label=resolved_zoom_out
                                enabled=enabled
                                on_click=Callback::new(move |()| trigger_zoom_out.run(()))
                            />
                        }.into_any())
                    }}
                    {if zoom_in_slot.is_set() {
                        zoom_in_slot.run()
                    } else {
                        Some(view! {
                            <IconButton
                                icon=icons::ADD
                                label=resolved_zoom_in
                                enabled=enabled
                                on_click=Callback::new(move |()| trigger_zoom_in.run(()))
                            />
                        }.into_any())
                    }}
                </div>
            })}

            {extra_controls.is_set().then(|| extra_controls.run())}
        </div>
    }
}
