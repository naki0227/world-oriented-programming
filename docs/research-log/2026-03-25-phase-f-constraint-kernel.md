# Research Log: 2026-03-25 Phase F Constraint Kernel

## Session Goal

Start Phase F by replacing scattered special-case constraint handling with a more systematic constraint kernel inside the runtime.

## Actions Taken

- refactored `orbis/src/world.rs` so constraints are compiled from parsed syntax into a shared runtime enum with indexed references
- moved constraint construction into `Constraint::from_parts(...)`
- moved admissibility checks into `Constraint::validate(...)`
- moved event discovery into `Constraint::candidate_event(...)`
- moved event application behind a shared `EventKind::apply(...)` path

## Outcome

- the runtime no longer spreads each constraint kind across separate ad hoc loops
- velocity limits, forbidden regions, plane reflection, and elastic collision now share one lifecycle model
- the codebase is better positioned for repair strategies, richer constraint categories, and future solver work

## Verification Note

- `cargo test` passed after the refactor

## Next Recommended Step

Separate hard contradiction from repair policy so the runtime can eventually choose between reject, clamp, reflect, or defer behavior depending on the constraint kind.
