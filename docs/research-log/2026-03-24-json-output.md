# Research Log: 2026-03-24 JSON Output

## Session Goal

Add a structured output path so simulation results can feed visualization, analysis, and paper assets.

## Context

The runtime can already simulate multi-object worlds, but plain text output is awkward for downstream tools.
JSON snapshots provide a minimal bridge without committing to a viewer implementation yet.

## Actions Taken

- added `SimulationReport::to_json`
- added `sekai simulate-json <file.sk>`
- documented the output format
- added a serialization regression test

## Expected Value

- easier viewer integration
- easier generation of paper figures and tables
- more stable automated verification against expected trajectories

## Verification Result

Commands used:

```text
cargo run -p sekai-cli -- simulate-json examples/two_body_collision.sk
cargo run -p sekai-cli -- simulate-json examples/bounce.sk
cargo test
```

Observed outcomes:

- JSON output was produced successfully for both single-object and multi-object scenarios
- the output preserves snapshot time and per-sphere position and velocity
- serialization regression tests passed along with existing world-behavior tests

## Next Recommended Step

Use JSON snapshots as the contract for the first viewer or plotting tool.
