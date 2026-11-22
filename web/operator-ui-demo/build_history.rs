//! Strict parsing and code generation for per-commit component change files.

use super::{render, rewrite_link};
use pulldown_cmark::{Event, Parser, Tag};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{fs, io};

type Error = Box<dyn std::error::Error>;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Change {
    date: String,
    status: ChangeStatus,
    kind: ChangeKind,
    commit: Option<String>,
    en: Text,
    uk: Text,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Text {
    summary: String,
    check: Option<String>,
    migration: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum ChangeStatus {
    Approved,
    Draft,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum ChangeKind {
    New,
    Feature,
    Fix,
    Breaking,
    Docs,
    Refactor,
}

impl ChangeKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Feature => "feature",
            Self::Fix => "fix",
            Self::Breaking => "breaking",
            Self::Docs => "docs",
            Self::Refactor => "refactor",
        }
    }
}

fn valid_commit(hash: &str) -> bool {
    (7..=40).contains(&hash.len()) && hash.bytes().all(|c| c.is_ascii_hexdigit())
}

/// A UTC sorting key, including fractions; comparing RFC 3339 strings ignores offsets.
fn date_key(source: &str) -> Result<(i64, u32), Error> {
    let parsed: toml::value::Datetime = source.parse()?;
    let (Some(date), Some(time), Some(offset)) = (parsed.date, parsed.time, parsed.offset) else {
        return Err("change date must include a date, time and UTC offset".into());
    };
    let year = i64::from(date.year);
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let days = 365 * year + (year + 3) / 4 - (year + 99) / 100
        + (year + 399) / 400
        + months[..usize::from(date.month - 1)].iter().sum::<i64>()
        + i64::from(date.day - 1)
        + i64::from(leap && date.month > 2);
    let offset = match offset {
        toml::value::Offset::Z => 0,
        toml::value::Offset::Custom { minutes } => i64::from(minutes) * 60,
    };
    Ok((
        days * 86_400
            + i64::from(time.hour) * 3_600
            + i64::from(time.minute) * 60
            + i64::from(time.second)
            - offset,
        time.nanosecond,
    ))
}

fn parse_change(source: &str) -> Result<(Change, (i64, u32)), Error> {
    let change: Change = toml::from_str(source)?;
    let key = date_key(&change.date)?;
    if change
        .commit
        .as_deref()
        .is_some_and(|hash| !valid_commit(hash))
    {
        return Err("change commit must be a 7 to 40 character hexadecimal hash".into());
    }
    for (language, text) in [("en", &change.en), ("uk", &change.uk)] {
        if text.summary.trim().is_empty() {
            return Err(format!("{language}.summary must not be empty").into());
        }
        if matches!(change.kind, ChangeKind::Breaking)
            && text
                .migration
                .as_deref()
                .is_none_or(|s| s.trim().is_empty())
        {
            return Err(format!("{language}.migration is required for breaking changes").into());
        }
    }
    Ok((change, key))
}

fn git(repo: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .current_dir(repo)
        .args(args)
        .stderr(Stdio::null())
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub(super) type AddedCommits = HashMap<String, Option<String>>;

pub(super) fn added_commits(repo: &Path) -> AddedCommits {
    let mut commits = AddedCommits::new();
    // A shallow boundary can masquerade as the addition of every file in that commit.
    if git(repo, &["rev-parse", "--is-shallow-repository"]).as_deref() != Some("false") {
        return commits;
    }
    // Read all additions and component edits together, once for the entire build.
    let Some(log) = git(
        repo,
        &[
            "log",
            "--full-history",
            "--no-color",
            "--no-ext-diff",
            "--no-renames",
            "--format=%x00%h",
            "--name-status",
            "--",
            "docs/changes",
            "docs/components",
            "web/operator-ui/src/components",
        ],
    ) else {
        return commits;
    };
    for entry in log.split('\0') {
        let mut lines = entry.lines();
        let Some(hash) = lines.next().filter(|hash| valid_commit(hash)) else {
            continue;
        };
        let paths: Vec<_> = lines.filter_map(|line| line.split_once('\t')).collect();
        for &(status, path) in &paths {
            if status != "A" {
                continue;
            }
            let Some((component, _)) = path
                .strip_prefix("docs/changes/")
                .and_then(|path| path.split_once('/'))
            else {
                continue;
            };
            // This grouped page documents the value_field module; primitives has its own folder.
            let module = match component {
                "value-fields" => "value_field".to_string(),
                name => name.replace('-', "_"),
            };
            let implementation = format!("web/operator-ui/src/components/{module}/");
            let specification = format!("docs/components/{component}.md");
            let touched = paths
                .iter()
                .any(|(_, path)| path.starts_with(&implementation) || *path == specification);
            // Keep even rejected additions so an older incarnation cannot supply a link.
            commits
                .entry(path.to_string())
                .or_insert_with(|| touched.then(|| hash.to_string()));
        }
    }
    commits
}

pub(super) fn watch_git(repo: &Path) {
    let branch = git(repo, &["symbolic-ref", "-q", "HEAD"]);
    for name in ["HEAD", "logs/HEAD", "shallow", "packed-refs"]
        .into_iter()
        .chain(branch.as_deref())
    {
        if let Some(path) = git(repo, &["rev-parse", "--git-path", name]) {
            let path = repo.join(path);
            if path.is_file() {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}

fn markdown(source: &str, dir: &Path) -> String {
    render(
        Parser::new(source)
            .map(|mut event| {
                if let Event::Start(Tag::Link { dest_url, .. })
                | Event::Start(Tag::Image { dest_url, .. }) = &mut event
                {
                    *dest_url = rewrite_link(dest_url, dir).into();
                }
                event
            })
            .collect(),
    )
}

pub(super) fn validate_components(docs: &Path) -> Result<(), Error> {
    let dir = docs.join("changes");
    if !dir.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("invalid component name")?;
        if !path.is_dir() || !docs.join("components").join(format!("{name}.md")).is_file() {
            return Err(format!(
                "{}: expected a component change directory with a specification",
                path.display()
            )
            .into());
        }
    }
    Ok(())
}

/// Relative Markdown links keep the component specification as their base.
pub(super) fn history(
    dir: &Path,
    link_base: &Path,
    repo: &Path,
    commits: &AddedCommits,
) -> Result<String, Error> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(String::new()),
        Err(error) => return Err(format!("{}: {error}", dir.display()).into()),
    };
    let mut changes = Vec::new();
    for entry in entries {
        let file = entry?.path();
        if file.extension().is_none_or(|extension| extension != "toml") || !file.is_file() {
            return Err(format!("{}: expected a TOML change file", file.display()).into());
        }
        let load = || -> Result<_, Error> {
            let (mut change, key) = parse_change(&fs::read_to_string(&file)?)?;
            if change.commit.is_none() {
                change.commit = file
                    .strip_prefix(repo)
                    .ok()
                    .and_then(Path::to_str)
                    .and_then(|path| commits.get(path))
                    .cloned()
                    .flatten();
            }
            Ok((key, file.clone(), change))
        };
        changes.push(load().map_err(|error| format!("{}: {error}", file.display()))?);
    }
    changes.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    let mut code = String::new();
    for (key, _, change) in changes {
        let status = match change.status {
            ChangeStatus::Approved => "approved",
            ChangeStatus::Draft => "draft",
        };
        writeln!(
            code,
            "Change {{ date: {:?}, sort_key: {key:?}, status: {status:?}, kind: {:?}, commit: {:?}, en_html: {:?}, uk_html: {:?}, en_check_html: {:?}, uk_check_html: {:?}, en_migration_html: {:?}, uk_migration_html: {:?} }},",
            change.date,
            change.kind.as_str(),
            change.commit,
            markdown(&change.en.summary, link_base),
            markdown(&change.uk.summary, link_base),
            change.en.check.as_deref().map(|s| markdown(s, link_base)),
            change.uk.check.as_deref().map(|s| markdown(s, link_base)),
            change
                .en
                .migration
                .as_deref()
                .map(|s| markdown(s, link_base)),
            change
                .uk
                .migration
                .as_deref()
                .map(|s| markdown(s, link_base)),
        )?;
    }
    Ok(code)
}

#[cfg(test)]
#[path = "build_history_tests.rs"]
mod tests;
