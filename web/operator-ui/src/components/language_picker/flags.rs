//! Built-in SVG flag definitions for LanguagePicker.

use leptos::prelude::*;

/// Returns the inline SVG view for a built-in country code, if recognized.
pub fn builtin_flag_view(code: &str) -> Option<AnyView> {
    let upper = code.to_ascii_uppercase();
    match upper.as_str() {
        "UA" => Some(flag_ua().into_any()),
        "GB" => Some(flag_gb().into_any()),
        "US" => Some(flag_us().into_any()),
        "DE" => Some(flag_de().into_any()),
        "FR" => Some(flag_fr().into_any()),
        "ES" => Some(flag_es().into_any()),
        "IT" => Some(flag_it().into_any()),
        "PL" => Some(flag_pl().into_any()),
        "PT" => Some(flag_pt().into_any()),
        "NL" => Some(flag_nl().into_any()),
        "CZ" => Some(flag_cz().into_any()),
        "SE" => Some(flag_se().into_any()),
        "JP" => Some(flag_jp().into_any()),
        "CN" => Some(flag_cn().into_any()),
        "KR" => Some(flag_kr().into_any()),
        "TR" => Some(flag_tr().into_any()),
        _ => None,
    }
}

fn flag_ua() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="10" fill="#0057B7"/>
            <rect y="10" width="30" height="10" fill="#FFD700"/>
        </svg>
    }
}

fn flag_pl() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="10" fill="#FFFFFF"/>
            <rect y="10" width="30" height="10" fill="#DC143C"/>
        </svg>
    }
}

fn flag_de() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="6.67" fill="#000000"/>
            <rect y="6.67" width="30" height="6.67" fill="#DD0000"/>
            <rect y="13.34" width="30" height="6.66" fill="#FFCE00"/>
        </svg>
    }
}

fn flag_fr() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="10" height="20" fill="#002395"/>
            <rect x="10" width="10" height="20" fill="#FFFFFF"/>
            <rect x="20" width="10" height="20" fill="#ED2939"/>
        </svg>
    }
}

fn flag_it() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="10" height="20" fill="#009246"/>
            <rect x="10" width="10" height="20" fill="#FFFFFF"/>
            <rect x="20" width="10" height="20" fill="#CE2B37"/>
        </svg>
    }
}

fn flag_nl() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="6.67" fill="#AE1C28"/>
            <rect y="6.67" width="30" height="6.67" fill="#FFFFFF"/>
            <rect y="13.34" width="30" height="6.66" fill="#21468B"/>
        </svg>
    }
}

fn flag_se() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="20" fill="#006AA7"/>
            <rect x="9" width="4" height="20" fill="#FECC00"/>
            <rect y="8" width="30" height="4" fill="#FECC00"/>
        </svg>
    }
}

fn flag_cz() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="10" fill="#FFFFFF"/>
            <rect y="10" width="30" height="10" fill="#D7141A"/>
            <polygon points="0,0 15,10 0,20" fill="#11457E"/>
        </svg>
    }
}

fn flag_jp() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="20" fill="#FFFFFF"/>
            <circle cx="15" cy="10" r="6" fill="#BC002D"/>
        </svg>
    }
}

fn flag_es() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="5" fill="#AA151B"/>
            <rect y="5" width="30" height="10" fill="#F1BF00"/>
            <rect y="15" width="30" height="5" fill="#AA151B"/>
            <rect x="6" y="8" width="4" height="4" fill="#AA151B"/>
            <rect x="7" y="9" width="2" height="2" fill="#F1BF00"/>
        </svg>
    }
}

fn flag_gb() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="20" fill="#012169"/>
            <path d="M0,0 L30,20 M30,0 L0,20" stroke="#FFFFFF" stroke-width="3.6"/>
            <path d="M0,0 L30,20 M30,0 L0,20" stroke="#C8102E" stroke-width="1.8"/>
            <rect x="12" width="6" height="20" fill="#FFFFFF"/>
            <rect y="7" width="30" height="6" fill="#FFFFFF"/>
            <rect x="13.2" width="3.6" height="20" fill="#C8102E"/>
            <rect y="8.2" width="30" height="3.6" fill="#C8102E"/>
        </svg>
    }
}

fn flag_us() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="20" fill="#B22234"/>
            <rect y="1.54" width="30" height="1.54" fill="#FFFFFF"/>
            <rect y="4.62" width="30" height="1.54" fill="#FFFFFF"/>
            <rect y="7.69" width="30" height="1.54" fill="#FFFFFF"/>
            <rect y="10.77" width="30" height="1.54" fill="#FFFFFF"/>
            <rect y="13.85" width="30" height="1.54" fill="#FFFFFF"/>
            <rect y="16.92" width="30" height="1.54" fill="#FFFFFF"/>
            <rect width="12" height="10.77" fill="#3C3B6E"/>
            <circle cx="2" cy="2" r="0.6" fill="#FFFFFF"/>
            <circle cx="5" cy="2" r="0.6" fill="#FFFFFF"/>
            <circle cx="8" cy="2" r="0.6" fill="#FFFFFF"/>
            <circle cx="11" cy="2" r="0.6" fill="#FFFFFF"/>
            <circle cx="3.5" cy="4" r="0.6" fill="#FFFFFF"/>
            <circle cx="6.5" cy="4" r="0.6" fill="#FFFFFF"/>
            <circle cx="9.5" cy="4" r="0.6" fill="#FFFFFF"/>
            <circle cx="2" cy="6" r="0.6" fill="#FFFFFF"/>
            <circle cx="5" cy="6" r="0.6" fill="#FFFFFF"/>
            <circle cx="8" cy="6" r="0.6" fill="#FFFFFF"/>
            <circle cx="11" cy="6" r="0.6" fill="#FFFFFF"/>
            <circle cx="3.5" cy="8" r="0.6" fill="#FFFFFF"/>
            <circle cx="6.5" cy="8" r="0.6" fill="#FFFFFF"/>
            <circle cx="9.5" cy="8" r="0.6" fill="#FFFFFF"/>
        </svg>
    }
}

fn flag_cn() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="20" fill="#EE1C25"/>
            <polygon points="5,2 6,4.5 8.5,4.5 6.5,6 7.5,8.5 5,7 2.5,8.5 3.5,6 1.5,4.5 4,4.5" fill="#FFFF00"/>
            <circle cx="10" cy="2.5" r="0.75" fill="#FFFF00"/>
            <circle cx="12" cy="4.5" r="0.75" fill="#FFFF00"/>
            <circle cx="12" cy="7.5" r="0.75" fill="#FFFF00"/>
            <circle cx="10" cy="9.5" r="0.75" fill="#FFFF00"/>
        </svg>
    }
}

fn flag_kr() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="20" fill="#FFFFFF"/>
            <path d="M15,5.5 A4.5,4.5 0 0,1 15,14.5 A2.25,2.25 0 0,1 15,10 A2.25,2.25 0 0,0 15,5.5" fill="#CD2E3A"/>
            <path d="M15,14.5 A4.5,4.5 0 0,1 15,5.5 A2.25,2.25 0 0,1 15,10 A2.25,2.25 0 0,0 15,14.5" fill="#0047A0"/>
            <rect x="4" y="3" width="1" height="3" transform="rotate(-30 4.5 4.5)" fill="#000000"/>
            <rect x="25" y="3" width="1" height="3" transform="rotate(30 25.5 4.5)" fill="#000000"/>
            <rect x="4" y="14" width="1" height="3" transform="rotate(30 4.5 15.5)" fill="#000000"/>
            <rect x="25" y="14" width="1" height="3" transform="rotate(-30 25.5 15.5)" fill="#000000"/>
        </svg>
    }
}

fn flag_tr() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="30" height="20" fill="#E30A17"/>
            <circle cx="12" cy="10" r="5" fill="#FFFFFF"/>
            <circle cx="13.5" cy="10" r="4" fill="#E30A17"/>
            <polygon points="18,7.5 18.8,9.2 20.6,9.2 19.1,10.3 19.7,12 18,11 16.3,12 16.9,10.3 15.4,9.2 17.2,9.2" fill="#FFFFFF"/>
        </svg>
    }
}

fn flag_pt() -> impl IntoView {
    view! {
        <svg viewBox="0 0 30 20" aria-hidden="true">
            <rect width="12" height="20" fill="#006600"/>
            <rect x="12" width="18" height="20" fill="#FF0000"/>
            <circle cx="12" cy="10" r="4" fill="#FFCC00"/>
            <rect x="10.5" y="8" width="3" height="4" fill="#FFFFFF"/>
            <rect x="11" y="8.5" width="2" height="3" fill="#002395"/>
        </svg>
    }
}
