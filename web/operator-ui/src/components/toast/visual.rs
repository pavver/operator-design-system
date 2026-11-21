use super::*;

/// Wrapped text lines at which a toast switches to the header layout.
const TALL_TOAST_LINES: f64 = 4.0;

/// One notification; ToastHost supplies its lifetime, hover pause and dismissal.
#[component]
pub fn Toast(
    spec: ToastSpec,
    #[prop(optional, into)] on_dismiss: Option<Callback<()>>,
    #[prop(optional, into)] on_hover: Option<Callback<bool>>,
    #[prop(optional, into)] leaving: Signal<bool>,
    #[prop(optional, into)] shift: MaybeProp<f32>,
    #[prop(optional, into)] viewport_margin: MaybeProp<f32>,
    #[prop(optional, into)] gap: MaybeProp<f32>,
    #[prop(optional, into)] width: MaybeProp<f32>,
    #[prop(optional, into)] min_height: MaybeProp<f32>,
    #[prop(optional, into)] padding_y: MaybeProp<f32>,
    #[prop(optional, into)] padding_start: MaybeProp<f32>,
    #[prop(optional, into)] shadow_blur: MaybeProp<f32>,
    #[prop(optional, into)] label_font_size: MaybeProp<f32>,
    #[prop(optional, into)] label_line_height: MaybeProp<f32>,
    #[prop(optional, into)] text_padding_end: MaybeProp<f32>,
    #[prop(optional, into)] button_size: MaybeProp<f32>,
    #[prop(optional, into)] font_size: MaybeProp<f32>,
    #[prop(optional, into)] line_height: MaybeProp<f32>,
    #[prop(optional, into)] icon_size: MaybeProp<f32>,
    #[prop(optional, into)] close_icon_size: MaybeProp<f32>,
    #[prop(optional, into)] letter_spacing: MaybeProp<f32>,
    #[prop(optional, into)] padding_end: MaybeProp<f32>,
    #[prop(optional, into)] shadow_y: MaybeProp<f32>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] header: Slot,
    #[prop(optional, into)] actions: Slot,
    #[prop(optional, into)] footer: Slot,
    #[prop(optional, into)] tone_label: MaybeProp<String>,
    #[prop(optional, into)] dismiss_label: MaybeProp<String>,
    #[prop(optional_no_strip, into)] icon: Option<IconData>,
    #[prop(optional_no_strip, into)] dismiss_icon: Option<IconData>,
) -> impl IntoView {
    let tone = spec.tone;
    let dismiss_label = ui_text(dismiss_label, "toast.dismiss", "Dismiss");
    let dismiss_icon = ui_icon(dismiss_icon, "toast.dismiss", icons::CLOSE);
    let icon = ui_icon(
        icon,
        match tone {
            Tone::Info => "toast.info",
            Tone::Success => "toast.success",
            Tone::Warning => "toast.warning",
            Tone::Error => "toast.error",
        },
        tone.icon(),
    );
    let tone_label = ui_text(
        tone_label,
        match tone {
            Tone::Info => "toast.info",
            Tone::Success => "toast.success",
            Tone::Warning => "toast.warning",
            Tone::Error => "toast.error",
        },
        tone.label(),
    );
    let role = if tone == Tone::Error {
        "alert"
    } else {
        "status"
    };

    // Count wrapped text lines; 4 or more switch the toast to the header layout.
    // The header layout gives the text more width, so it switches back only at 2 lines or fewer.
    let text_ref = NodeRef::<leptos::html::Span>::new();
    let tall = RwSignal::new(false);
    let measure = move || {
        let Some(el) = text_ref.get_untracked() else {
            return;
        };
        let Some(style) = web_sys::window().and_then(|w| w.get_computed_style(&el).ok().flatten())
        else {
            return;
        };
        let px = |name: &str| {
            style
                .get_property_value(name)
                .ok()
                .and_then(|v| v.trim_end_matches("px").parse::<f64>().ok())
                .unwrap_or(0.0)
        };
        let line = px("line-height");
        if line <= 0.0 {
            return;
        }
        let content = f64::from(el.scroll_height()) - px("padding-top") - px("padding-bottom");
        let lines = (content / line).round();
        if lines >= TALL_TOAST_LINES {
            tall.set(true);
        } else if lines <= TALL_TOAST_LINES - 2.0 {
            tall.set(false);
        }
    };
    Effect::new(move |_| {
        let Some(el) = text_ref.get() else {
            return;
        };
        let callback = Closure::<dyn FnMut()>::new(measure);
        let Ok(observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()) else {
            return;
        };
        observer.observe(&el);
        let keep = SendWrapper::new((observer, callback));
        on_cleanup(move || keep.0.disconnect());
    });

    view! {
        <div
            class=root_class("op-toast", class)
            style=root_style(vec![Metric::du("--op-toast-shadow-y", shadow_y),Metric::du("--op-toast-padding-end", padding_end),Metric::du("--op-toast-shift", shift),Metric::du("--op-toast-viewport-margin", viewport_margin),Metric::du("--op-toast-gap", gap),Metric::du("--op-toast-width", width),Metric::du("--op-toast-min-height", min_height),Metric::du("--op-toast-padding-y", padding_y),Metric::du("--op-toast-padding-start", padding_start),Metric::du("--op-toast-shadow-blur", shadow_blur),Metric::tu("--op-toast-label-font-size", label_font_size),Metric::tu("--op-toast-label-line-height", label_line_height),Metric::du("--op-toast-text-padding-end", text_padding_end),Metric::du("--op-toast-button-size", button_size),Metric::tu("--op-toast-font-size", font_size),Metric::tu("--op-toast-line-height", line_height),Metric::du("--op-toast-icon-size", icon_size),Metric::du("--op-toast-close-icon-size", close_icon_size),Metric::tu("--op-toast-letter-spacing", letter_spacing)], style)
            role=role
            data-tone=tone.as_str()
            data-header=header.is_set().then_some("")
            data-tall=move || tall.get().then_some("")
            data-leaving=move || leaving.get().then_some("")
            on:pointerenter=move |_| { if let Some(cb) = on_hover { cb.run(true); } }
            on:pointerleave=move |_| { if let Some(cb) = on_hover { cb.run(false); } }
        >
            <span class="op-toast-icon">{move || view! { <Icon icon=icon.get()/> }}</span>
            {if header.is_set() { view! { <div class="op-toast-header">{header.run()}</div> }.into_any() } else { view! { <span class="op-toast-label" aria-hidden="true">{move || tone_label.get()}</span> }.into_any() }}
            <span node_ref=text_ref class="op-toast-text">{spec.text}</span>
            <div class="op-toast-actions">
            {actions.run()}
            {spec
                .action
                .map(|(label, cb)| {
                    view! {
                        <button
                            type="button"
                            class="op-rippled op-toast-action"
                            on:pointerdown=move |ev| spawn_ripple(&ev)
                            on:click=move |_| {
                                cb.run(());
                                if let Some(cb) = on_dismiss { cb.run(()); }
                            }
                        >
                            {label}
                        </button>
                    }
                })}
            </div>
            {footer.is_set().then(|| view! { <div class="op-toast-footer">{footer.run()}</div> })}
            <button
                type="button"
                class="op-rippled op-toast-close"
                aria-label=move || dismiss_label.get()
                on:pointerdown=move |ev| spawn_ripple(&ev)
                on:click=move |_| { if let Some(cb) = on_dismiss { cb.run(()); } }
            >
                {move || view! { <Icon icon=dismiss_icon.get()/> }}
            </button>
        </div>
    }
}
