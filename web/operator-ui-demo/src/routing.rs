//! Hash routes and document anchor scrolling.

use crate::docs;

#[derive(Clone, PartialEq, Eq)]
pub struct Route {
    pub slug: String,
    pub anchor: Option<String>,
}

pub fn current_hash() -> String {
    web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_default()
}

pub fn parse_route(hash: &str) -> Route {
    let path = hash.trim_start_matches('#').trim_start_matches('/');
    if path.is_empty() {
        return Route {
            slug: String::new(),
            anchor: None,
        };
    }
    // The longest prefix that names a document is the page; the rest is an anchor.
    let parts: Vec<&str> = path.split('/').collect();
    for n in (1..=parts.len()).rev() {
        let slug = parts[..n].join("/");
        if docs::find(&slug).is_some() {
            let anchor = (n < parts.len()).then(|| parts[n..].join("/"));
            return Route { slug, anchor };
        }
    }
    Route {
        slug: path.to_string(),
        anchor: None,
    }
}

pub fn scroll_to(id: Option<&str>) {
    let Some(window) = web_sys::window() else {
        return;
    };
    // Browsers percent-encode Cyrillic fragments in location.hash.
    let decoded = id.map(|id| {
        js_sys::decode_uri_component(id)
            .ok()
            .and_then(|value| value.as_string())
            .unwrap_or_else(|| id.to_string())
    });
    match decoded.and_then(|id| window.document().and_then(|d| d.get_element_by_id(&id))) {
        Some(el) => el.scroll_into_view(),
        None => window.scroll_to_with_x_and_y(0.0, 0.0),
    }
}
