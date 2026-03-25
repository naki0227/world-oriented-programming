# Paper Figure Pack v0.1

## Purpose

This document defines how to generate figure assets that are immediately usable in papers or slides.

## Outputs

Running the renderer now produces:

- a PNG figure, optionally with an embedded caption panel
- a Markdown caption file

## Example

```text
python3 scripts/render_figure.py examples/two_body_collision.sk
```

This creates:

- `figures/two_body_collision-xy.png`
- `figures/two_body_collision-caption.md`

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
