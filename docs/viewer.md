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
- display of active world laws, their categories, supported policies, and per-run law activity
- display of report-level law analytics for category and outcome totals
- display of Phase I candidate resolution summaries when reports include action-candidate metadata
- display of static Phase I candidate inventories and action directives when reports come from `sekai analyze`
- display of run-level Phase I convergence analytics for direct, fallback, repaired, tie-broken, and observationally equivalent outcomes
- display of observation timeline checkpoints when reports expose frontier-by-frontier observation status
- quick comparison between candidate deferred, law-update-after-defer, prefer-after-defer, rescore-after-defer, resolve-after-defer, partial-deferred, persistent-deferred, staggered-resolve, fallback, repaired-selection, tie-broken, and observationally equivalent tie Phase I samples
- support for multi-entity Phase I reports with more than one candidate-resolution card
- quick comparison between forbidden-region `reject`, `clamp`, and `reflect` samples
- quick comparison between clear and occluded `visible(A, B)` worlds
- quick comparison between clear and occluded visibility-conditioned pursuit worlds
- quick comparison between clear and occluded pursuit-world branching
- a minimal diagram-aware draft editor in `xy` mode
- automatic `.sk` draft generation from placed spheres and floor settings
- automatic constraint candidate suggestion with user-controlled adoption
- optional forbidden-region drafting for `not inside(...)` candidates
- local `Run Draft` round-trip execution through the `sekai` runtime

Static inspection can also be produced from:

```text
cargo run -p sekai-cli -- analyze <scene.sk>
```

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
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity.sk > viewer/samples/candidate_velocity.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_clamped.sk > viewer/samples/candidate_velocity_clamped.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_deferred.sk > viewer/samples/candidate_velocity_deferred.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_law_updated_resolve.sk > viewer/samples/candidate_velocity_law_updated_resolve.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_preferred_resolve.sk > viewer/samples/candidate_velocity_preferred_resolve.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_rescored_resolve.sk > viewer/samples/candidate_velocity_rescored_resolve.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_equivalent_tie.sk > viewer/samples/candidate_velocity_equivalent_tie.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_deferred_resolve.sk > viewer/samples/candidate_velocity_deferred_resolve.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_partial_deferred.sk > viewer/samples/candidate_velocity_partial_deferred.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_partial_deferred_persistent.sk > viewer/samples/candidate_velocity_partial_deferred_persistent.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_staggered_resolve.sk > viewer/samples/candidate_velocity_staggered_resolve.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_tied.sk > viewer/samples/candidate_velocity_tied.json
cargo run -p sekai-cli -- simulate-report examples/candidate_velocity_two_entity.sk > viewer/samples/candidate_velocity_two_entity.json
cargo run -p sekai-cli -- analyze examples/candidate_velocity_deferred.sk > viewer/samples/candidate_velocity_deferred_analyze.json
cargo run -p sekai-cli -- analyze examples/candidate_velocity_preferred_resolve.sk > viewer/samples/candidate_velocity_preferred_resolve_analyze.json
cargo run -p sekai-cli -- simulate-report examples/reflected_region.sk > viewer/samples/reflected_region.json
cargo run -p sekai-cli -- simulate-report examples/forbidden_region.sk > viewer/samples/forbidden_region.json
cargo run -p sekai-cli -- simulate-report examples/two_body_collision.sk > viewer/samples/two_body_collision.json
cargo run -p sekai-cli -- simulate-report examples/visibility_clear.sk > viewer/samples/visibility_clear.json
cargo run -p sekai-cli -- simulate-report examples/visibility_occluded.sk > viewer/samples/visibility_occluded.json
cargo run -p sekai-cli -- simulate-report examples/visibility_pursuit_clear.sk > viewer/samples/visibility_pursuit_clear.json
cargo run -p sekai-cli -- simulate-report examples/visibility_pursuit_occluded.sk > viewer/samples/visibility_pursuit_occluded.json
cargo run -p sekai-cli -- simulate-report examples/visibility_pursuit_world_clear.sk > viewer/samples/visibility_pursuit_world_clear.json
cargo run -p sekai-cli -- simulate-report examples/visibility_pursuit_world_occluded.sk > viewer/samples/visibility_pursuit_world_occluded.json
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

## Phase I Notes

When a report includes `candidate_resolutions`, the viewer now separates:

- run-level convergence totals
- per-entity convergence mode
- per-entity observation mode
- symbolic underdetermination
- observational underdetermination

This makes it easier to tell whether a world remained ambiguous only symbolically or also at the observation layer.

The candidate cards also expose whether a deferred entity was later resolved and, if so, at which observation frontier that happened.

The report header now also exposes a run-level observation status, so unresolved Phase I cases can be spotted without opening any entity card.

When `observation_timeline` is present, the viewer also shows frontier-by-frontier status cards, which is especially useful for staged re-convergence examples.

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
