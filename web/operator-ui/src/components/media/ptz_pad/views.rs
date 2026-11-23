use super::types::*;
use crate::components::icon_button::IconButton;
use crate::components::primitives::Icon;
use crate::core::Slot;
use leptos::prelude::*;
use operator_ui_tokens::dim;
use operator_ui_tokens::icons;

pub fn angle_to_direction(angle_deg: f32, show_diagonals: bool) -> PtzDirection {
    if (-22.5..22.5).contains(&angle_deg) {
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
    }
}

#[component]
pub fn PtzCardinalArrows(current_dir: RwSignal<PtzDirection>) -> impl IntoView {
    view! {
        <div class="op-ptz-arrow op-ptz-arrow-up" data-active=move || current_dir.get().is_up().to_string()>
            <Icon icon=icons::ARROW_UPWARD size=dim::ICON_FIELD />
        </div>
        <div class="op-ptz-arrow op-ptz-arrow-down" data-active=move || current_dir.get().is_down().to_string()>
            <Icon icon=icons::ARROW_DOWNWARD size=dim::ICON_FIELD />
        </div>
        <div class="op-ptz-arrow op-ptz-arrow-left" data-active=move || current_dir.get().is_left().to_string()>
            <Icon icon=icons::ARROW_BACK size=dim::ICON_FIELD />
        </div>
        <div class="op-ptz-arrow op-ptz-arrow-right" data-active=move || current_dir.get().is_right().to_string()>
            <Icon icon=icons::CHEVRON_RIGHT size=dim::ICON_FIELD />
        </div>
    }
}

#[component]
pub fn PtzDiagonalArrows(current_dir: RwSignal<PtzDirection>) -> impl IntoView {
    view! {
        <div class="op-ptz-arrow op-ptz-arrow-up-left" data-active=move || (current_dir.get() == PtzDirection::UpLeft).to_string()>
            <Icon icon=icons::ARROW_UPWARD size=dim::ICON_XSMALL />
        </div>
        <div class="op-ptz-arrow op-ptz-arrow-up-right" data-active=move || (current_dir.get() == PtzDirection::UpRight).to_string()>
            <Icon icon=icons::ARROW_UPWARD size=dim::ICON_XSMALL />
        </div>
        <div class="op-ptz-arrow op-ptz-arrow-down-left" data-active=move || (current_dir.get() == PtzDirection::DownLeft).to_string()>
            <Icon icon=icons::ARROW_UPWARD size=dim::ICON_XSMALL />
        </div>
        <div class="op-ptz-arrow op-ptz-arrow-down-right" data-active=move || (current_dir.get() == PtzDirection::DownRight).to_string()>
            <Icon icon=icons::ARROW_UPWARD size=dim::ICON_XSMALL />
        </div>
    }
}

#[component]
pub fn PtzZoomBar(
    zoom_in_slot: Slot,
    zoom_out_slot: Slot,
    resolved_zoom_in: Signal<String>,
    resolved_zoom_out: Signal<String>,
    enabled: Signal<bool>,
    trigger_zoom_in: Callback<()>,
    trigger_zoom_out: Callback<()>,
) -> impl IntoView {
    view! {
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
    }
}

#[allow(clippy::too_many_arguments)]
pub fn apply_pointer_drag(
    pad: &web_sys::Element,
    client_x: f64,
    client_y: f64,
    show_diagonals: bool,
    offset_x: RwSignal<f32>,
    offset_y: RwSignal<f32>,
    current_dir: RwSignal<PtzDirection>,
    dispatch_command: Callback<PtzCommand>,
) {
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
        let new_dir = angle_to_direction(angle_deg, show_diagonals);

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
}
