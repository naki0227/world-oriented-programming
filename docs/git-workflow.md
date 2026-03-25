# Git Workflow

## Purpose

This repository should preserve research meaning in its Git history, not just code changes.

## Default Rule

Before starting any substantial task:

1. branch from `main`
2. do the work on that branch
3. commit with a message that names the research milestone clearly
4. merge back only when the result is a stable step in the project narrative

## Branching Model

- `main`
  - stable research baseline
  - should remain usable for paper work, demos, and reference implementations
- `feature/*`
  - new implementation or research ideas
  - default place for active work
- `paper/*`
  - optional branch family for submission-only restructuring or venue-specific formatting

## Naming Guidance

Prefer names that describe the research idea:

- `feature/phase-g-time-semantics`
- `feature/constraint-repair-policies`
- `feature/viewer-fired-laws`
- `feature/diagram-logic-extraction`

Avoid names that only describe a tiny code edit:

- `fix-stuff`
- `update-files`
- `branch1`

## Commit Guidance

Commit messages should explain both:

- what changed technically
- why that change matters in the research roadmap

Good examples:

- `Add Phase F repair policies for velocity and region constraints`
- `Expose constraint firing metadata in runtime reports`
- `Complete Phase E anonymous review package`

## Recommended Habit

For each new thread of work:

1. decide the research goal
2. create the branch first
3. keep research logs in sync while working
4. commit when a meaningful milestone is reached

This keeps the repository usable as both an engineering history and a research record.

## Branch Retention

Merged branches do not need to be deleted immediately.
For this repository, branch history is also research history.

Recommended rule:

- keep phase-level branches such as `feature/phase-f-*` or `feature/phase-g-*`
- delete only throwaway or abandoned branches that no longer add research value
- treat merged phase branches as archived milestones unless active work resumes on them

This keeps GitHub readable while preserving the path by which the research evolved.
