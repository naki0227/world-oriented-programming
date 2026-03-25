# Phase F Constraint Generalization

## Goal

Turn the current prototype from a small collection of special-case laws into a constraint layer that can be extended systematically.

## Current Direction

Phase F begins by unifying constraint handling around a shared lifecycle:

1. build a constraint from parsed syntax
2. validate world admissibility through that constraint
3. ask the constraint for its next relevant event, if any
4. apply the resulting event through a shared event path

## Why This Matters

Before this phase, the runtime handled each law through scattered `match` logic in world construction, validation, and event scheduling.
That worked for a prototype, but it made the constraint layer harder to grow into a real language feature.

The new structure makes it easier to:

- add new constraint kinds without duplicating logic across multiple passes
- separate build-time constraint checks from runtime world validation
- reason about which constraints are purely static, continuously validated, or event-generating
- prepare for later repair policies and richer solver behavior

## Immediate Deliverables

- constraint build path centralized in `orbis/src/world.rs`
- validation centralized per constraint kind
- event discovery centralized per constraint kind
- event application kept on a shared runtime path
- first repair-policy hooks added for velocity-limit and region constraints through `reject`, `clamp`, and region-local `reflect`

## Progress Since Start

Phase F now covers more than raw law execution:

- repair policies are explicit and per-law
- region exclusion now supports `reject`, `clamp`, and `reflect`
- reports expose law activity and partial failure context
- reports classify each law as `invariant`, `boundary`, or `interaction`
- reports expose which policies each law supports

## Next Steps

1. extend repair-policy support beyond the current velocity-limit and region-exclusion policies
2. enrich viewer and analysis tools around constraint category and policy comparison
3. prepare richer constraint forms beyond the current four prototype laws
4. connect this classification layer to later semantic work in Phase G
