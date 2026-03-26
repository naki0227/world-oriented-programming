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

## Reading

- In this initial corpus, `sekai` specifications are shorter and structurally more declarative.
- The imperative baselines spend more of their specification budget on control and explicit update structure.
- The widened corpus keeps the same pattern even for repair-oriented and deferred-convergence scenarios.
- The first visibility slice keeps the same pattern while moving the corpus toward richer geometry.
- These measurements are only a first scaffold; later Phase K work should widen the corpus and refine the metrics.
