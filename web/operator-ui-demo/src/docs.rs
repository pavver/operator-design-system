//! Documentation pages rendered from `docs/**/*.md` by `build.rs`.

use crate::i18n::Lang;

pub struct Change {
    pub date: &'static str,
    pub sort_key: (i64, u32),
    pub status: &'static str,
    pub kind: &'static str,
    pub commit: Option<&'static str>,
    pub en_html: &'static str,
    pub uk_html: &'static str,
    pub en_check_html: Option<&'static str>,
    pub uk_check_html: Option<&'static str>,
    pub en_migration_html: Option<&'static str>,
    pub uk_migration_html: Option<&'static str>,
}

/// One English document and its optional Ukrainian mirror.
pub struct Doc {
    pub slug: &'static str,
    pub title: &'static str,
    /// The English `Status:` line remains the source of truth.
    pub status: Option<&'static str>,
    pub lead: &'static str,
    pub body: &'static str,
    pub toc: &'static [(&'static str, &'static str)],
    pub uk_title: Option<&'static str>,
    pub uk_lead: Option<&'static str>,
    pub uk_body: Option<&'static str>,
    pub uk_toc: Option<&'static [(&'static str, &'static str)]>,
    pub changes: &'static [Change],
}

impl Doc {
    pub fn title(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::Uk => self.uk_title.unwrap_or(self.title),
            Lang::En => self.title,
        }
    }

    pub fn lead(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::Uk => self.uk_lead.unwrap_or(self.lead),
            Lang::En => self.lead,
        }
    }

    pub fn body(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::Uk => self.uk_body.unwrap_or(self.body),
            Lang::En => self.body,
        }
    }

    pub fn toc(&self, lang: Lang) -> &'static [(&'static str, &'static str)] {
        match lang {
            Lang::Uk => self.uk_toc.unwrap_or(self.toc),
            Lang::En => self.toc,
        }
    }

    pub fn draft_changes(&self) -> usize {
        self.changes.iter().filter(|c| c.status == "draft").count()
    }
}

include!(concat!(env!("OUT_DIR"), "/docs.rs"));

pub fn find(slug: &str) -> Option<&'static Doc> {
    DOCS.iter().find(|d| d.slug == slug)
}
