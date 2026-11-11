//! Documentation pages rendered from `docs/**/*.md` by `build.rs`.

/// One Markdown document from `docs/`.
pub struct Doc {
    /// Path under `docs/` without the extension, for example `components/switch`.
    pub slug: &'static str,
    pub title: &'static str,
    /// `reviewed` or `draft` from the `Status:` line.
    pub status: Option<&'static str>,
    /// HTML of the paragraphs between the title and the first section.
    pub lead: &'static str,
    /// HTML from the first `##` section to the end.
    pub body: &'static str,
    /// Second-level headings as `(id, text)`.
    pub toc: &'static [(&'static str, &'static str)],
}

include!(concat!(env!("OUT_DIR"), "/docs.rs"));

pub fn find(slug: &str) -> Option<&'static Doc> {
    DOCS.iter().find(|d| d.slug == slug)
}
