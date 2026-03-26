# arXiv Release Package

## Purpose

This note tracks the public-facing package needed when the seed paper is prepared for arXiv.

## Core Files

- `paper/main.tex`
- `paper/main.pdf`
- `paper/references.bib`
- `README.md`
- `docs/public-facing-summary.md`
- `docs/arxiv-author-metadata-template.md`
- `scripts/prepare_arxiv_package.sh`

## Recommended Abstract Basis

Use the current abstract in `paper/main.tex` as the canonical basis.
If a shorter arXiv summary is needed, compress around these points:

1. `sekai` treats programs as executable world descriptions.
2. The prototype already supports update-free evolution, world-law constraints, and local synchronization.
3. The current system includes a viewer, a first semantics trajectory, underdetermined-world convergence, and a first visibility-based geometry pillar.
4. The contribution is coherence, not mature generality.

## Public Metadata Checklist

- title matches the manuscript title
- abstract matches the current seed-paper framing
- repository README uses the same one-sentence description
- figures referenced publicly use the paper-ready image variants when available
- the repository branch used for the public release is clean and reproducible
- author metadata is filled before making a named-public version

## Staging Command

Prepare a bundle skeleton with:

```sh
./scripts/prepare_arxiv_package.sh
```

This creates `dist/arxiv-package/` containing:

- `paper/main.tex`
- `paper/main.pdf`
- `paper/references.bib`
- any figures referenced by `paper/main.tex` or `paper/appendix.tex`
- the public summary and author-metadata checklist

## Suggested arXiv Summary

We present `sekai`, an experimental world-oriented programming language in which programs describe executable worlds through space, time, and constraints rather than through instruction sequences.
The current prototype combines declarative world description, constraint-first admissibility, local synchronization, a first semantics trajectory, underdetermined-world convergence slices, and a viewer round-trip from diagrammatic drafting to execution.
The paper does not claim mature generality; it argues that this representational shift is already coherent enough to support a small executable system and a first comparative evaluation.
