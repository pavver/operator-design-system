#!/usr/bin/env python3
"""Backfill per-commit component change files from Git history.

Usage: python3 .tasks/backfill_changes.py <repo-root> --translations <texts.toml>
Existing component change directories are kept.
Translations are keyed by commit hash, with [<hash>.en] and [<hash>.uk] tables.
Each table requires summary and may include check and migration.
Breaking changes require migration in both languages.
Commits without complete bilingual text are reported and skipped.
"""

import argparse
from pathlib import Path
import re
import subprocess
import tomllib

from migrate_change_files import infer_kind, write_change

SKIP_TYPES = {"chore", "style", "ci", "build", "release"}
EXTRA_DIRS = {"value-fields": ["value_field"], "confirm-dialog": ["confirm_dialog"]}
KINDS = {"new", "feature", "fix", "breaking", "docs", "refactor"}


def clean(subject: str) -> str:
    text = re.sub(r"^[a-z]+(?:\([^)]*\))?!?:\s*", "", subject)
    text = text[0].upper() + text[1:] if text else text
    return text.rstrip(".") + "."


def translated_text(record: dict, kind: str) -> bool:
    for language in ("en", "uk"):
        text = record.get(language, {})
        if not isinstance(text, dict) or set(text) - {"summary", "check", "migration"}:
            return False
        if not isinstance(text.get("summary"), str) or not text["summary"].strip():
            return False
        if any(not isinstance(value, str) or not value.strip() for value in text.values()):
            return False
        if kind == "breaking" and not text.get("migration"):
            return False
    return all((key in record["en"]) == (key in record["uk"]) for key in ("check", "migration"))


def backfill(repo: Path, translations: dict) -> None:
    for spec in sorted((repo / "docs/components").glob("*.md")):
        component = spec.stem
        if (repo / "docs/changes" / component).exists():
            continue
        folders = EXTRA_DIRS.get(component, [component.replace("-", "_")])
        paths = [str(spec.relative_to(repo))] + [
            f"web/operator-ui/src/components/{folder}"
            for folder in folders
            if (repo / "web/operator-ui/src/components" / folder).is_dir()
        ]
        result = subprocess.run(
            ["git", "log", "--format=%H%x09%aI%x09%s", "--", *paths],
            cwd=repo, check=True, capture_output=True, text=True,
        )
        for line in result.stdout.splitlines():
            commit, date, subject = line.split("\t", 2)
            conventional = re.match(r"^([a-z]+)(?:\([^)]*\))?(!)?:", subject)
            category = conventional.group(1) if conventional else ""
            if category in SKIP_TYPES:
                continue
            matches = [value for key, value in translations.items() if commit.startswith(key)]
            if len(matches) != 1 or not isinstance(matches[0], dict):
                print(f"Skipped {component} {commit[:7]}: supply one matching bilingual translation.")
                continue
            record = matches[0]
            if set(record) - {"kind", "en", "uk"}:
                raise ValueError(f"Unknown translation fields for {commit}")
            breaking = conventional is not None and conventional.group(2) is not None
            inferred = "breaking" if breaking else (
                {"fix": "fix", "docs": "docs", "refactor": "refactor"}.get(category)
                or infer_kind(clean(subject))
            )
            kind = record.get("kind", inferred)
            if kind not in KINDS or not translated_text(record, kind):
                print(f"Skipped {component} {commit[:7]}: incomplete kind or bilingual texts.")
                continue
            change = {"date": date, "status": "draft", "kind": kind, "commit": commit,
                      "en": record["en"], "uk": record["uk"]}
            destination = write_change(repo, component, change)
            print(destination.relative_to(repo))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("repo", type=Path)
    parser.add_argument("--translations", type=Path, required=True)
    args = parser.parse_args()
    translations = tomllib.loads(args.translations.read_text())
    if any(not re.fullmatch(r"[a-fA-F0-9]{7,40}", key) for key in translations):
        parser.error("translation keys must be 7–40 hexadecimal commit hashes")
    backfill(args.repo.resolve(), {key.lower(): value for key, value in translations.items()})


if __name__ == "__main__":
    main()
