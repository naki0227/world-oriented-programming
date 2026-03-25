# Syntax v0.1 Draft

## Purpose

This document defines the first minimal textual DSL for `sekai`.
The design goal is not completeness.
It is to reach the first milestone with the smallest declarative surface.

## Design Rules

- declarations first
- no general-purpose statements
- no user-visible `update`
- no arbitrary mutation loop
- syntax should read like world specification

## Minimal Example

```text
sphere A
plane floor

position(A) = (0, 10, 0)
velocity(A) = (1, -3, 0)

constraint:
    reflect_on_collision(A, floor)
```

## Surface Elements

### Entity Declarations

```text
sphere A
plane floor
region forbidden_zone
```

### Property Assignments

```text
position(A) = (0, 10, 0)
velocity(A) = (1, -3, 0)
radius(A) = 1
normal(floor) = (0, 1, 0)
offset(floor) = 0
```

### Constraint Block

```text
constraint:
    not inside(A, forbidden_zone)
    speed(A) <= 5
    reflect_on_collision(A, floor)
    elastic collision(A, B)
```

### Observation Block

Possible future form:

```text
observe:
    snapshot at 0
    snapshot at 1.5
```

This block is optional for v0.1 if observation is controlled by CLI arguments instead.

## Draft Grammar

```ebnf
program         = { top_level } ;

top_level       = entity_decl
                | property_assign
                | constraint_block
                | observe_block ;

entity_decl     = entity_kind identifier ;
entity_kind     = "sphere" | "plane" | "region" ;

property_assign = property_name "(" identifier ")" "=" expr ;

constraint_block = "constraint" ":" newline { indent constraint_expr newline } ;
observe_block    = "observe" ":" newline { indent observe_expr newline } ;

constraint_expr = "not" "inside" "(" identifier "," identifier ")"
                | "speed" "(" identifier ")" "<=" number
                | "reflect_on_collision" "(" identifier "," identifier ")"
                | "elastic" "collision" "(" identifier "," identifier ")" ;

observe_expr    = "snapshot" "at" number ;

expr            = vector
                | number
                | identifier ;

vector          = "(" number "," number "," number ")" ;
```

## Explicit Non-Goals For v0.1

- user-defined functions
- loops
- branching syntax
- arbitrary event programming
- soft decision systems
- full 3D editor integration

## Why This Is Enough

This syntax can already support the milestone example:

- define a sphere
- define a floor
- assign initial position and velocity
- declare collision behavior
- simulate and observe

That is sufficient for validating the first world-oriented execution path.

For the current prototype, `region` is implemented as an axis-aligned box defined by:

```text
min(zone) = (x1, y1, z1)
max(zone) = (x2, y2, z2)
```

The current runtime also supports multiple spheres with explicit pairwise elastic collision constraints.
