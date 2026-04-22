# Flagship Comparison Notes

Last updated: 2026-04-18

## Purpose

This document records the first manual comparison between the flagship `sekai` scenario and its favorable library-style Python baseline.

The point is not to claim that `sekai` wins because it is shorter.
The library baseline intentionally gives Python a compact world-building API.
That makes the comparison stricter: if Python is allowed to look declarative, the remaining `sekai` claim must be about semantic identity, report integration, and observation-frontier structure.

## Compared Files

- `examples/visibility_coordination_flagship.sk`
- `benchmarks/library_style/visibility_coordination_flagship.py`
- `viewer/samples/visibility_coordination_flagship.json`
- `figures/visibility_coordination_flagship-xy.png`

## Manual Reading

| Axis | `sekai` flagship | Library-style Python baseline | Current interpretation |
| --- | --- | --- | --- |
| Entity and geometry declarations | Direct DSL declarations: `sphere`, `plane`, `region`, `position`, `velocity`, `radius` | Compact API calls: `world.sphere`, `world.plane`, `world.region` | Both are readable as world description. LOC alone should not be over-claimed here. |
| Law identity | `constraint:` block states speed laws as language-level source constructs | `world.law("speed_limit", ...)` records a law through an API | The Python version can name laws, but lawhood is library convention rather than syntax. |
| Candidate identity | `action:` block groups candidates, deferral, resolution, and visibility preferences | API calls encode candidates and preferences one by one | Both expose candidates, but `sekai` visually separates action-world structure from geometry and constraints. |
| Observation frontiers | `observe:` block explicitly lists `snapshot at 0`, `1`, and `2` | `world.observe(0)`, `world.observe(1)`, `world.observe(2)` | Both request observations. `sekai` makes observation a source section rather than another host-language call. |
| Report continuity | Runtime emits observation timeline, candidate resolutions, activities, constraints, and snapshots from one executable world | Baseline can simulate only if `worldlib.py` reimplements the same reporting contract | `sekai` has the stronger claim when source, runtime report, viewer, and paper figure all share one pipeline. |
| Semantic frontier story | `unresolved(2) -> unresolved(1) -> determinate(0)` is tied to observed frontiers | Same story can be modeled, but depends on library semantics | The flagship evidence should emphasize semantic frontier structure rather than surface brevity. |
| Viewer integration | The `.sk` source directly produces `viewer/samples/visibility_coordination_flagship.json` | The baseline is not the viewer input unless it recreates the report schema | This is one of the clearest current differentiators. |

## Strongest Claim

The fair claim is:

> `sekai` makes the observation-frontier structure part of the language/runtime/report pipeline, while the favorable Python baseline can express a similar world only by relying on a library convention that must reproduce the same semantic contract.

This is stronger and safer than:

> `sekai` is shorter.

## Weak Points

- The current `sekai` syntax is still small and hand-built.
- The library baseline is intentionally favorable but not independently implemented as a full competing system.
- The flagship does not yet include contradiction; it focuses on convergence.
- The viewer's smooth playback is display interpolation, not semantic observation.

## Next Evaluation Tasks

- Add per-axis manual coding to `docs/phase-k-comparative-evaluation.md`.
- Decide whether the flagship should replace the older visibility triptych as the main geometry figure.
- Add a separate contradiction-bearing flagship variant only if it does not blur the clean convergence story.
- Keep any cognitive-load language hypothetical until a real study exists.
