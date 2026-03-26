#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$ROOT_DIR/dist/arxiv-package"
PAPER_DIR="$OUT_DIR/paper"
FIGURE_DIR="$OUT_DIR/figures"
DOC_DIR="$OUT_DIR/docs"

rm -rf "$OUT_DIR"
mkdir -p "$PAPER_DIR" "$FIGURE_DIR" "$DOC_DIR"

cp "$ROOT_DIR/paper/main.tex" "$PAPER_DIR/"
cp "$ROOT_DIR/paper/references.bib" "$PAPER_DIR/"

if [[ -f "$ROOT_DIR/paper/main.pdf" ]]; then
  cp "$ROOT_DIR/paper/main.pdf" "$PAPER_DIR/"
fi

if [[ -f "$ROOT_DIR/paper/appendix.tex" ]]; then
  cp "$ROOT_DIR/paper/appendix.tex" "$PAPER_DIR/"
fi

cp "$ROOT_DIR/docs/arxiv-release-package.md" "$DOC_DIR/"
cp "$ROOT_DIR/docs/public-facing-summary.md" "$DOC_DIR/"
cp "$ROOT_DIR/docs/arxiv-author-metadata-template.md" "$DOC_DIR/"

while IFS= read -r fig; do
  [[ -z "$fig" ]] && continue
  cp "$ROOT_DIR/figures/$fig" "$FIGURE_DIR/"
done < <(
  rg --no-filename -o '\.\./figures/[A-Za-z0-9._-]+' "$ROOT_DIR/paper/main.tex" "$ROOT_DIR/paper/appendix.tex" 2>/dev/null \
    | sed 's#^\.\./figures/##' \
    | sort -u
)

printf 'Prepared arXiv package at %s\n' "$OUT_DIR"
