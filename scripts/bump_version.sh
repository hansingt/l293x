#!/usr/bin/env bash
set -euo pipefail

# Determine the project root
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
PROJECT_ROOT="$(cd -- "$SCRIPT_DIR/.." &>/dev/null && pwd)"

# The next version of the project
TAG_NAME="$(git cliff --bumped-version)"
NEXT_VERSION="${TAG_NAME/v/}"

# Generate a new changelog
echo "Generating changelog"
git cliff --bump -o "${PROJECT_ROOT}/CHANGELOG.md"

# Replace the version in Cargo.toml
echo "Updating Cargo.toml version"
sed -i "s/^version\(\s*\)=\(\s*\)\([\"']\?\)[0-9]\+\.[0-9]\+\.[0-9]\+\(-[a-z]\+[0-9]\+\)\?\([\"']\?\)$/version\1=\2\3${NEXT_VERSION}\5/g" "${PROJECT_ROOT}/Cargo.toml"

echo "Version bumped to ${NEXT_VERSION}"
