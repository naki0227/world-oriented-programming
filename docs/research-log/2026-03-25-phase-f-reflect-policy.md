# 2026-03-25 Phase F Reflect Policy

## Summary

Phase F now includes a third repair policy for region exclusion laws.
In addition to `reject` and `clamp`, `not_inside(...)` can now use `reflect` to bounce a sphere off the nearest forbidden-region boundary.

## Changes

- added `reflect` to the parsed repair-policy vocabulary
- implemented region-local reflection for `not_inside(...)`
- kept `velocity_limit(...)` restricted so unsupported `reflect` usage fails explicitly
- added a dedicated example and unit test for reflected region repair

## Research Value

This makes the policy layer more clearly interpretable as part of the world model.
The same logical law can now be expressed with multiple enforcement styles:

- reject the world
- clamp the world back to admissibility
- reflect the world off a forbidden boundary

That is a stronger demonstration that constraints in `sekai` are not only predicates, but operational laws.
