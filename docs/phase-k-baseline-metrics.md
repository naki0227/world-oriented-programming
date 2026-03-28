# Phase K Baseline Metrics

Initial scaffold measurements from `python3 scripts/spec_metrics.py`.

These numbers are intentionally modest and structural. They are not final evidence,
but they make the evaluation phase concrete and reproducible.

## Bounce

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 15 | 32 |
| token count | 81 | 313 |
| branch keywords | 0 | 3 |
| loop keywords | 0 | 3 |
| state-assignment lines | 6 | 15 |
| declarative lines | 13 | 0 |
| declarative density | 0.867 | 0.000 |

## Two-Body Collision

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 15 | 35 |
| token count | 86 | 377 |
| branch keywords | 0 | 2 |
| loop keywords | 0 | 2 |
| state-assignment lines | 6 | 13 |
| declarative lines | 14 | 0 |
| declarative density | 0.933 | 0.000 |

## Candidate Velocity

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 12 | 29 |
| token count | 82 | 246 |
| branch keywords | 0 | 3 |
| loop keywords | 0 | 2 |
| state-assignment lines | 6 | 9 |
| declarative lines | 11 | 0 |
| declarative density | 0.917 | 0.000 |

## Clamped Region

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 15 | 33 |
| token count | 96 | 275 |
| branch keywords | 0 | 4 |
| loop keywords | 0 | 1 |
| state-assignment lines | 7 | 16 |
| declarative lines | 14 | 0 |
| declarative density | 0.933 | 0.000 |

## Candidate Velocity Deferred

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 13 | 36 |
| token count | 86 | 317 |
| branch keywords | 0 | 4 |
| loop keywords | 0 | 3 |
| state-assignment lines | 6 | 13 |
| declarative lines | 11 | 0 |
| declarative density | 0.846 | 0.000 |

## Visibility Occluded

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 16 | 39 |
| token count | 105 | 417 |
| branch keywords | 0 | 5 |
| loop keywords | 0 | 1 |
| state-assignment lines | 8 | 17 |
| declarative lines | 15 | 0 |
| declarative density | 0.938 | 0.000 |

## Visibility Pursuit Occluded

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 18 | 51 |
| token count | 139 | 600 |
| branch keywords | 0 | 6 |
| loop keywords | 0 | 2 |
| state-assignment lines | 10 | 21 |
| declarative lines | 17 | 0 |
| declarative density | 0.944 | 0.000 |

## Visibility Pursuit World Occluded

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 20 | 56 |
| token count | 163 | 669 |
| branch keywords | 0 | 8 |
| loop keywords | 0 | 3 |
| state-assignment lines | 11 | 21 |
| declarative lines | 18 | 0 |
| declarative density | 0.900 | 0.000 |

## Visibility Corridor World Occluded

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 28 | 62 |
| token count | 233 | 760 |
| branch keywords | 0 | 9 |
| loop keywords | 0 | 4 |
| state-assignment lines | 17 | 20 |
| declarative lines | 26 | 0 |
| declarative density | 0.929 | 0.000 |

## Surface Channel

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 18 | 34 |
| token count | 100 | 346 |
| branch keywords | 0 | 4 |
| loop keywords | 0 | 3 |
| state-assignment lines | 7 | 17 |
| declarative lines | 16 | 0 |
| declarative density | 0.889 | 0.000 |

## Surface Room Clamped

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 22 | 61 |
| token count | 139 | 536 |
| branch keywords | 0 | 5 |
| loop keywords | 0 | 5 |
| state-assignment lines | 11 | 20 |
| declarative lines | 21 | 0 |
| declarative density | 0.955 | 0.000 |

## Surface Room Reflective

| metric | sekai | imperative baseline |
| --- | ---: | ---: |
| logical LOC | 26 | 41 |
| token count | 152 | 419 |
| branch keywords | 0 | 6 |
| loop keywords | 0 | 3 |
| state-assignment lines | 11 | 19 |
| declarative lines | 22 | 0 |
| declarative density | 0.846 | 0.000 |

## Reading

- In this initial corpus, `sekai` specifications are shorter and structurally more declarative.
- The imperative baselines spend more of their specification budget on control and explicit update structure.
- The widened corpus keeps the same pattern even for repair-oriented and deferred-convergence scenarios.
- The first visibility slice keeps the same pattern while moving the corpus toward richer geometry.
- The visibility-pursuit pair keeps the same structural pattern even when geometry changes candidate selection rather than only causing contradiction.
- The branching visibility-pursuit world keeps the same pattern even when geometry selects between continuation families such as pursuit and search.
- The corridor visibility world keeps the same pattern even when several regions must jointly preserve or block line of sight.
- The first multi-surface channel keeps the same pattern even when one world carries several distinct contact surfaces.
- The first bounded surface room keeps the same pattern even when admissibility is defined by several planes rather than by one region or one contact surface.
- The first reflective surface room keeps the same pattern even when several declared contact boundaries must all remain active at once.
- These measurements are only a first scaffold; later Phase K work should widen the corpus and refine the metrics.
