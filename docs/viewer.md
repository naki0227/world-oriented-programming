# Viewer v0.1

## Purpose

This document describes the first interactive viewer for `sekai` snapshot reports.
It now also covers the first draft-to-runtime round-trip path.

## What It Does

The viewer reads JSON output from:

```text
cargo run -p sekai-cli -- simulate-report <scene.sk>
```

and provides:

- a canvas-based world view
- 3D perspective view with camera controls
- a time slider over snapshots
- play/pause snapshot playback
- switching between `3d`, `xy`, and `xz`
- per-snapshot inspection of object positions and velocities
- contradiction display for failed worlds, including the last stable snapshot when available
- display of active world laws and per-run law activity
- a minimal diagram-aware draft editor in `xy` mode
- automatic `.sk` draft generation from placed spheres and floor settings
- automatic constraint candidate suggestion with user-controlled adoption
- optional forbidden-region drafting for `not inside(...)` candidates
- local `Run Draft` round-trip execution through the `sekai` runtime

## Files

- `viewer/index.html`
- `viewer/styles.css`
- `viewer/app.js`
- `viewer/samples/`

## Local Run

From the repository root:

```text
python3 scripts/viewer_server.py --port 8000
```

Then open:

```text
http://localhost:8000/viewer/
```

If you only need static viewing without round-trip execution, `python3 -m http.server 8000` still works.

## Sample Reports

Sample JSON files can be generated with:

```text
cargo run -p sekai-cli -- simulate-report examples/bounce.sk > viewer/samples/bounce.json
cargo run -p sekai-cli -- simulate-report examples/clamped_speed.sk > viewer/samples/clamped_speed.json
cargo run -p sekai-cli -- simulate-report examples/clamped_region.sk > viewer/samples/clamped_region.json
cargo run -p sekai-cli -- simulate-report examples/reflected_region.sk > viewer/samples/reflected_region.json
cargo run -p sekai-cli -- simulate-report examples/forbidden_region.sk > viewer/samples/forbidden_region.json
cargo run -p sekai-cli -- simulate-report examples/two_body_collision.sk > viewer/samples/two_body_collision.json
```

## Why This Viewer Exists

This is not the final visual interface.
Its purpose is to create the first interactive bridge between executable world descriptions and direct inspection of world evolution.
That makes it useful for:

- debugging runtime behavior
- demonstrating the core paradigm
- preparing future viewer or editor work
- supporting paper figures and talks

## Draft Editing Flow

1. Turn on `Edit Draft`
2. Switch to `XY`
3. Click the canvas to place spheres
4. Select a draft sphere and edit its radius or velocity
5. Enable or disable the floor
6. Optionally enable a forbidden region and edit its bounds
7. Review suggested constraints and adopt the ones you want
8. Click `Run Draft` to execute the generated scene through `sekai`
9. Review the returned world state or contradiction report, including the last stable snapshot and law activity if the run fails
10. Copy the generated `.sk` draft from the sidebar if you want to save it as an example

## Round-Trip Notes

`Run Draft` posts the generated DSL to a local endpoint:

```text
POST /api/simulate
```

The server writes a temporary `.sk` file, runs:

```text
cargo run -p sekai-cli -- simulate-report <temp.sk>
```

and returns the structured simulation report back to the viewer.

## Screenshot Workflow

Use [screenshots.md](/Users/hw24a094/world-oriented-programming/docs/screenshots.md) for the current paper capture checklist.
