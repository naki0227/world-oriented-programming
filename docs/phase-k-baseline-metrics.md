# Phase K Baseline Metrics

Measurements from `python3 scripts/spec_metrics.py --markdown`.

These numbers are structural coding results, not final evidence. The current pass uses common categories for `sekai` and every baseline family so that setup and world-description lines in baselines are not forced to zero.

## Bounce

| metric | sekai | imperative | event driven | library style |
| --- | ---: | ---: | ---: | ---: |
| logical LOC | 15 | 32 | 45 | 16 |
| token count | 81 | 313 | 381 | 156 |
| branch keywords | 0 | 3 | 3 | 0 |
| loop keywords | 0 | 3 | 4 | 0 |
| state-assignment lines | 6 | 15 | 16 | 2 |
| world-content lines | 15 | 3 | 5 | 9 |
| world-content density | 1.000 | 0.094 | 0.111 | 0.562 |
| mechanics lines | 0 | 20 | 22 | 0 |
| mechanics density | 0.000 | 0.625 | 0.489 | 0.000 |

## Two Body Collision

| metric | sekai | imperative | event driven | library style |
| --- | ---: | ---: | ---: | ---: |
| logical LOC | 15 | 35 | 43 | 14 |
| token count | 86 | 377 | 408 | 138 |
| branch keywords | 0 | 2 | 2 | 0 |
| loop keywords | 0 | 2 | 2 | 0 |
| state-assignment lines | 6 | 13 | 15 | 2 |
| world-content lines | 15 | 3 | 3 | 7 |
| world-content density | 1.000 | 0.086 | 0.070 | 0.500 |
| mechanics lines | 0 | 18 | 21 | 0 |
| mechanics density | 0.000 | 0.514 | 0.488 | 0.000 |

## Candidate Velocity

| metric | sekai | imperative | event driven | library style |
| --- | ---: | ---: | ---: | ---: |
| logical LOC | 12 | 29 | 29 | 13 |
| token count | 82 | 246 | 300 | 152 |
| branch keywords | 0 | 3 | 4 | 0 |
| loop keywords | 0 | 2 | 2 | 0 |
| state-assignment lines | 6 | 9 | 7 | 2 |
| world-content lines | 12 | 4 | 2 | 6 |
| world-content density | 1.000 | 0.138 | 0.069 | 0.462 |
| mechanics lines | 0 | 11 | 11 | 0 |
| mechanics density | 0.000 | 0.379 | 0.379 | 0.000 |

## Clamped Region

| metric | sekai | imperative | event driven | library style |
| --- | ---: | ---: | ---: | ---: |
| logical LOC | 15 | 33 | 37 | 14 |
| token count | 96 | 275 | 292 | 156 |
| branch keywords | 0 | 4 | 2 | 0 |
| loop keywords | 0 | 1 | 1 | 0 |
| state-assignment lines | 7 | 16 | 16 | 2 |
| world-content lines | 15 | 4 | 6 | 7 |
| world-content density | 1.000 | 0.121 | 0.162 | 0.500 |
| mechanics lines | 0 | 22 | 18 | 0 |
| mechanics density | 0.000 | 0.667 | 0.486 | 0.000 |

## Candidate Velocity Deferred

| metric | sekai | imperative | event driven | library style |
| --- | ---: | ---: | ---: | ---: |
| logical LOC | 13 | 36 | 29 | 14 |
| token count | 86 | 317 | 359 | 164 |
| branch keywords | 0 | 4 | 4 | 0 |
| loop keywords | 0 | 3 | 4 | 0 |
| state-assignment lines | 6 | 13 | 9 | 2 |
| world-content lines | 13 | 6 | 3 | 7 |
| world-content density | 1.000 | 0.167 | 0.103 | 0.500 |
| mechanics lines | 0 | 13 | 11 | 0 |
| mechanics density | 0.000 | 0.361 | 0.379 | 0.000 |

## Visibility Occluded

| metric | sekai | imperative | event driven | library style |
| --- | ---: | ---: | ---: | ---: |
| logical LOC | 16 | 39 | 41 | 13 |
| token count | 105 | 417 | 466 | 126 |
| branch keywords | 0 | 4 | 4 | 0 |
| loop keywords | 0 | 1 | 1 | 0 |
| state-assignment lines | 8 | 17 | 18 | 2 |
| world-content lines | 16 | 16 | 16 | 6 |
| world-content density | 1.000 | 0.410 | 0.390 | 0.462 |
| mechanics lines | 0 | 7 | 8 | 0 |
| mechanics density | 0.000 | 0.179 | 0.195 | 0.000 |

## Visibility Pursuit Occluded

| metric | sekai | imperative | event driven | library style |
| --- | ---: | ---: | ---: | ---: |
| logical LOC | 18 | 51 | 44 | 15 |
| token count | 139 | 600 | 578 | 178 |
| branch keywords | 0 | 5 | 5 | 0 |
| loop keywords | 0 | 2 | 3 | 0 |
| state-assignment lines | 10 | 21 | 17 | 2 |
| world-content lines | 18 | 19 | 15 | 8 |
| world-content density | 1.000 | 0.373 | 0.341 | 0.533 |
| mechanics lines | 0 | 13 | 14 | 0 |
| mechanics density | 0.000 | 0.255 | 0.318 | 0.000 |

## Visibility Pursuit World Occluded

| metric | sekai | imperative | event driven | library style |
| --- | ---: | ---: | ---: | ---: |
| logical LOC | 20 | 56 | 45 | 17 |
| token count | 163 | 669 | 624 | 220 |
| branch keywords | 0 | 7 | 5 | 0 |
| loop keywords | 0 | 3 | 3 | 0 |
| state-assignment lines | 11 | 21 | 18 | 2 |
| world-content lines | 20 | 21 | 16 | 10 |
| world-content density | 1.000 | 0.375 | 0.356 | 0.588 |
| mechanics lines | 0 | 16 | 14 | 0 |
| mechanics density | 0.000 | 0.286 | 0.311 | 0.000 |

## Visibility Corridor World Occluded

| metric | sekai | imperative | event driven | library style |
| --- | ---: | ---: | ---: | ---: |
| logical LOC | 28 | 62 | 54 | 19 |
| token count | 233 | 760 | 771 | 276 |
| branch keywords | 0 | 8 | 6 | 0 |
| loop keywords | 0 | 4 | 4 | 0 |
| state-assignment lines | 17 | 20 | 19 | 2 |
| world-content lines | 28 | 20 | 17 | 12 |
| world-content density | 1.000 | 0.323 | 0.315 | 0.632 |
| mechanics lines | 0 | 18 | 16 | 0 |
| mechanics density | 0.000 | 0.290 | 0.296 | 0.000 |

## Visibility Coordination Flagship

| metric | sekai | library style |
| --- | ---: | ---: |
| logical LOC | 47 | 34 |
| token count | 377 | 596 |
| branch keywords | 0 | 0 |
| loop keywords | 0 | 0 |
| state-assignment lines | 26 | 2 |
| world-content lines | 47 | 27 |
| world-content density | 1.000 | 0.794 |
| mechanics lines | 0 | 0 |
| mechanics density | 0.000 | 0.000 |

## Reading

- The current corpus still shows `sekai` specifications spending more of their surface form on world content.
- The baselines now receive credit for setup and domain declarations, so the comparison no longer depends on assigning them zero declarative content.
- The event-driven baselines are a stronger comparison point than the compact imperative baselines because they remove some frame-loop machinery from user code.
- The library-style baselines show that a good host-language API can hide mechanics; the remaining `sekai` claim must therefore focus on language-level laws, contradiction identity, source spans, reports, and viewer continuity.
- The stronger signal is the difference between world-content density and mechanics density, not raw LOC alone.
- These measurements remain heuristic. The first manual flagship review now lives in `docs/flagship-comparison.md`; the next step is to compress that review into a paper table.
