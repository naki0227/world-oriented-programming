# 2026-03-25 Phase I Later-Rescore Re-Convergence

Extended the Phase I deferred-world slice from later preference activation to later score updates.

The new prototype directive is:

- `rescore_candidate_at(A, beta, 1, 1)`

This allows a deferred tie to be resolved because the soft ranking itself changed at a later frontier.

The prototype now distinguishes:

- `resolved_after_preference`
- `resolved_after_rescore`

and exposes:

- `active_score_adjustments`
- `rescore_resolved_entities`

in the structured report.

Static analysis now also surfaces this intent through a richer hint such as:

- `defer_then_rescore_beta_by_+1.000_at_1.000`

This matters because it pushes Phase I beyond "delay, then choose" and toward "delay, then update the preference landscape itself before convergence."
