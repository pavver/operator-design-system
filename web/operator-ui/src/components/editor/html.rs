//! Allowlisted editor HTML; DOMParser keeps parsed documents inert.
use wasm_bindgen::JsCast;
/// Safe URL schemes for generated links, including relative paths and fragments.
pub fn safe_link(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty() || value.chars().any(char::is_control) {
        return false;
    }
    let lower = value.to_ascii_lowercase();
    if let Some((scheme, _)) = lower.split_once(':')
        && !scheme.contains(['/', '#', '?'])
    {
        return matches!(scheme, "http" | "https" | "mailto" | "tel");
    }
    !lower.starts_with(['\\'])
}
/// Strips active content, unsupported elements and attributes from editor HTML.
pub fn sanitize_html(html: &str) -> String {
    let Ok(parser) = web_sys::DomParser::new() else {
        return String::new();
    };
    let Ok(doc) = parser.parse_from_string(html, web_sys::SupportedType::TextHtml) else {
        return String::new();
    };
    let Some(body) = doc.body() else {
        return String::new();
    };
    clean(&body);
    body.inner_html()
}
fn clean(parent: &web_sys::Element) {
    let children = parent.child_nodes();
    let nodes = (0..children.length())
        .filter_map(|i| children.item(i))
        .collect::<Vec<_>>();
    for node in nodes {
        if node.node_type() == web_sys::Node::COMMENT_NODE {
            let _ = parent.remove_child(&node);
            continue;
        }
        let Some(el) = node.dyn_ref::<web_sys::Element>() else {
            continue;
        };
        let tag = el.tag_name().to_ascii_lowercase();
        if matches!(
            tag.as_str(),
            "script"
                | "style"
                | "iframe"
                | "object"
                | "embed"
                | "svg"
                | "math"
                | "template"
                | "link"
                | "meta"
        ) {
            let _ = parent.remove_child(el);
            continue;
        }
        clean(el);
        if !matches!(
            tag.as_str(),
            "p" | "div"
                | "br"
                | "b"
                | "strong"
                | "i"
                | "em"
                | "u"
                | "s"
                | "strike"
                | "ul"
                | "ol"
                | "li"
                | "a"
                | "h1"
                | "h2"
                | "h3"
                | "h4"
                | "h5"
                | "h6"
                | "blockquote"
        ) {
            while let Some(child) = el.first_child() {
                let _ = parent.insert_before(&child, Some(el));
            }
            let _ = parent.remove_child(el);
            continue;
        }
        for name in el
            .get_attribute_names()
            .iter()
            .filter_map(|v| v.as_string())
        {
            if !(tag == "a"
                && name == "href"
                && el.get_attribute("href").is_some_and(|v| safe_link(&v)))
            {
                let _ = el.remove_attribute(&name);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::safe_link;
    #[test]
    fn rejects_script_and_obfuscated_protocols() {
        for url in [
            "javascript:alert(1)",
            "JaVaScRiPt:alert(1)",
            "data:text/html,x",
            "java\nscript:x",
            "\\evil",
        ] {
            assert!(!safe_link(url));
        }
        for url in [
            "https://example.org",
            "mailto:a@example.org",
            "/docs",
            "#section",
            "../notes",
            "notes?x=a:b",
        ] {
            assert!(safe_link(url));
        }
    }
}
