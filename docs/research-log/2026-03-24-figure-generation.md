# Research Log: 2026-03-24 Figure Generation

## Session Goal

Create a reproducible path from executable world descriptions to static figures suitable for research communication.

## Context

JSON snapshots already provide a stable structured interface.
The next step is to turn them into images that can be inserted into notes, slides, and eventually papers.

## Actions Taken

- added a Pillow-based figure renderer
- made the renderer call `sekai simulate-json` internally
- generated multi-panel PNG figures from snapshot sequences
- documented the figure-generation workflow

## Design Choice

The first figure tool uses a simple 2D projection and one panel per snapshot.
This keeps the rendering deterministic and easy to use in research workflows without introducing a full viewer.

## Verification Result

Commands used:

```text
python3 scripts/render_figure.py examples/two_body_collision.sk
python3 scripts/render_figure.py examples/bounce.sk
cargo test
```

Observed outcomes:

- `figures/two_body_collision-xy.png` was generated successfully
- `figures/bounce-xy.png` was generated successfully
- existing runtime tests continued to pass

## Next Recommended Step

Add figure styling presets for papers and produce a first set of canonical figures for the core examples.
