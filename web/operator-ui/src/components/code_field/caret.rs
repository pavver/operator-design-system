//! Screen position of the caret in a textarea, measured with a hidden mirror element.

use wasm_bindgen::JsCast;

/// Screen rectangle `(left, top, width, height)` of the caret at UTF-16 offset `caret`.
///
/// The mirror copies the textarea's box and typography into an unscaled element in `<body>`, so
/// the result also holds for textareas inside a zoomed canvas.
pub fn caret_rect(
    textarea: &web_sys::HtmlTextAreaElement,
    caret: u32,
) -> Option<(f64, f64, f64, f64)> {
    let window = web_sys::window()?;
    let document = window.document()?;
    let computed = window.get_computed_style(textarea).ok().flatten()?;
    let div = document.create_element("div").ok()?;
    let style = div.dyn_ref::<web_sys::HtmlElement>()?.style();
    for (name, value) in [
        ("position", "absolute"),
        ("visibility", "hidden"),
        ("top", "0"),
        ("left", "0"),
        ("white-space", "pre-wrap"),
        ("overflow-wrap", "anywhere"),
        ("overflow", "hidden"),
        ("border-style", "solid"),
        ("border-color", "transparent"),
    ] {
        let _ = style.set_property(name, value);
    }
    for name in [
        "box-sizing",
        "width",
        "font-family",
        "font-size",
        "font-weight",
        "font-style",
        "letter-spacing",
        "line-height",
        "tab-size",
        "padding-top",
        "padding-right",
        "padding-bottom",
        "padding-left",
        "border-top-width",
        "border-right-width",
        "border-bottom-width",
        "border-left-width",
    ] {
        let _ = style.set_property(name, &computed.get_property_value(name).unwrap_or_default());
    }
    let text = textarea.value();
    let units: Vec<u16> = text.encode_utf16().take(caret as usize).collect();
    let before = String::from_utf16_lossy(&units);
    let _ = div.append_child(&document.create_text_node(&before));
    let marker = document.create_element("span").ok()?;
    marker.set_text_content(Some("\u{200b}"));
    let _ = div.append_child(&marker);
    document.body()?.append_child(&div).ok()?;

    let ta = textarea.get_bounding_client_rect();
    let width = f64::from(textarea.offset_width());
    let scale = if width > 0.0 { ta.width() / width } else { 1.0 };
    let d = div.get_bounding_client_rect();
    let m = marker.get_bounding_client_rect();
    div.remove();
    Some((
        ta.left() + (m.left() - d.left()) * scale - f64::from(textarea.scroll_left()) * scale,
        ta.top() + (m.top() - d.top()) * scale - f64::from(textarea.scroll_top()) * scale,
        1.0,
        m.height() * scale,
    ))
}
