# sekai / World-Oriented Programming

This repository records the research and prototyping process for `sekai`, a world-description language centered on space, time, relations, and constraints.

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

## Current Milestone

The first milestone is:

Declaratively simulate a bouncing sphere on a floor without writing an `update` loop.

Current prototype command:

```text
cargo run -p sekai-cli -- simulate examples/bounce.sk
```

Constraint example:

```text
cargo run -p sekai-cli -- simulate examples/forbidden_region.sk
```

Local synchronization example:

```text
cargo run -p sekai-cli -- simulate examples/two_body_collision.sk
```

Structured output example:

```text
cargo run -p sekai-cli -- simulate-json examples/two_body_collision.sk
```

Static law analysis example:

```text
cargo run -p sekai-cli -- analyze examples/reflected_region.sk
```

Figure generation example:

```text
python3 scripts/render_figure.py examples/two_body_collision.sk
```

Viewer example:

```text
python3 -m http.server 8000
```

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
