# Research Log: 2026-03-24 Captioned Figures

## Session Goal

Push the figure workflow one step further so that it produces near-paper-ready assets instead of images alone.

## Context

Static figures had already been generated from executable `sekai` scenes.
The remaining friction for paper writing was that captions still had to be composed manually each time.

## Actions Taken

- extended the figure renderer to emit caption Markdown files
- added an embedded caption panel option in generated PNGs
- introduced scene-aware caption templates for the core examples
- documented the paper-figure workflow

## Expected Value

- lower friction when drafting papers or slides
- more consistent phrasing across figures
- tighter coupling between executable examples and research presentation assets

## Verification Result

Commands used:

```text
python3 scripts/render_figure.py examples/two_body_collision.sk
python3 scripts/render_figure.py examples/bounce.sk
cargo test
```

Observed outcomes:

- `figures/two_body_collision-xy.png` and `figures/two_body_collision-caption.md` were generated successfully
- `figures/bounce-xy.png` and `figures/bounce-caption.md` were generated successfully
- generated caption text is immediately reusable in paper drafting
- existing runtime tests remained green

## Next Recommended Step

Curate a first canonical figure set and align caption style with the intended paper venue.
