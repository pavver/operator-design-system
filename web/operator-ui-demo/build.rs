//! Renders `docs/**/*.md` into HTML at build time so the gallery pages and the agent-facing
//! Markdown share one source.

use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
use serde::Deserialize;
use std::fmt::Write as _;
use std::path::{Component, Path, PathBuf};
use std::{env, fs};

const REPO_BLOB: &str = "https://github.com/pavver/operator-design-system/blob/main/";

struct Doc {
    slug: String,
    title: String,
    status: Option<String>,
    lead: String,
    body: String,
    toc: Vec<(String, String)>,
}

fn slugify(text: &str) -> String {
    let mut out = String::new();
    let mut dash = false;
    for c in text.chars().flat_map(char::to_lowercase) {
        if c.is_alphanumeric() {
            out.push(c);
            dash = false;
        } else if !dash && !out.is_empty() {
            out.push('-');
            dash = true;
        }
    }
    out.trim_end_matches('-').to_string()
}

fn normalize(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other),
        }
    }
    out
}

/// Rewrites relative links: docs pages become hash routes, other repository files open on GitHub.
fn rewrite_link(dest: &str, file_dir: &Path) -> String {
    if dest.contains("://") || dest.starts_with('#') || dest.starts_with("mailto:") {
        return dest.to_string();
    }
    let (path, anchor) = dest.split_once('#').unwrap_or((dest, ""));
    let repo_path = normalize(&file_dir.join(path));
    let repo = repo_path.to_string_lossy().replace('\\', "/");
    if let Some(slug) = repo
        .strip_prefix("docs/")
        .and_then(|rest| rest.strip_suffix(".md"))
    {
        return if anchor.is_empty() {
            format!("#/{slug}")
        } else {
            format!("#/{slug}/{anchor}")
        };
    }
    format!("{REPO_BLOB}{repo}")
}

fn heading_text(events: &[Event], start: usize) -> String {
    let mut text = String::new();
    for e in &events[start + 1..] {
        match e {
            Event::End(TagEnd::Heading(_)) => break,
            Event::Text(t) | Event::Code(t) => text.push_str(t),
            _ => {}
        }
    }
    text
}

fn render(events: Vec<Event>) -> String {
    let mut out = String::new();
    html::push_html(&mut out, events.into_iter());
    out
}

fn parse(slug: &str, source: &str, file_dir: &Path) -> Doc {
    let parser = Parser::new_ext(
        source,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    );
    let mut events: Vec<Event> = parser.collect();

    let mut title = slug.to_string();
    let mut toc = Vec::new();
    let mut first_h2 = None;
    let mut h1_end = None;
    let mut used = Vec::<String>::new();

    for i in 0..events.len() {
        let level = match &events[i] {
            Event::Start(Tag::Heading { level, .. }) => *level,
            _ => continue,
        };
        let text = heading_text(&events, i);
        if level == HeadingLevel::H1 {
            title = text;
            continue;
        }
        let mut id = slugify(&text);
        let base = id.clone();
        let mut n = 2;
        while used.contains(&id) {
            id = format!("{base}-{n}");
            n += 1;
        }
        used.push(id.clone());
        if level == HeadingLevel::H2 {
            first_h2.get_or_insert(i);
            toc.push((id.clone(), text));
        }
        if let Event::Start(Tag::Heading { id: slot, .. }) = &mut events[i] {
            *slot = Some(CowStr::from(id));
        }
    }
    for (i, e) in events.iter().enumerate() {
        if matches!(e, Event::End(TagEnd::Heading(HeadingLevel::H1))) {
            h1_end = Some(i);
            break;
        }
    }

    for e in &mut events {
        if let Event::Start(Tag::Link { dest_url, .. })
        | Event::Start(Tag::Image { dest_url, .. }) = e
        {
            *dest_url = CowStr::from(rewrite_link(dest_url, file_dir));
        }
    }

    let split = first_h2.unwrap_or(events.len());
    let lead_start = h1_end.map_or(0, |i| i + 1).min(split);
    let body_events = events.split_off(split);
    let lead_events: Vec<Event> = events.into_iter().skip(lead_start).collect();
    let mut lead = render(lead_events);

    let mut status = None;
    if let Some((start, len)) = lead
        .find("<p>Status: ")
        .or_else(|| lead.find("<p>Статус: "))
        .and_then(|start| lead[start..].find("</p>").map(|len| (start, len)))
    {
        let para = lead[start..start + len].to_string();
        if let (Some(a), Some(b)) = (para.find("<strong>"), para.find("</strong>")) {
            status = Some(para[a + 8..b].to_string());
        }
        lead.replace_range(start..start + len + 4, "");
    }

    Doc {
        slug: slug.to_string(),
        title,
        status,
        lead: lead.trim().to_string(),
        body: render(body_events),
        toc,
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct History {
    change: Vec<Change>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Change {
    date: String,
    status: ChangeStatus,
    commit: Option<String>,
    en: String,
    uk: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum ChangeStatus {
    Approved,
    Draft,
}

fn history(file: &Path, dir: &Path) -> Result<String, Box<dyn std::error::Error>> {
    if !file.exists() {
        return Ok(String::new());
    }
    let load = || -> Result<String, Box<dyn std::error::Error>> {
        let history: History = toml::from_str(&fs::read_to_string(file)?)?;
        let mut code = String::new();
        for change in history.change {
            let date: toml::value::Datetime = change.date.parse()?;
            if date.date.is_none() || date.time.is_none() || date.offset.is_none() {
                return Err("change date must include a date, time and UTC offset".into());
            }
            if change.commit.as_ref().is_some_and(|hash| {
                !(7..=40).contains(&hash.len()) || !hash.bytes().all(|c| c.is_ascii_hexdigit())
            }) {
                return Err("change commit must be a 7 to 40 character hexadecimal hash".into());
            }
            let status = match change.status {
                ChangeStatus::Approved => "approved",
                ChangeStatus::Draft => "draft",
            };
            let markdown = |source: &str| {
                let events = Parser::new(source)
                    .map(|event| match event {
                        Event::Start(Tag::Link {
                            link_type,
                            dest_url,
                            title,
                            id,
                        }) => Event::Start(Tag::Link {
                            link_type,
                            dest_url: rewrite_link(&dest_url, dir).into(),
                            title,
                            id,
                        }),
                        other => other,
                    })
                    .collect();
                render(events)
            };
            writeln!(
                code,
                "Change {{ date: {:?}, status: {status:?}, commit: {:?}, en_html: {:?}, uk_html: {:?} }},",
                change.date,
                change.commit,
                markdown(&change.en),
                markdown(&change.uk)
            )?;
        }
        Ok(code)
    };
    load().map_err(|error| format!("{}: {error}", file.display()).into())
}

fn toc_code(doc: &Doc) -> String {
    doc.toc
        .iter()
        .map(|(id, title)| format!("({id:?}, {title:?}),"))
        .collect()
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && path.file_name().is_some_and(|name| name == "i18n") {
            continue;
        }
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo = manifest
        .parent()
        .and_then(Path::parent)
        .ok_or("repository root not found")?
        .to_path_buf();
    let docs = repo.join("docs");
    println!("cargo:rerun-if-changed={}", docs.display());

    let mut files = Vec::new();
    collect(&docs, &mut files);
    files.sort();

    let mut code = String::from("pub static DOCS: &[Doc] = &[\n");
    for file in files {
        println!("cargo:rerun-if-changed={}", file.display());
        let rel = file.strip_prefix(&docs)?.with_extension("");
        let slug = rel.to_string_lossy().replace('\\', "/");
        let dir = file
            .parent()
            .ok_or("no parent")?
            .strip_prefix(&repo)?
            .to_path_buf();
        let doc = parse(&slug, &fs::read_to_string(&file)?, &dir);
        let toc = toc_code(&doc);
        let mirror = docs.join("i18n/uk").join(file.strip_prefix(&docs)?);
        let uk = if mirror.exists() {
            Some(parse(&slug, &fs::read_to_string(&mirror)?, &dir))
        } else {
            None
        };
        let uk_title = uk.as_ref().map(|d| d.title.as_str());
        let uk_lead = uk.as_ref().map(|d| d.lead.as_str());
        let uk_body = uk.as_ref().map(|d| d.body.as_str());
        let uk_toc = uk.as_ref().map_or_else(
            || "None".to_string(),
            |d| format!("Some(&[{}])", toc_code(d)),
        );
        let changes = if slug.starts_with("components/") {
            history(&file.with_extension("changes.toml"), &dir)?
        } else {
            String::new()
        };
        writeln!(
            code,
            "    Doc {{ slug: {:?}, title: {:?}, status: {:?}, lead: {:?}, body: {:?}, toc: &[{toc}], uk_title: {uk_title:?}, uk_lead: {uk_lead:?}, uk_body: {uk_body:?}, uk_toc: {uk_toc}, changes: &[{changes}] }},",
            doc.slug, doc.title, doc.status, doc.lead, doc.body
        )?;
    }
    code.push_str("];\n");
    fs::write(PathBuf::from(env::var("OUT_DIR")?).join("docs.rs"), code)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ukrainian_content_uses_english_link_base_and_removes_status() {
        let doc = parse(
            "components/search-field",
            "# Пошук\n\nВступ.\n\nСтатус: **draft** у вебреалізації.\n\n## Поведінка\n\n[Стани](../foundations/states.md#controlstate).\n",
            Path::new("docs/components"),
        );
        assert_eq!(doc.title, "Пошук");
        assert_eq!(doc.status.as_deref(), Some("draft"));
        assert!(!doc.lead.contains("Статус"));
        assert_eq!(
            doc.toc,
            [("поведінка".to_string(), "Поведінка".to_string())]
        );
        assert!(
            doc.body
                .contains("href=\"#/foundations/states/controlstate\"")
        );
        assert_eq!(
            rewrite_link(
                "../../../docs/components/search-field.md",
                Path::new("docs/components")
            ),
            "#/components/search-field"
        );
    }

    #[test]
    fn mirrors_are_not_routes_and_history_markdown_is_rendered()
    -> Result<(), Box<dyn std::error::Error>> {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs");
        let mut files = Vec::new();
        collect(&repo, &mut files);
        assert!(
            files
                .iter()
                .any(|p| p.ends_with("components/search-field.md"))
        );
        assert!(
            !files
                .iter()
                .any(|p| p.components().any(|c| c.as_os_str() == "i18n"))
        );
        let changes = history(
            &repo.join("components/search-field.changes.toml"),
            Path::new("docs/components"),
        )?;
        assert!(changes.contains("<code>clear_label</code>"));
        assert!(changes.contains("status: \"draft\""));
        assert!(changes.contains("status: \"approved\""));
        assert!(changes.contains("e59d5b4"));
        Ok(())
    }

    #[test]
    fn malformed_history_reports_its_file_name() -> Result<(), Box<dyn std::error::Error>> {
        let file = env::temp_dir().join(format!(
            "ods-invalid-history-{}.changes.toml",
            std::process::id()
        ));
        for source in [
            "this is not TOML",
            "[[change]]\ndate = '2026-11-21T12:00:00Z'\nstatus = 'rejected'\nen = 'Text'\nuk = 'Текст'",
            "[[change]]\ndate = '2026-11-21'\nstatus = 'draft'\nen = 'Text'\nuk = 'Текст'",
            "[[change]]\ndate = '2026-11-21T12:00:00Z'\nstatus = 'draft'\nen = 'Text'",
        ] {
            fs::write(&file, source)?;
            let result = history(&file, Path::new("docs/components"));
            fs::remove_file(&file)?;
            let Err(error) = result else {
                return Err("malformed history was accepted".into());
            };
            assert!(error.to_string().contains(&file.display().to_string()));
        }
        Ok(())
    }
}
