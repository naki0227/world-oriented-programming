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

- the manuscript content has been moved into a LaTeX source file
- supplementary figures omitted from the main paper have been moved into `appendix.tex`
- bibliography entries have been converted into BibTeX entries
- figures point to the repository's existing paper-ready image files
- the setup has been compiled successfully with `tectonic`
- the current main-paper PDF output is `paper/main.pdf`
- the supplementary PDF output is `paper/appendix.pdf`
- the Phase E submission roadmap is tracked in `docs/phase-e-submission-plan.md`
- the main manuscript now keeps only argument-critical figures, while interface-supporting figures live in `appendix.tex`
- the build still reports light layout warnings, so another typography pass is still worthwhile before submission
- Phase E packaging is complete for an anonymous review-ready manuscript bundle

## Likely Next Steps

1. continue tightening the manuscript only if page pressure or reviewer readability demands it
2. switch from anonymous review mode to named metadata when the submission plan requires it
3. adapt front matter only if the target venue or track changes
