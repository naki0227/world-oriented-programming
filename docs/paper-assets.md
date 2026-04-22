# Paper Assets v0.1

## Purpose

This document lists the current executable assets that can already be cited in a paper draft.

## Figures

### Figure 1

- image: `figures/bounce-xy.png`
- caption source: `figures/bounce-caption.md`
- role: minimal proof that a world can evolve without a user-authored update loop

### Figure 2

- image: `figures/two_body_collision-xy.png`
- caption source: `figures/two_body_collision-caption.md`
- role: demonstration of local synchronization through explicit sphere-sphere interaction

### Figure 3

- image: `figures/viewer-01-3d-two-body-paper.png`
- caption source: `docs/paper-figures-captions.md`
- role: viewer overview for executable world inspection

### Figure 4

- image: `figures/viewer-02-draft-candidates-paper.png`
- caption source: `docs/paper-figures-captions.md`
- role: diagram-to-logic candidate generation

### Figure 5

- image: `figures/viewer-03-roundtrip-result-paper.png`
- caption source: `docs/paper-figures-captions.md`
- role: round-trip from diagram editing to runtime execution

### Figure 6

- image: `figures/viewer-04-contradiction-report-paper.png`
- caption source: `docs/paper-figures-captions.md`
- role: contradiction reporting as a world-law violation inside the viewer

### Visibility Set

- image: `figures/viewer-visibility-01-clear-run-paper.png`
- caption source: `docs/paper-figures-captions.md`
- role: visibility-conditioned pursuit in the clear-line-of-sight case

- image: `figures/viewer-visibility-02-occluded-preset-paper.png`
- caption source: `docs/paper-figures-captions.md`
- role: draft-editor preset for a visibility-conditioned world

- image: `figures/viewer-visibility-03-occluded-run-paper.png`
- caption source: `docs/paper-figures-captions.md`
- role: visibility-conditioned branching toward search in the occluded case

### Flagship Convergence Figure

- image: `figures/visibility_coordination_flagship-xy.png`
- vector fallback: `figures/visibility_coordination_flagship-xy.svg`
- caption source: `figures/visibility_coordination_flagship-caption.md`
- role: main evidence for staggered underdetermined-world convergence across observation frontiers

## Executable Examples

- `examples/bounce.sk`
- `examples/forbidden_region.sk`
- `examples/two_body_collision.sk`
- `examples/visibility_coordination_flagship.sk`
- `examples/visibility_coordination_flagship_contradiction.sk`

## Structured Viewer Samples

- `viewer/samples/visibility_coordination_flagship.json`
- `viewer/samples/visibility_coordination_flagship_contradiction.json`

## Validation Commands

```text
cargo test
cargo run -p sekai-cli -- simulate examples/bounce.sk
cargo run -p sekai-cli -- simulate examples/forbidden_region.sk
cargo run -p sekai-cli -- simulate examples/two_body_collision.sk
cargo run -p sekai-cli -- simulate-report examples/visibility_coordination_flagship.sk
cargo run -p sekai-cli -- simulate-report examples/visibility_coordination_flagship_contradiction.sk
python3 scripts/render_figure.py examples/bounce.sk
python3 scripts/render_figure.py examples/two_body_collision.sk
PYTHONPATH=/tmp/sekai-py python3 scripts/render_figure.py examples/visibility_coordination_flagship.sk --output figures/visibility_coordination_flagship-xy.png
```

## Recommended Use In A Draft

- use Figure 1 in the prototype overview or minimal validation section
- use Figure 2 in the synchronization or interaction section
- use Figure 3 near the prototype interface overview
- use Figure 4 in the visual-logical interface discussion
- use Figure 5 in the round-trip execution discussion
- use Figure 6 in the constraint-validation discussion
- use the visibility set in a richer-geometry discussion or appendix once the paper needs a dedicated visibility storyline
- use the flagship convergence figure as the main evidence for staggered underdetermined-world convergence and visibility-conditioned coordination
