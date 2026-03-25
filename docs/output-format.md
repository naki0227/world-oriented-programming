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

## JSON Shape

```json
{
  "source": "examples/two_body_collision.sk",
  "constraints": [
    {
      "kind": "elastic_collision",
      "targets": ["A", "B"],
      "policy": "implicit",
      "fired_count": 1,
      "repaired_count": 0
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
  "constraints": [],
  "snapshots": []
}
```

Successful reports use:

- `"status": "ok"`
- `"error": null`
- non-empty `snapshots`

## Rationale

This format is intentionally small but useful for:

- future viewer integration
- figure generation for papers
- regression tests based on snapshot data
- external analysis scripts
- exposing which world laws and repair policies were active during execution
- distinguishing declared laws from laws that actually fired or repaired state
