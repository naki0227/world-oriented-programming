# Semantic First-Class Audit

Last updated: 2026-04-22

## Purpose

This note audits the current prototype against the stronger claim that `sekai` makes certain semantic objects first-class.

It is intentionally stricter than a roadmap or vision note.
Its job is to separate:

- what the current prototype already supports clearly
- what is only partially realized
- what is still future-facing language ambition

That separation matters because the project becomes much stronger when it says:

> these objects are already executable now

instead of:

> these objects are conceptually important someday

## Audit Scale

- `Supported`: visible in source, runtime behavior, structured output, and at least one documented example or viewer path
- `Partial`: present in some layers, but missing stable structure, complete semantics, or clean end-to-end evidence
- `Not yet`: discussed in vision or semantics notes, but not yet a stable executable object

## Audit Table

| Semantic object | Current status | Why | Evidence to point to | Main gap |
| --- | --- | --- | --- | --- |
| World description as a source-level object | Supported | `.sk` programs directly declare entities, geometry, laws, actions, and observations | executable examples, CLI `simulate-report`, viewer samples | geometry and entity vocabulary are still narrow |
| Law identity and law outcome | Supported | reports expose constraints with `kind`, `category`, `targets`, `policy`, `outcome`, and counts | `docs/output-format.md`, report JSON, viewer law/activity panels | source spans and stable law ids are still limited |
| Observation frontier | Supported | explicit `observe:` blocks produce `observation_timeline` and viewer frontier cards | flagship sample, viewer playback, `docs/semantics-core.md` | observation scope is still global/prototype-scoped rather than deeply formalized |
| Observation status (`determinate`, `representative`, `unresolved`) | Supported | run-level and frontier-level statuses are explicit in reports and viewer UI | `observation_summary`, `observation_timeline`, candidate scenarios | `representative` is semantically smaller than a full equivalence theory |
| Deferred continuation | Supported | ambiguous top-score choices can remain deferred across observations and later resolve | Phase I samples, `candidate_resolutions`, viewer candidate cards | defer is still tied to the current candidate model only |
| Candidate branching and convergence outcome | Supported | reports expose `candidate_resolutions`, `convergence_mode`, top labels, skipped candidates, and later resolution metadata | flagship and Phase I samples, output-format docs | branch structure is summarized, not a richer explicit continuation graph |
| Convergence history | Supported | each candidate-bearing entity now carries `convergence_steps` with ordered frontier/defer/select/resolve phases in addition to summary metadata | candidate-resolution reports, viewer candidate cards, output-format docs | the trace is still compact and prototype-scoped rather than a full continuation graph |
| Contradiction as semantic result | Supported | contradiction now appears as structured metadata with law kind, category, participants, policy, frontier, failed predicate, and message, while also remaining visible in analytics and activities | failure envelopes, `docs/output-format.md`, forbidden-region and flagship-contradiction reports | it is still a prototype report object rather than a fully theorem-level semantic artifact |
| Unresolved world state as first-class structure | Supported | unresolved candidate/action worlds now coexist with explicit `fact_resolutions`, so observation status can remain unresolved because a world fact itself stayed open across frontiers | `examples/uncertain_world_fact.sk`, `fact_resolutions`, `observation_timeline`, viewer fact cards | fact semantics are still slot/value based rather than a richer relational theory |
| Explanation continuity from source to runtime to viewer | Partial | many source-level concerns survive into reports and viewer panels | examples -> `simulate-report` -> viewer -> figures | not every source-level construct has a stable, inspectable identity through the whole pipeline |
| Ask-for-confirmation as semantic outcome | Not yet | discussed as a desirable future runtime behavior in roadmap framing | roadmap and comparison notes | no source construct, runtime output, or viewer representation yet |
| Partial-confidence perception grounding | Not yet | the long-term architecture mentions grounding perception outputs into world facts | roadmap Stage G | no executable representation for confidence-bearing perceptual facts yet |
| Incremental perception-driven update semantics | Not yet | planned as a major advantage for practical pilots | roadmap Stage G | no implemented perception boundary or changed-observation update path yet |

## What The Prototype Can Already Defend Strongly

These are the strongest current claims:

1. `sekai` has source-level world description rather than only runtime glue.
2. laws already have inspectable runtime outcomes
3. observation frontiers are explicit and survive into reports and viewer output
4. underdetermined candidate worlds can remain unresolved and later converge
5. deferred resolution is not merely hidden controller logic; it appears in structured output

These five points are already strong enough to defend a real prototype contribution.

## What Should Be Softened In Public Claims

These claims should currently be phrased carefully:

### Uncertainty is generally first-class

Now true in a first small slice.
The current system makes unresolved candidate/action worlds first-class and also supports unresolved fact slots that remain open across observation frontiers.
It still does not ground perception confidences directly.

Better wording:

> the prototype has executable slices of underdetermined action-world semantics and deferred world-fact semantics

### The system already grounds perception into semantics

Not yet.
That is still architecture direction, not current runtime reality.

Better wording:

> the current prototype defines the execution layer that a perception grounding boundary could target

## Highest-Value Gaps

If the project wants to strengthen the "first-class semantics" claim, the next most valuable gaps to close are:

1. preserve more source-level identity through report and viewer output
2. carry the structured contradiction object more explicitly into viewer inspection
3. decide whether convergence history should later become a report-level object in addition to per-entity traces
4. move from slot/value fact ambiguity toward richer relational fact semantics

## Recommended Discipline

For now, the project should divide its language carefully:

- strong claim: first-class observation, law outcome, contradiction metadata, deferred continuation, staged convergence, per-entity convergence history, and a first executable slice of unresolved world facts in the current prototype
- medium claim: explanation continuity is partially first-class but still needs hardening
- future claim: perception grounding, partial-confidence world facts, and confirmation protocols belong to the roadmap, not to current implementation claims

That discipline does not weaken the research.
It makes the defended core much sharper.
