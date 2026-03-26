# sekai / World-Oriented Programming

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.19235961.svg)](https://doi.org/10.5281/zenodo.19235961)

`sekai` is an experimental world-oriented programming language for describing executable worlds through space, time, and constraints rather than through instruction sequences.

This repository records the research, prototype implementation, manuscript, viewer, and evaluation scaffold for that project.

Public release:

- Zenodo DOI: `10.5281/zenodo.19235961`
- Zenodo record: `https://zenodo.org/records/19235961`
- Named public manuscript: `paper/main-public.tex`

## Project Statement

`sekai` is not a language for writing procedures.
It is a language for defining worlds directly through spatial structure, temporal evolution, and logical constraints.

The long-term goal is a programming environment where:

- description = modeling = execution
- 3D space is a first-class concept
- time is continuous and built into the world model
- the default execution model is globally asynchronous and locally synchronized
- exceptions are described as constraints, not patches
- diagrams, formulas, logic, and simulation refer to the same underlying world

## Repository Structure

- `docs/philosophy.md`: core philosophy and design principles
- `docs/model.md`: initial formal world model
- `docs/phase1-prototype.md`: first executable prototype scope
- `docs/roadmap.md`: staged development roadmap
- `docs/requirements.md`: initial functional and research requirements
- `docs/syntax.md`: first DSL syntax draft
- `docs/paper-outline.md`: draft paper title, abstract, and chapter plan
- `docs/output-format.md`: structured snapshot output format
- `docs/figure-generation.md`: static figure generation workflow
- `docs/paper-figures.md`: captioned paper-figure workflow
- `docs/paper-assets.md`: executable assets usable in a paper draft
- `docs/paper-figures-captions.md`: numbered figure caption draft
- `docs/paper-evaluation-draft.md`: prototype evaluation prose draft
- `docs/paper-results-draft.md`: in-text results prose draft
- `docs/paper-manuscript-skeleton.md`: first full manuscript skeleton
- `docs/paper-draft-v1.md`: first continuous paper draft
- `docs/paper-draft-v2.md`: revised draft aligned to the provisional venue
- `docs/paper-draft-v3.md`: near-submission draft with explicit figure placement
- `docs/related-work-draft.md`: related work section draft
- `docs/references-notes.md`: citation planning notes
- `docs/bibliography-draft.md`: formatted reference list draft
- `docs/venue-strategy.md`: provisional submission target strategy
- `docs/paper-submission-checklist.md`: remaining tasks before submission
- `docs/viewer.md`: minimal interactive viewer workflow
- `docs/git-workflow.md`: branch and commit workflow for research-safe development
- `docs/research-log/`: dated research logs
- `docs/decisions/`: architectural and conceptual decision records
- `docs/templates/`: reusable templates for future logs and papers
- `examples/`: reference `.sk` scenarios

## Current Status

The repository now contains:

- a prototype DSL and runtime
- executable examples for bouncing, contradiction, synchronization, convergence, and visibility
- a browser-based viewer and diagram-to-logic round-trip
- a seed-paper manuscript in LaTeX at `paper/main.tex`
- comparative evaluation scaffolding and imperative baselines

Current focus:

- final submission shaping for the seed paper
- public-facing packaging for arXiv / README / venue use
- the first richer-geometry pillar centered on visibility

## Quick Start

Minimal declarative evolution:

```sh
cargo run -p sekai-cli -- simulate examples/bounce.sk
```

Constraint-first contradiction:

```sh
cargo run -p sekai-cli -- simulate examples/forbidden_region.sk
```

Local synchronization:

```sh
cargo run -p sekai-cli -- simulate examples/two_body_collision.sk
```

Structured report:

```sh
cargo run -p sekai-cli -- simulate-json examples/two_body_collision.sk
```

Static law analysis:

```sh
cargo run -p sekai-cli -- analyze examples/reflected_region.sk
```

Figure generation:

```sh
python3 scripts/render_figure.py examples/two_body_collision.sk
```

Viewer:

```sh
python3 -m http.server 8000
```

Then open `http://127.0.0.1:8000/viewer/`.

## Public Summary

Short version:

`sekai` explores a shift from procedure-centered programming to executable world description.
Programs define entities, geometry, temporal development, and world laws directly; execution is treated as the evolution of an admissible world.

Longer version:

The current prototype already supports:

- update-free world evolution
- constraints as world laws
- global asynchrony with local synchronization
- underdetermined-world convergence slices
- visibility-conditioned branching worlds
- a viewer path from diagrammatic drafting to execution and contradiction reporting

For a paper-oriented overview, see `paper/main.tex`.
For the named public manuscript, see `paper/main-public.tex`.
For public-facing wording, see `docs/public-facing-summary.md`.
For arXiv staging, see `docs/arxiv-release-package.md`.
For citation and archive metadata, see `CITATION.cff` and `.zenodo.json`.
For a public-release flow before arXiv endorsement, see `docs/public-release-checklist.md`.

## Working Rule For This Repository

Every substantial step should be recorded in:

1. a dated research log entry
2. a decision record if a design choice becomes stable
3. a spec update if the project definition changes

## Git Workflow

- `main` is the stable research baseline
- before new work starts, create a branch first
- branch names should reflect the research topic, not just the file being edited
- use commits that explain the milestone in research terms as well as implementation terms

Examples:

- `feature/phase-g-time-semantics`
- `feature/constraint-repair-policies`
- `feature/viewer-fired-laws`
