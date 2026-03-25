# Research Log: 2026-03-25 Phase F Repair Hook

## Session Goal

Move beyond reject-only constraint handling by adding a first repair-policy hook to the runtime.

## Actions Taken

- extended the runtime constraint model with a `RepairPolicy`
- changed world enforcement from immutable validation to an `enforce` pass that can now mutate state when repair is allowed
- added parser support for policy-prefixed constraints such as `clamp speed(A) <= 5`
- implemented `clamp` for velocity-limit constraints as the first repair-capable law
- added `examples/clamped_speed.sk` and a unit test covering clamped execution

## Outcome

- the system no longer treats every supported constraint violation as an unconditional contradiction
- Phase F now has an explicit separation between a constraint's meaning and its handling policy
- `clamp` now works for velocity limits and for `not_inside(...)` through nearest-boundary projection
- the runtime has a concrete place to grow future policies such as defer, reflect, project, or search-based repair

## Verification Note

- `cargo test` passed after the repair-hook changes
- `examples/clamped_region.sk` now executes successfully and stops the violating axis at the boundary

## Next Recommended Step

Generalize repair policies beyond velocity limits, starting with region and interaction constraints, and then expose the selected policy in reports for debugging and evaluation.
