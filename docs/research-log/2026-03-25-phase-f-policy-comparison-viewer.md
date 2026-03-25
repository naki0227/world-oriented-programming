# 2026-03-25 Phase F Policy Comparison Viewer

## Summary

The viewer now includes a lightweight comparison panel for forbidden-region laws.
This makes the difference between `reject`, `clamp`, and `reflect` visible without requiring users to manually hunt through samples.

## Changes

- added a `Policy Comparison` panel to the viewer sidebar
- exposed one-click switching between `forbidden_region`, `clamped_region`, and `reflected_region`
- summarized the current law outcome as `contradiction`, `repaired world`, or `admissible world`
- kept the comparison tightly focused on the existing forbidden-region law family

## Research Value

This turns the viewer into a more explicit policy-inspection tool.
Instead of only showing that repair policies exist, the system now helps users compare enforcement styles on the same logical law.

That strengthens Phase F as a study of world-law behavior, not only a runtime refactor.
