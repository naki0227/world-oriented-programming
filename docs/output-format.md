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
  "convergence_analytics": {
    "candidate_entities": 0,
    "direct_entities": 0,
    "fallback_entities": 0,
    "repaired_entities": 0,
    "tie_broken_entities": 0,
    "equivalent_tie_entities": 0,
    "determinate_entities": 0,
    "representative_entities": 0,
    "ambiguous_entities": 0,
    "symbolically_underdetermined_entities": 0,
    "observationally_underdetermined_entities": 0,
    "rejected_candidates_total": 0,
    "skipped_candidates_total": 0
  },
  "observation_summary": {
    "status": "determinate",
    "representative_entities": 0,
    "ambiguous_entities": 0
  },
  "candidate_resolutions": [],
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
  "convergence_analytics": {
    "candidate_entities": 0,
    "direct_entities": 0,
    "fallback_entities": 0,
    "repaired_entities": 0,
    "tie_broken_entities": 0,
    "equivalent_tie_entities": 0,
    "determinate_entities": 0,
    "representative_entities": 0,
    "ambiguous_entities": 0,
    "symbolically_underdetermined_entities": 0,
    "observationally_underdetermined_entities": 0,
    "rejected_candidates_total": 0,
    "skipped_candidates_total": 0
  },
  "observation_summary": {
    "status": "determinate",
    "representative_entities": 0,
    "ambiguous_entities": 0
  },
  "candidate_resolutions": [],
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

Phase I reports may additionally include:

- `convergence_analytics` for run-level totals over candidate-bearing entities
- `observation_summary` for the run-level observation status (`determinate`, `representative`, or `unresolved`)
- `candidate_resolutions` when initial action candidates were evaluated before observation
- `convergence_mode` per entity (`direct`, `fallback`, `repaired`, `deferred`, `tie_broken`, or `equivalent_tie`)
- `observation_mode` per entity (`determinate`, `representative`, or `ambiguous`)
- `observation_labels` for the labels that still matter at the observation layer
- `symbolically_underdetermined` and `observationally_underdetermined` for each candidate-bearing entity
- `selected_candidate` and `selected_score` for the chosen branch
- `top_score` and `top_labels` for the highest-score frontier before deterministic tie-breaking
- `skipped_candidates` when early selection prevents later candidates from being evaluated
- `tie_broken` when multiple top-score candidates existed and deterministic ordering selected one
- `equivalent_top_labels` and `observationally_equivalent_tie` when tied candidates collapse to the same observed result
- `repaired_after_selection` when a hard law repaired the chosen branch into admissibility

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
- exposing a compact candidate-resolution summary for underdetermined-world runs
- exposing whether a run remained symbolically or observationally underdetermined after convergence
- exposing whether symbolic ties have already collapsed into a determinate or representative observation
- exposing a minimal `Obs? = U` style status when observation is still unresolved

## Static Analysis

`sekai analyze` uses the same constraint build path but stops before time evolution.
It reports the declared law inventory, aggregated law analytics, and any Phase I candidate inventory without simulation snapshots.

For example, a Phase I analyze result may additionally include:

```json
"candidate_inventory": [
  {
    "entity": "A",
    "total_candidates": 2,
    "labels": ["fast", "safe"],
    "top_score": "5.000",
    "top_labels": ["fast"]
  }
]
```
