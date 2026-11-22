#!/usr/bin/env python3
"""Split legacy component histories into one TOML file per change.

Run from any directory; an optional positional argument selects another repository.
Existing identical destinations are safe to reuse after an interrupted migration.
"""

import argparse
import datetime as dt
import json
from pathlib import Path
import re
import tomllib


def quote(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def infer_kind(summary: str) -> str:
    text = summary.lower()
    if re.search(r"new (?:draft )?component", text):
        return "new"
    if any(word in text for word in ("customization", "props", "string hook", "metric")):
        return "feature"
    if re.search(r"\b(fix(?:es|ed)?|correct(?:ed)?|repair(?:ed)?)\b", text) or text.startswith(
        ("remove duplicate", "keep explicit error", "register window listeners")
    ):
        return "fix"
    return "feature"


def change_path(repo: Path, component: str, change: dict) -> Path:
    date = dt.datetime.fromisoformat(change["date"].replace("Z", "+00:00"))
    if date.utcoffset() is None:
        raise ValueError("change date must include a UTC offset")
    words = re.findall(r"[a-z0-9]+", change["en"]["summary"].lower())[:8]
    slug = "-".join(words)[:64].rstrip("-") or "change"
    return repo / "docs/changes" / component / f"{date:%Y-%m-%d-%H%M}-{slug}.toml"


def serialize(change: dict) -> str:
    lines = [f"{key} = {quote(change[key])}" for key in ("date", "status", "kind")]
    if "commit" in change:
        lines.append(f"commit = {quote(change['commit'])}")
    for language in ("en", "uk"):
        lines.extend(("", f"[{language}]"))
        for key in ("summary", "check", "migration"):
            if key in change[language]:
                lines.append(f"{key} = {quote(change[language][key])}")
    return "\n".join(lines) + "\n"


def write_change(repo: Path, component: str, change: dict) -> Path:
    destination = change_path(repo, component, change)
    source = serialize(change)
    base = destination
    suffix = 2
    while destination.exists():
        if destination.read_text() == source:
            return destination
        destination = base.with_stem(f"{base.stem}-{suffix}")
        suffix += 1
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(source)
    return destination


def migrate(repo: Path) -> None:
    legacy = sorted((repo / "docs/components").glob("*.changes.toml"))
    written = []
    for file in legacy:
        component = file.name.removesuffix(".changes.toml")
        if not file.with_name(f"{component}.md").is_file():
            raise ValueError(f"No specification for {file}")
        for entry in tomllib.loads(file.read_text())["change"]:
            change = {key: entry[key] for key in ("date", "status", "commit") if key in entry}
            change["kind"] = infer_kind(entry["en"])
            change.update({language: {"summary": entry[language]} for language in ("en", "uk")})
            written.append(write_change(repo, component, change))
    # Retain every original until all destinations have been written successfully.
    for file in legacy:
        file.unlink()
    print(f"Migrated {len(written)} entries from {len(legacy)} component histories.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("repo", nargs="?", type=Path, default=Path(__file__).resolve().parents[1])
    migrate(parser.parse_args().repo.resolve())
