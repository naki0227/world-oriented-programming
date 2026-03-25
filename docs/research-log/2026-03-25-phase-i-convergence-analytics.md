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

Each entity-level candidate resolution now also says whether the world remained symbolically underdetermined and whether that underdetermination was still visible at the observation layer.

This pushes Phase I closer to a real convergence model instead of a set of isolated candidate-selection examples.

The paper was also updated with a short note so this new convergence-analytics layer is reflected in the written research narrative rather than remaining only in the runtime and viewer.
