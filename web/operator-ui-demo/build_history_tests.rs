//! Host coverage for change schemas, ordering, rendering, and optional Git links.

use super::*;

const VALID: &str = "date = '2026-11-21T12:00:00+02:00'\nstatus = 'draft'\nkind = 'feature'\n[en]\nsummary = 'Text with `code`'\ncheck = '[Field](field.md)'\n[uk]\nsummary = 'Текст'\n";

struct Fixture(std::path::PathBuf);

impl Fixture {
    fn new(name: &str) -> Result<Self, Error> {
        let dir = std::env::temp_dir().join(format!("ods-history-{name}-{}", std::process::id()));
        fs::create_dir_all(&dir)?;
        Ok(Self(dir))
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn schema_rejects_unknown_fields_and_incomplete_changes() {
    assert!(parse_change(VALID).is_ok());
    for source in [
        "this is not TOML".to_string(),
        VALID.replace("draft", "rejected"),
        VALID.replace("feature", "other"),
        VALID.replace("2026-11-21T12:00:00+02:00", "2026-11-21"),
        VALID.replace("2026-11-21T12:00:00+02:00", "2026-11-21T12:00:00"),
        VALID.replace("status = 'draft'\n", ""),
        VALID.replace("kind = 'feature'\n", ""),
        VALID.replace("summary = 'Text with `code`'\n", ""),
        VALID.replace("[uk]\nsummary = 'Текст'\n", ""),
        VALID.replace("summary = 'Текст'", "summary = ''"),
        VALID.replace("[en]", "extra = true\n[en]"),
        VALID.replace("[en]", "commit = 'xyz1234'\n[en]"),
        VALID.replace("[en]", "commit = 'abcdef'\n[en]"),
        VALID.replace("[en]", "commit = '12345678901234567890123456789012345678901ab'\n[en]"),
        VALID.replace("[en]", "[en]\nextra = true"),
        VALID.replace("[uk]", "[uk]\nextra = true"),
        VALID.replace("feature", "breaking"),
    ] {
        assert!(parse_change(&source).is_err(), "accepted: {source}");
    }
    for hash in ["AbC1234", "1234567890abcdef1234567890abcdef12345678"] {
        assert!(parse_change(&VALID.replace("[en]", &format!("commit = '{hash}'\n[en]"))).is_ok());
    }
}

#[test]
fn breaking_changes_require_migration_in_both_languages() -> Result<(), Error> {
    let source = VALID.replace("feature", "breaking");
    let source = source.replace("[en]", "[en]\nmigration = 'Rename `old` to `new`'");
    assert!(parse_change(&source).is_err());
    let source = source.replace("[uk]", "[uk]\nmigration = 'Перейменуйте `old` на `new`'");
    let (change, _) = parse_change(&source)?;
    let migration = markdown(
        change.en.migration.as_deref().unwrap_or(""),
        Path::new("docs/components"),
    );
    assert!(migration.contains("<code>new</code>"));
    assert!(parse_change(&source.replace("Перейменуйте `old` на `new`", " ")).is_err());
    Ok(())
}

#[test]
fn dates_are_sorted_by_instant_including_fractional_seconds() -> Result<(), Error> {
    assert!(date_key("2026-11-21T12:00:00+02:00")? < date_key("2026-11-21T11:00:00Z")?);
    assert_eq!(
        date_key("2026-11-22T01:00:00+02:00")?,
        date_key("2026-11-21T23:00:00Z")?
    );
    assert!(date_key("2024-02-29T23:59:59Z")? < date_key("2024-03-01T00:00:00Z")?);
    assert!(date_key("2026-11-21T12:00:00.1Z")? < date_key("2026-11-21T12:00:00.2Z")?);
    Ok(())
}

#[test]
fn directory_history_sorts_renders_details_and_tolerates_missing_git_history() -> Result<(), Error> {
    let fixture = Fixture::new("directory")?;
    let dir = fixture.0.join("changes/search-field");
    let base = Path::new("docs/components");
    assert!(history(&dir, base, &fixture.0)?.is_empty());
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("2026-11-21-1200-old.toml"), VALID)?;
    let source = VALID
        .replace("2026-11-21T12:00:00+02:00", "2026-11-21T11:00:00Z")
        .replace("feature", "breaking")
        .replace("[en]", "[en]\nmigration = 'Rename `old` to `new`'")
        .replace("[uk]", "[uk]\ncheck = 'Перевірте поле'\nmigration = 'Змініть виклик'");
    fs::write(dir.join("2026-11-21-1100-new.toml"), source)?;
    let code = history(&dir, base, &fixture.0)?;
    let new = code.find("2026-11-21T11:00:00Z").ok_or("missing newest change")?;
    let old = code.find("2026-11-21T12:00:00+02:00").ok_or("missing oldest change")?;
    assert!(new < old);
    assert!(code.contains("commit: None"));
    assert!(code.contains("kind: \"breaking\""));
    assert!(code.contains("en_check_html: Some(\"<p><a href=\\\"#/components/field\\\""));
    assert!(code.contains("uk_check_html: Some(\"<p>Перевірте поле"));
    assert!(code.contains("en_migration_html: Some(\"<p>Rename <code>old</code>"));
    assert!(code.contains("uk_migration_html: Some(\"<p>Змініть виклик"));
    Ok(())
}

#[test]
fn malformed_history_reports_its_file_name() -> Result<(), Error> {
    let fixture = Fixture::new("invalid")?;
    let file = fixture.0.join("2026-11-21-1200-invalid.toml");
    fs::write(&file, "invalid TOML")?;
    let result = history(&fixture.0, Path::new("docs/components"), &fixture.0);
    let Err(error) = result else {
        return Err("malformed history was accepted".into());
    };
    assert!(error.to_string().contains(&file.display().to_string()));
    Ok(())
}

#[test]
fn optional_commit_uses_the_adding_commit_and_skips_shallow_history() -> Result<(), Error> {
    let fixture = Fixture::new("git")?;
    let repo = &fixture.0;
    // Source archives can be checked without a Git installation.
    if git(repo, &["--version"]).is_none() {
        return Ok(());
    }
    git(repo, &["init", "--quiet"]).ok_or("git init failed")?;
    let file = repo.join("2026-11-21-1200-feature.toml");
    fs::write(&file, VALID)?;
    git(repo, &["add", "."]).ok_or("git add failed")?;
    let commit = || -> Result<(), Error> {
        git(repo, &[
            "-c", "user.name=History fixture", "-c", "user.email=history@example.invalid",
            "-c", "core.hooksPath=/dev/null", "-c", "commit.gpgsign=false",
            "commit", "--quiet", "-am", "fixture",
        ]).ok_or("git commit failed")?;
        Ok(())
    };
    commit()?;
    let added = git(repo, &["rev-parse", "--short", "HEAD"]).ok_or("missing commit")?;
    assert_eq!(added_commit(repo, &file), Some(added.clone()));
    fs::write(&file, VALID.replace("feature", "fix"))?;
    commit()?;
    assert_eq!(added_commit(repo, &file), Some(added));
    assert_eq!(added_commit(repo, &repo.join("untracked.toml")), None);
    let head = git(repo, &["rev-parse", "HEAD"]).ok_or("missing HEAD")?;
    fs::write(repo.join(".git/shallow"), format!("{head}\n"))?;
    assert_eq!(added_commit(repo, &file), None);
    Ok(())
}
