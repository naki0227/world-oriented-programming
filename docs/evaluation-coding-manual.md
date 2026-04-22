# Evaluation Coding Manual

Last updated: 2026-04-18

## Purpose

This document defines how the Phase K structural metrics should be interpreted.
The goal is not to prove superiority by counting lines.
The goal is to make representational burden visible in a reproducible way.

The key rule is:

> Both `sekai` and baseline programs must receive credit for world-description content.

Earlier metrics treated imperative baselines as having zero declarative content.
That was too favorable to `sekai`.
The current metric pass instead assigns every logical line to a shared category.

## Logical Lines

A logical line is a non-empty, non-comment source line.

Blank lines and comments are excluded.

## Shared Categories

### World Declaration

Lines that define entities, geometry, initial state, or domain constants.

Examples:

- `sphere A`
- `position(A) = (0, 0, 0)`
- `wall_min = {"x": 1.0, ...}`
- `sphere = Sphere(...)`

### Law Declaration

Lines that state admissibility laws, constraints, or policies as source-level declarations.

Examples:

- `constraint:`
- `reflect_on_collision(A, floor)`
- `clamp not inside(A, zone)`

In the current imperative baselines, most law logic appears as event detection or repair logic rather than source-level law declarations.
Future library-style baselines may receive law-declaration credit if they expose laws directly.

### Action Declaration

Lines that declare candidate actions, labels, scores, or preferences.

Examples:

- `candidate_velocity(A, pursue) = (1, 0, 0) score 5`
- `{"label": "pursue", "velocity": ..., "score": 5}`

### Observation Request

Lines that request snapshots or observations.

Examples:

- `observe:`
- `snapshot at 3`

The current imperative baselines often express observation through snapshot dictionaries and target-time loops.
Those lines are currently categorized as world declaration, update mechanics, or reporting depending on their role.

### Update Mechanics

Lines that advance state, mutate positions or velocities, increment time, or call stepping functions.

Examples:

- `sphere.position[0] += sphere.velocity[0] * dt`
- `current_time += dt`
- `step(sphere, dt)`

### Event Detection

Lines that detect collisions, region entry, line-of-sight, occlusion, or other event/law triggers.

Examples:

- `if sphere.position[1] - sphere.radius <= 0:`
- `line_segment_intersects_box(...)`
- `if can_see(a, b, wall_min, wall_max):`

### Repair Logic

Lines that directly repair a state after a violation.

The current script has a category for this, but many simple repairs are still detected as update mechanics or event detection because they are syntactically interleaved.
This is an area for future refinement.

### Branching / Selection Logic

Lines that route control, rank candidates, sort alternatives, loop over options, or choose outcomes.

Examples:

- `for candidate in candidates:`
- `candidates.sort(...)`
- `else:`

### Reporting / Boilerplate

Lines that return snapshots or package results.

Examples:

- `return snapshots`
- `return {"selected": selected["label"], ...}`

### Language Boilerplate

Lines required by the host language but not central to the modeled world.

Examples:

- `class Sphere:`
- `def simulate():`
- `if __name__ == "__main__":`

### Other

Lines not confidently classified by the current heuristic.
High `other` counts should trigger manual inspection.

## Derived Metrics

### World-Content Lines

```text
world_declaration + law_declaration + action_declaration + observation_request
```

This approximates how much surface form is spent saying what the world is.

### Mechanics Lines

```text
update_mechanics + event_detection + repair_logic + branching_selection
```

This approximates how much surface form is spent managing how execution proceeds.

### World-Content Density

```text
world_content_lines / logical_loc
```

This replaces the older one-sided "declarative density" metric.

### Mechanics Density

```text
mechanics_lines / logical_loc
```

This is the main companion metric.
For `sekai`, mechanics should mostly live in the runtime.
For imperative baselines, mechanics often appears in user-space code.

## Interpretation Rules

- Do not claim that fewer lines means a language is better.
- Do not claim lower cognitive load without a user study.
- Do claim that the current corpus shifts visible source burden from mechanics toward world content.
- Treat large differences as evidence for follow-up investigation, not as final proof.
- Inspect scenarios manually when metrics and narrative disagree.

## Known Limitations

- The current classifier is heuristic and line-based.
- It does not parse Python ASTs or the `sekai` DSL grammar.
- Some lines combine several roles.
- Host-language idioms can change counts without changing conceptual burden.
- Existing imperative baselines are still compact hand-written baselines, not the strongest possible comparison set.
- Event-driven baselines reduce fixed-step loop machinery, but they still express laws and event handling in ordinary Python control flow.
- Library-style baselines intentionally hide most mechanics behind a small world-building API. They are useful because they force the `sekai` claim to move beyond LOC and toward language-level law identity, source spans, contradiction reports, and viewer continuity.

## Next Improvements

- Add manual coding review for the flagship scenario.
- Compare source/report integration across `sekai` and the library-style baseline.
- Emit per-category tables in the paper appendix rather than only aggregate densities.
