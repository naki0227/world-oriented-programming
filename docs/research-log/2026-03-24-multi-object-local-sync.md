# Research Log: 2026-03-24 Multi-Object Local Synchronization

## Session Goal

Extend the prototype from a single moving sphere to a small multi-object world where local synchronization has visible meaning.

## Context

The earlier prototype already showed that a world can evolve without a user-written update loop.
To make the asynchronous-plus-local-synchronous idea more concrete, the runtime now needs interactions between independent moving objects.

## Actions Taken

- changed the runtime from a single-sphere model to a multi-sphere model
- changed snapshots so they record all spheres at an observation time
- added explicit `elastic collision(A, B)` constraints
- implemented event selection over sphere-plane, sphere-region, and sphere-sphere events
- added a two-body reference example
- updated the CLI output for multi-sphere snapshots

## Observations

- local synchronization becomes easier to explain once two independently moving spheres meet only at collision time
- explicit collision constraints help keep interaction semantics legible
- the model is still intentionally narrow, but it now demonstrates genuine interaction rather than isolated motion

## Verification Result

Commands used:

```text
cargo run -p sekai-cli -- simulate examples/bounce.sk
cargo run -p sekai-cli -- simulate examples/forbidden_region.sk
cargo run -p sekai-cli -- simulate examples/two_body_collision.sk
cargo test
```

Observed outcomes:

- `bounce.sk` still reflects correctly on floor contact
- `forbidden_region.sk` still halts at `t=2.000` on contradiction
- `two_body_collision.sk` shows independent motion until contact, then post-collision velocity exchange
- unit tests passed for floor reflection, forbidden region, velocity limit, and two-body elastic collision

## Open Questions

1. Should sphere-sphere collision eventually become a default physical law rather than an explicit constraint?
2. How should simultaneous events be prioritized when more than one local interaction occurs at once?
3. Should observation eventually support partial or camera-scoped snapshots rather than always materializing the whole world?

## Next Recommended Step

Either:

- add structured output for visualization and paper figures
- or extend the scheduler to handle richer simultaneous local interactions
