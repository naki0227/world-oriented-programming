# Paper Template Setup

This directory contains the first ACM/SIGPLAN-oriented LaTeX manuscript setup for the `sekai` paper.

## Files

- `main.tex`
- `appendix.tex`
- `references.bib`
- `../docs/phase-e-submission-plan.md`

## Intended Class

The manuscript is currently written for the ACM `acmart` class in a SIGPLAN-style conference format.

Recommended starting point:

```text
\documentclass[sigplan,review,anonymous]{acmart}
```

You may want to switch options later depending on the exact submission track.

## Current Status

- the manuscript content is in `paper/main.tex`
- the seed-paper argument now includes:
  - semantics trajectory
  - underdetermined-world convergence
  - compact comparative evaluation
  - a visibility-based richer-geometry pillar
- supplementary figures omitted from the main paper live in `appendix.tex`
- bibliography entries are maintained in `references.bib`
- figures point to paper-ready image files, including `*-paper.png` viewer crops
- the setup compiles successfully with `tectonic`
- the current main-paper PDF output is `paper/main.pdf`
- the supplementary PDF output is `paper/appendix.pdf`
- the manuscript is now in a seed-paper-final state for internal review and public packaging

## Likely Next Steps

1. switch from anonymous review mode to named metadata when the submission plan requires it
2. prepare the arXiv/public package using `../docs/arxiv-release-package.md`
3. do one last venue-specific sanity pass only if the target venue or track changes

## arXiv Prep

Use:

```sh
./scripts/prepare_arxiv_package.sh
```

from the repository root to stage a public bundle under `dist/arxiv-package/`.
The staged package still expects real author metadata before a named-public arXiv upload.
