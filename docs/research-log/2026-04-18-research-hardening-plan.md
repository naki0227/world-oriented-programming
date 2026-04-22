# 2026-04-18: Research-Hardening Plan

## Session Goal

Read the current task and roadmap material, then convert a strict critique of the project into a concrete plan for making `sekai` a stronger and more defensible research language.

## Context

The current prototype already has meaningful pieces:

- a runnable DSL and runtime
- semantics notes around time frontiers, events, local synchronization, and contradiction
- Phase I underdetermined-world work
- Phase J visibility and richer geometry work
- Phase K comparative evaluation scaffolding
- a viewer and diagram-to-execution loop

The critique identified several risks:

- the novelty claim can sound broader than the evidence supports
- the semantics are still more of a trajectory than a compact formal core
- the evaluation metrics currently favor `sekai`
- imperative baselines are too narrow
- many small examples do not yet create one decisive flagship case
- paper scope has started to absorb too many active research threads

## Actions Taken

- Read `CURRENT_TASK.md`, `docs/roadmap.md`, paper checklist, public release checklist, Phase I/J/K notes, and evaluation comparison docs.
- Added `docs/research-strategy-and-task-plan.md`.
- Linked that plan from `CURRENT_TASK.md` and `docs/roadmap.md`.
- Confirmed that `docs/model.md` no longer contains the conflict markers previously visible around Phase G2/G3.
- Added `docs/positioning-matrix.md` to make nearest-neighbor comparisons explicit.
- Added `docs/semantics-core.md` as the first single reference point for world frontiers, events, synchronization, enforcement, contradiction, observation, and convergence.
- Updated `scripts/spec_metrics.py` to classify both `sekai` and imperative baselines with shared structural categories.
- Regenerated `docs/phase-k-baseline-metrics.md` from the new world-content and mechanics-density metrics.
- Added `docs/evaluation-coding-manual.md` to document the structural coding rules and avoid one-sided declarative-density claims.
- Updated Phase K and roadmap wording so cognitive-load claims remain hypotheses unless directly studied.
- Added a full `benchmarks/event_driven/` baseline family for the current Phase K scenario corpus.
- Updated `scripts/spec_metrics.py` so one `sekai` scenario can be compared against multiple baseline families.
- Regenerated `docs/phase-k-baseline-metrics.md` with `imperative` and `event driven` columns.
- Added `benchmarks/event_driven/README.md` to describe the role of the event-driven comparison family.
- Added a full `benchmarks/library_style/` baseline family with a small `worldlib.py` API.
- Updated metrics so library-style API calls are classified as world, law, action, and observation content.
- Regenerated Phase K metrics with `imperative`, `event driven`, and `library style` columns.
- Added `examples/visibility_coordination_flagship.sk` as the first named flagship scenario.
- Added `benchmarks/library_style/visibility_coordination_flagship.py` as its favorable host-language API comparison.
- Added `viewer/samples/visibility_coordination_flagship.json` from the runtime report.
- Added `docs/flagship-scenario.md` to explain the scenario, observed behavior, and next paper/viewer tasks.
- Added a Rust regression test for the flagship observation story: unresolved at `t=0`, partially unresolved at `t=1`, determinate at `t=2`.
- Added smooth viewer playback between semantic observation frontiers.
- Kept the semantics/display distinction explicit: interpolation is a visual aid, not an additional observation.
- Extended `scripts/render_figure.py` so it can use `simulate-report`, render observation timeline metadata, and fall back to SVG when Pillow is unavailable.
- Generated `figures/visibility_coordination_flagship-xy.png`, `figures/visibility_coordination_flagship-xy.svg`, and `figures/visibility_coordination_flagship-caption.md`.
- Inserted the flagship convergence figure into `paper/main.tex` and `paper/main-body.tex`.
- Added `docs/flagship-comparison.md` as the first manual comparison against the favorable library-style baseline.

## Decisions Recorded

The next stage should be treated as research hardening, not only feature growth.

The central thesis should be narrowed from:

> Programs do not need update loops.

to:

> A program can define an observable spatial-temporal world whose entities, laws, events, contradictions, local synchronization, and underdetermined continuations share one executable semantic model.

The near-term pillars are:

- novelty and positioning
- semantic core
- fair evaluation
- flagship vertical slice
- viewer as research instrument
- language and runtime hardening
- paper strategy

## Open Questions

- Which single flagship scenario should become the main evidence case?
- How much Phase I convergence material belongs in the seed paper versus a follow-up paper?
- Which stronger baseline should be implemented first: event-driven Python, Modelica-like pseudocode, or ECS/game-engine style?
- How formal should the current operational semantics become before the next submission target?

## Next Recommended Step

Start with Week 1 of `docs/research-strategy-and-task-plan.md`:

1. create `docs/semantics-proof-obligations.md`
2. compress `docs/flagship-comparison.md` into a paper-ready table
3. decide whether the flagship figure should replace the older visibility triptych in the main paper
4. modestly reframe evaluation language in the paper
