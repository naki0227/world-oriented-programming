# Paper Figure Pack v0.1

## Purpose

This document defines how to generate figure assets that are immediately usable in papers or slides.

## Outputs

Running the renderer now produces:

- a PNG figure, optionally with an embedded caption panel, when Pillow is available
- an SVG fallback when Pillow is unavailable
- a Markdown caption file

## Example

```text
python3 scripts/render_figure.py examples/two_body_collision.sk
```

This creates:

- `figures/two_body_collision-xy.png`
- `figures/two_body_collision-caption.md`

For the current flagship figure:

```text
PYTHONPATH=/tmp/sekai-py python3 scripts/render_figure.py examples/visibility_coordination_flagship.sk --output figures/visibility_coordination_flagship-xy.png
```

This creates:

- `figures/visibility_coordination_flagship-xy.png`
- `figures/visibility_coordination_flagship-caption.md`

Without Pillow, the same renderer falls back to SVG output:

```text
python3 scripts/render_figure.py examples/visibility_coordination_flagship.sk --output figures/visibility_coordination_flagship-xy.svg
```

## Recommended Workflow

1. generate the figure from the `.sk` scene
2. inspect the PNG
3. copy or adapt the generated caption text into the paper
4. revise only if the surrounding paper context needs a tighter emphasis

## Current Figure Caption Strategy

The first version uses scene-aware caption templates for the core examples:

- `bounce`
- `two_body_collision`
- `forbidden_region`

This keeps the captions readable and paper-oriented while the example set is still small.

## Next Figure Target

The next paper-facing figure should be generated or captured from:

- `examples/visibility_coordination_flagship.sk`
- `viewer/samples/visibility_coordination_flagship.json`
- `figures/visibility_coordination_flagship-xy.png`

The figure should emphasize the observation timeline:

- `t=0`: two unresolved candidate-bearing entities
- `t=1`: `A` resolves toward `pursue_b`
- `t=2`: `D` resolves toward `support_c`

This should become the main visual evidence that observation status can change across semantic frontiers.

For talks or demos, the viewer can now play this sample with smooth interpolation between the three semantic frontiers.
For papers, still caption the frontiers as the evidence-bearing observations and treat the in-between motion as presentation aid.
