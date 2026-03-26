# 2026-03-26 arXiv Bibliography Packaging

## Summary

Prepared a safer arXiv upload path by introducing a dedicated upload entry point and bundling the generated bibliography output directly.

## Changes

- added `paper/main-arxiv.tex` as the arXiv upload entry point
- switched the arXiv entry point to `\input{main.bbl}` instead of relying on `\bibliography{references}`
- updated `scripts/prepare_arxiv_package.sh` to stage:
  - `paper/main-arxiv.tex`
  - `paper/main.bbl`
- updated arXiv-facing documentation to point users to `paper/main-arxiv.tex`

## Verification

- `tectonic paper/main-arxiv.tex`
- `./scripts/prepare_arxiv_package.sh`
- confirmed that `dist/arxiv-package/` contains `paper/main-arxiv.tex`, `paper/main.bbl`, and the referenced figures

## Outcome

The repository now has a clearer split between:

- `paper/main.tex` for anonymous review
- `paper/main-public.tex` for local named-public builds
- `paper/main-arxiv.tex` for arXiv upload
