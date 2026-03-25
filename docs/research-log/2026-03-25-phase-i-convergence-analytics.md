# 2026-03-25 Phase I Convergence Analytics

Extended the Phase I report model from per-entity candidate resolution to run-level convergence analytics.

The report now distinguishes:
- direct resolution
- fallback after reject
- repaired selection
- tie-broken resolution
- observationally equivalent ties

It also now distinguishes observation-level statuses:
- `determinate`
- `representative`
- `ambiguous`

At run level the report now also exposes:
- `determinate`
- `representative`
- `unresolved`

This is the first prototype-level hook for `Obs? = U`.

The next extension on top of this pass adds an explicit deferred case, where an ambiguous top-score tie is left unresolved instead of forcing representative branch selection.

The static analysis path now also exposes that directive, so `sekai analyze` can show that a scene intends to defer ambiguous top-score resolution before any simulation runs.

It now also exposes a small static resolution hint, so tied-top cases can be distinguished from deferred-top cases before runtime.

The viewer now also accepts a static `analyze` sample for deferred Phase I worlds, so the pre-runtime intent and the post-runtime convergence behavior can be inspected in the same interface.

Each entity-level candidate resolution now also says whether the world remained symbolically underdetermined and whether that underdetermination was still visible at the observation layer.

This pushes Phase I closer to a real convergence model instead of a set of isolated candidate-selection examples.

The paper was also updated with a short note so this new convergence-analytics layer is reflected in the written research narrative rather than remaining only in the runtime and viewer.
