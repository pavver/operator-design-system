//! Carousel component: animated slides with navigation arrows, dots, swipe, autoplay, and keyboard control.

use crate::components::primitives::Icon;
use crate::core::{Metric, root_class, root_style, ui_text};
use gloo_timers::future::TimeoutFuture;
use leptos::ev;
use leptos::prelude::*;
use leptos::task::spawn_local;
use operator_ui_tokens::icons;

pub(crate) const CSS: &str = include_str!("carousel.css");

/// Carousel container for cycling through a set of slides.
#[component]
pub fn Carousel(
    current: RwSignal<usize>,
    count: usize,
    #[prop(default = true.into(), into)] arrows: Signal<bool>,
    #[prop(default = true.into(), into)] dots: Signal<bool>,
    #[prop(default = true.into(), into)] infinite: Signal<bool>,
    #[prop(default = false.into(), into)] autoplay: Signal<bool>,
    #[prop(default = 5000.into(), into)] interval_ms: Signal<u64>,
    #[prop(default = true.into(), into)] swipeable: Signal<bool>,
    #[prop(optional, into)] prev_label: MaybeProp<String>,
    #[prop(optional, into)] next_label: MaybeProp<String>,
    #[prop(optional, into)] aria_label: MaybeProp<String>,
    /// Height in DU (`--op-carousel-height`).
    #[prop(optional, into)]
    height: MaybeProp<f32>,
    /// Arrow button size in DU (`--op-carousel-arrow-size`).
    #[prop(optional, into)]
    arrow_size: MaybeProp<f32>,
    /// Dot indicator size in DU (`--op-carousel-dot-size`).
    #[prop(optional, into)]
    dot_size: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let prev_str = ui_text(prev_label, "carousel.prev", "Previous slide");
    let next_str = ui_text(next_label, "carousel.next", "Next slide");
    let carousel_label = ui_text(aria_label, "carousel.label", "Carousel");

    let is_paused = RwSignal::new(false);
    let touch_start_x = RwSignal::new(0.0f64);

    let go_prev = move || {
        let curr = current.get();
        if curr > 0 {
            current.set(curr - 1);
        } else if infinite.get() && count > 0 {
            current.set(count - 1);
        }
    };

    let go_next = move || {
        let curr = current.get();
        if curr + 1 < count {
            current.set(curr + 1);
        } else if infinite.get() {
            current.set(0);
        }
    };

    // Autoplay timer loop
    Effect::new(move |_| {
        if !autoplay.get() || count <= 1 {
            return;
        }
        let interval = interval_ms.get();
        spawn_local(async move {
            loop {
                TimeoutFuture::new(interval as u32).await;
                if !is_paused.get() && autoplay.get() {
                    go_next();
                }
            }
        });
    });

    let on_keydown = move |ev: ev::KeyboardEvent| match ev.key().as_str() {
        "ArrowLeft" => {
            ev.prevent_default();
            go_prev();
        }
        "ArrowRight" => {
            ev.prevent_default();
            go_next();
        }
        _ => {}
    };

    let on_touchstart = move |ev: ev::TouchEvent| {
        if !swipeable.get() {
            return;
        }
        if let Some(touch) = ev.touches().get(0) {
            touch_start_x.set(touch.client_x() as f64);
        }
    };

    let on_touchend = move |ev: ev::TouchEvent| {
        if !swipeable.get() {
            return;
        }
        if let Some(touch) = ev.changed_touches().get(0) {
            let diff = touch_start_x.get() - touch.client_x() as f64;
            if diff > 40.0 {
                go_next();
            } else if diff < -40.0 {
                go_prev();
            }
        }
    };

    let metrics = vec![
        Metric::du("--op-carousel-height", height),
        Metric::du("--op-carousel-arrow-size", arrow_size),
        Metric::du("--op-carousel-dot-size", dot_size),
    ];
    let class_fn = root_class("op-carousel", class);
    let style_fn = root_style(metrics, style);

    let can_prev = Signal::derive(move || infinite.get() || current.get() > 0);
    let can_next = Signal::derive(move || infinite.get() || current.get() + 1 < count);

    view! {
        <div
            class=class_fn
            style=style_fn
            role="region"
            aria-roledescription="carousel"
            aria-label=move || carousel_label.get()
            tabindex="0"
            on:keydown=on_keydown
            on:mouseenter=move |_| is_paused.set(true)
            on:mouseleave=move |_| is_paused.set(false)
            on:focusin=move |_| is_paused.set(true)
            on:focusout=move |_| is_paused.set(false)
            on:touchstart=on_touchstart
            on:touchend=on_touchend
        >
            <div class="op-carousel-viewport">
                <div
                    class="op-carousel-track"
                    style=move || format!("transform: translateX(-{}%);", current.get() * 100)
                >
                    {children()}
                </div>
            </div>

            {move || arrows.get().then(|| view! {
                <button
                    type="button"
                    class="op-carousel-arrow"
                    data-direction="prev"
                    aria-label=move || prev_str.get()
                    disabled=move || !can_prev.get()
                    on:click=move |_| go_prev()
                >
                    <Icon icon=icons::CHEVRON_LEFT size=20.0 />
                </button>
                <button
                    type="button"
                    class="op-carousel-arrow"
                    data-direction="next"
                    aria-label=move || next_str.get()
                    disabled=move || !can_next.get()
                    on:click=move |_| go_next()
                >
                    <Icon icon=icons::CHEVRON_RIGHT size=20.0 />
                </button>
            })}

            {move || (dots.get() && count > 1).then(|| view! {
                <div class="op-carousel-dots" role="tablist">
                    {(0..count).map(|idx| {
                        let is_active = Signal::derive(move || current.get() == idx);
                        view! {
                            <button
                                type="button"
                                class="op-carousel-dot"
                                data-active=move || is_active.get().then_some("")
                                role="tab"
                                aria-selected=move || is_active.get().then_some("true")
                                aria-label=format!("Slide {}", idx + 1)
                                on:click=move |_| current.set(idx)
                            />
                        }
                    }).collect_view()}
                </div>
            })}
        </div>
    }
}

/// A single slide inside a [`Carousel`].
#[component]
pub fn CarouselSlide(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let class_fn = root_class("op-carousel-slide", class);
    let style_fn = root_style(vec![], style);

    view! {
        <div
            class=class_fn
            style=style_fn
            role="group"
            aria-roledescription="slide"
        >
            {children()}
        </div>
    }
}
