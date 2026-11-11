#!/usr/bin/env bash
# Runs cargo clippy for operator-ui:
# 1. core only (--no-default-features)
# 2. each single component feature (--no-default-features --features <f>)
# 3. group features: reviewed, draft, full
set -euo pipefail

cd "$(dirname "$0")/.."

# Features excluded from the per-feature loop:
# - Group and meta features (tested separately or alias).
# - arborium: requires clang/LLVM with wasm target support, which CI and most users do not have.
EXCLUDE_FEATURES=("default" "reviewed" "draft" "full" "arborium")

echo "==> Checking core (no features)..."
cargo clippy -q --target wasm32-unknown-unknown -p operator-ui --no-default-features -- -D warnings

features=$(python3 -c '
import json, subprocess, sys
out = subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1"])
pkg = next(p for p in json.loads(out)["packages"] if p["name"] == "operator-ui")
excluded = set(sys.argv[1:])
for f in sorted(pkg["features"]):
    if f not in excluded:
        print(f)
' "${EXCLUDE_FEATURES[@]}")

for f in $features; do
    echo "==> Checking feature: $f..."
    cargo clippy -q --target wasm32-unknown-unknown -p operator-ui --no-default-features --features "$f" -- -D warnings
done

for g in reviewed draft full; do
    echo "==> Checking group feature: $g..."
    cargo clippy -q --target wasm32-unknown-unknown -p operator-ui --no-default-features --features "$g" -- -D warnings
done

echo "==> All feature checks passed successfully."
