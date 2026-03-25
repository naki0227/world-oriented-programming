# Research Log: 2026-03-25 Phase F Report Metadata

## Session Goal

Expose constraint-kind and repair-policy information in structured runtime output so that viewers, papers, and debugging tools can observe the law layer directly.

## Actions Taken

- extended `SimulationReport` with a `constraints` summary list
- added per-constraint summaries containing kind, targets, and policy
- updated JSON serialization for both report-only and envelope output
- updated the browser viewer so it displays active world laws in the sidebar
- updated output-format documentation and serialization tests

## Outcome

- runtime output now carries not just snapshot state but also the law structure that shaped that state
- viewer round-trips can show which laws were active, not only what positions and velocities resulted
- Phase F now supports better debugging, demos, and research-facing explanation
- reports can now distinguish laws that merely exist from laws that actually fired or repaired state during the run

## Verification Note

- `cargo test` passed after adding report metadata
- `simulate-report examples/clamped_region.sk` now reports `fired_count: 1` and `repaired_count: 1`
- `simulate-report examples/two_body_collision.sk` now reports `fired_count: 1` for `elastic_collision`

## Next Recommended Step

Use this Phase F cutoff as the first clean Git checkpoint, then branch future work around richer repair policies, simultaneous-event semantics, or viewer-side visualization of fired laws.
