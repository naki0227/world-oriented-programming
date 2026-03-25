# Figure Generation v0.1

## Purpose

This document defines the first static figure-generation workflow for `sekai`.

## Command

```text
python3 scripts/render_figure.py examples/two_body_collision.sk
```

This generates a PNG figure and a caption file in `figures/` by:

1. running `sekai simulate-json`
2. reading the structured snapshot output
3. projecting the selected coordinates onto a 2D plane
4. drawing one panel per snapshot
5. generating a paper-oriented caption

## Defaults

- output path: `figures/<scene-name>-xy.png`
- caption path: `figures/<scene-name>-caption.md`
- default projection: `xy`

## Options

```text
python3 scripts/render_figure.py examples/bounce.sk --plane xy
python3 scripts/render_figure.py examples/two_body_collision.sk --output figures/collision.png
python3 scripts/render_figure.py examples/two_body_collision.sk --no-caption-panel
```

## Intended Use

- paper figures
- lab notes
- regression artifacts
- quick visual inspection of world behavior
