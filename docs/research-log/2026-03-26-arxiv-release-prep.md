# 2026-03-26 arXiv Release Prep

## Summary

Prepared the repository for a future public arXiv release without changing the anonymous review manuscript itself.

## Changes

- Added `scripts/prepare_arxiv_package.sh` to stage an arXiv bundle under `dist/arxiv-package/`.
- Added `docs/arxiv-author-metadata-template.md` to separate the real blocker from the rest of the public-release work.
- Added `paper/main-public.tex` and `paper/main-body.tex` for a named-public manuscript path.
- Extended `docs/arxiv-release-package.md` with a concrete staging command and output description.
- Updated `paper/README.md` and `README.md` so the arXiv path is discoverable from the main entry points.

## Verification

- the staging script can be run from the repository root
- the remaining gap to a public arXiv release is now primarily author metadata, not package structure
