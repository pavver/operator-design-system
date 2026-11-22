#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> Checking CSS for duplicate keyframes..."
duplicates=$(grep -rhoE '@keyframes [a-z0-9-]+' --include=*.css web | sort | uniq -c | awk '$1>1' || true)
if [ -n "$duplicates" ]; then
    echo "FAIL: Duplicate CSS keyframes found:"
    echo "$duplicates"
    exit 1
fi

echo "==> Checking component CSS for raw colors..."
python3 -c '
import re, glob, sys

pattern = re.compile(r"(#[0-9a-fA-F]{3,8}\b|\brgba?\([^)]*\)|\bhsla?\([^)]*\)|(?<=[:\s])\b(?:white|black|red|green|blue|yellow)\b(?=[;\s,]|$))", re.IGNORECASE)

files = sorted(glob.glob("web/operator-ui/src/components/**/*.css", recursive=True))
found = []
for f in files:
    with open(f) as fp:
        lines = fp.readlines()
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        if stripped.startswith("/*") and stripped.endswith("*/"):
            continue
        matches = pattern.findall(line)
        if matches:
            found.append(f"{f}:{i}: {stripped}")

if found:
    print("\n".join(found))
    sys.exit(1)
'

echo "==> All CSS checks passed successfully."
