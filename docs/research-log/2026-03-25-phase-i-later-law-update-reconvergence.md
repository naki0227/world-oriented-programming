# 2026-03-25 Phase I Later-Law-Update Re-Convergence

Extended the Phase I deferred-world slice from later preference and later score updates to later admissibility updates.

The new prototype directive is:

- `update_speed_limit_at(A, 1, 6)`

This lets a deferred tie resolve because the hard law itself changed at a later frontier.

The prototype now distinguishes:

- `resolved_after_law_update`

and exposes:

- `active_law_updates`
- `law_updated_entities`

in the structured report.

This matters because it pushes Phase I beyond changing preferences over a fixed admissibility space.
The world can now also defer a decision until the admissibility space itself changes.
