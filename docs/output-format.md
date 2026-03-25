# Output Format v0.1

## Purpose

This document defines the first structured output format for simulation snapshots.

## Commands

Text output:

```text
cargo run -p sekai-cli -- simulate examples/bounce.sk
```

JSON output:

```text
cargo run -p sekai-cli -- simulate-json examples/two_body_collision.sk
```

Structured report output:

```text
cargo run -p sekai-cli -- simulate-report examples/forbidden_region.sk
```

Static law analysis:

```text
cargo run -p sekai-cli -- analyze examples/reflected_region.sk
```

## JSON Shape

```json
{
  "source": "examples/two_body_collision.sk",
  "analytics": {
    "total_constraints": 1,
    "invariant_constraints": 0,
    "boundary_constraints": 0,
    "interaction_constraints": 1,
    "idle_constraints": 0,
    "fired_constraints": 1,
    "repaired_constraints": 0,
    "contradicted_constraints": 0
  },
  "constraints": [
    {
      "kind": "elastic_collision",
      "category": "interaction",
      "targets": ["A", "B"],
      "policy": "implicit",
      "supported_policies": [],
      "outcome": "fired",
      "fired_count": 1,
      "repaired_count": 0,
      "contradicted_count": 0
    }
  ],
  "activities": [
    {
      "time": 1.0,
      "kind": "elastic_collision",
      "targets": ["A", "B"],
      "policy": "implicit",
      "action": "fired"
    }
  ],
  "snapshots": [
    {
      "time": 0.0,
      "spheres": [
        {
          "name": "A",
          "position": { "x": 0.0, "y": 2.0, "z": 0.0 },
          "velocity": { "x": 1.0, "y": 0.0, "z": 0.0 }
        }
      ]
    }
  ]
}
```

## Report Shape For Success Or Failure

```json
{
  "source": "examples/forbidden_region.sk",
  "status": "error",
  "error": "sphere `A` entered forbidden region `zone` at t=2.000",
  "analytics": {
    "total_constraints": 1,
    "invariant_constraints": 0,
    "boundary_constraints": 1,
    "interaction_constraints": 0,
    "idle_constraints": 0,
    "fired_constraints": 0,
    "repaired_constraints": 0,
    "contradicted_constraints": 1
  },
  "constraints": [
    {
      "kind": "not_inside",
      "category": "boundary",
      "targets": ["A", "zone"],
      "policy": "reject",
      "supported_policies": ["reject", "clamp", "reflect"],
      "outcome": "contradicted",
      "fired_count": 1,
      "repaired_count": 0,
      "contradicted_count": 1
    }
  ],
  "activities": [
    {
      "time": 2.0,
      "kind": "not_inside",
      "targets": ["A", "zone"],
      "policy": "reject",
      "action": "fired"
    }
  ],
  "snapshots": [
    {
      "time": 1.0,
      "spheres": []
    }
  ]
}
```

Successful reports use:

- `"status": "ok"`
- `"error": null`
- non-empty `snapshots`

Failure reports may still include:

- active `constraints`
- recorded `activities`
- partial `snapshots` produced before contradiction

## Rationale

This format is intentionally small but useful for:

- future viewer integration
- figure generation for papers
- regression tests based on snapshot data
- external analysis scripts
- exposing which world laws and repair policies were active during execution
- exposing whether each law is invariant, boundary-oriented, or interaction-oriented
- exposing which repair policies are supported by each law
- exposing each law's run-level outcome (`idle`, `fired`, `repaired`, or `contradicted`)
- exposing report-level totals so whole runs can be compared without manual counting
- distinguishing declared laws from laws that actually fired or repaired state
- tracing when fired or repaired laws occurred during execution
- exposing candidate-action selection as activity-log entries during the initial convergence step

## Static Analysis

`sekai analyze` uses the same constraint build path but stops before time evolution.
It reports the declared law inventory and aggregated law analytics without simulation snapshots.
