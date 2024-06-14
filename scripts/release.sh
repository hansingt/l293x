#!/usr/bin/env bash
set -e
set -o pipefail

# Determine the project root
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
PROJECT_ROOT="$(cd -- "$SCRIPT_DIR/.." &>/dev/null && pwd)"

# Bail out, if the working tree is not clean
git diff --exit-code "${PROJECT_ROOT}" &>/dev/null || CODE=$?; echo "Worktree is not clean!"; exit $CODE
git diff --cached --exit-code "${PROJECT_ROOT}" &>/dev/null || CODE=$?; echo "Worktree is not clean!"; exit $CODE

# The next version of the project
BUMPED_VERSION="$(git cliff --bumped-version)"
TAG_NAME="v${BUMPED_VERSION}"

# Generate a new changelog
git cliff --bump -o "${PROJECT_ROOT}/CHANGELOG.md"

# Replace the version in Cargo.toml
sed -i "s/\vversion=(\d+\.\d+\.\d+)/version=${BUMPED_VERSION}/g" "${PROJECT_ROOT}/Cargo.toml"

# Commit & push the changelog and version
git add "${PROJECT_ROOT}/Cargo.toml" "${PROJECT_ROOT}/CHANGELOG.md"
git commit -m "build: bump version"
git push

# Create a tag for the new version
git tag "${TAG_NAME}"
git push origin tag "${TAG_NAME}"
