#!/usr/bin/env bash
# Prepares a release: bumps the workspace version, runs the checks, commits and tags.
# Usage: scripts/release.sh X.Y.Z [--push]
set -euo pipefail

version="${1:-}"
push="${2:-}"
if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "usage: scripts/release.sh X.Y.Z [--push]" >&2
  exit 1
fi

cd "$(dirname "$0")/.."

if [[ "$(git rev-parse --abbrev-ref HEAD)" != "main" ]]; then
  echo "releases are made from main" >&2
  exit 1
fi
if [[ -n "$(git status --porcelain)" ]]; then
  echo "working tree is not clean" >&2
  exit 1
fi
if git rev-parse -q --verify "refs/tags/v${version}" >/dev/null; then
  echo "tag v${version} already exists" >&2
  exit 1
fi

# The first `version = ` line in the workspace manifest is [workspace.package].
sed -i "0,/^version = \".*\"/s//version = \"${version}\"/" Cargo.toml
# README examples pin the crate with `tag = "vX.Y.Z"`.
sed -i -E "s/tag = \"v[0-9]+\.[0-9]+\.[0-9]+\"/tag = \"v${version}\"/g" README.md

cargo run -q -p operator-tokens -- --check
cargo fmt --all -- --check
cargo clippy -q -p operator-tokens -p operator-ui-tokens -- -D warnings
cargo clippy -q --target wasm32-unknown-unknown -p operator-ui -p operator-ui-demo -- -D warnings
scripts/check-features.sh
cargo test -q -p operator-ui --lib --features full

git add Cargo.toml Cargo.lock README.md
git commit -q -m "chore(release): v${version}"
git tag -a "v${version}" -m "v${version}"
echo "created commit and tag v${version}"

if [[ "$push" == "--push" ]]; then
  git push origin main "v${version}"
else
  echo "push with: git push origin main v${version}"
fi
