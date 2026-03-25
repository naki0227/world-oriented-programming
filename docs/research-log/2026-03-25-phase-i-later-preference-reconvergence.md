# 2026-03-25 Phase I Later-Preference Re-Convergence

Extended the Phase I deferred-world slice so that a later frontier can resolve an ambiguous top-score tie by activating an explicit preference.

The new prototype directive is:

- `prefer_candidate_at(A, beta, 1)`

This works together with:

- `defer_on_ambiguous_top(A)`
- `resolve_deferred_at(A, 1)`

The result is a small executable case where a world first remains unresolved and later converges for a reason stronger than the prototype's default lexical tie-break.

The runtime now exposes:

- `convergence_mode: "resolved_after_preference"`
- `preferred_label`
- `preference_resolved_entities` in run-level convergence analytics

Static analysis now also surfaces this intent through:

- `action_directive_inventory`
- a richer `resolution_hint` such as `defer_then_prefer_beta_at_1.000`

Viewer support was extended with:

- a `candidate_velocity_preferred_resolve` runtime sample
- a `candidate_velocity_preferred_resolve_analyze` static sample
- direct inspection of `preferred_label` in candidate-resolution cards

This pushes Phase I closer to a true underdetermined-world model where later information can change how convergence proceeds, rather than merely delaying a deterministic tie-break.
