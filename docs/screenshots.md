# Screenshot Workflow

## Purpose

This document defines the minimum screenshot set for the current `sekai` paper and research log.

## Core Capture Set

Capture these screens whenever the viewer round-trip changes in a meaningful way:

1. `viewer-01-3d-two-body.png`
   - Open `two_body_collision`
   - Use `3D` mode
   - Show both spheres and the time slider
   - This supports the executable-world inspection story

2. `viewer-02-draft-candidates.png`
   - Turn on `Edit Draft`
   - Switch to `XY`
   - Place at least two spheres
   - Enable a forbidden region if relevant
   - Keep `Constraint Candidates` visible
   - This supports the diagram-to-logic story

3. `viewer-03-roundtrip-result.png`
   - Start from a draft
   - Click `Run Draft`
   - Show the resulting world state after execution
   - Keep the generated `.sk` visible if possible
   - This supports the round-trip execution story

4. `viewer-04-contradiction-report.png`
   - Run a draft or sample that violates `not inside(...)`
   - Keep the contradiction message visible
   - This supports the logical-constraint story

## Capture Notes

- Prefer the browser window at desktop width.
- Keep the full viewer layout visible when possible.
- Avoid cropped captures that hide the sidebar state.
- Record the sample or draft name in the research log entry for the same day.
- If a figure is intended for the paper, note the likely target section immediately after capture.

## Logging Rule

After taking screenshots, append a short note to the current research log entry with:

- the filename
- what changed since the previous capture
- what claim the image supports in the paper

## 2026-03-25 Capture Record

- `viewer-01-3d-two-body.png`
  - captured from the `two_body_collision` sample in `3D` mode
  - supports the claim that executable worlds can already be inspected through a unified viewer
  - likely paper use: prototype interface overview
  - paper-ready crop: `figures/viewer-01-3d-two-body-paper.png`

- `viewer-02-draft-candidates.png`
  - captured in draft-editing mode with two placed spheres and a visible forbidden region
  - supports the claim that diagrams can generate logical candidates rather than serving as passive decoration
  - likely paper use: visual-logical interface section
  - paper-ready crop: `figures/viewer-02-draft-candidates-paper.png`

- `viewer-03-roundtrip-result.png`
  - captured after `Run Draft`, with the executed world shown on the left and runtime metadata visible on the right
  - supports the claim that diagram construction can round-trip into execution without leaving the interface
  - likely paper use: prototype evaluation or interface section
  - paper-ready crop: `figures/viewer-03-roundtrip-result-paper.png`

- `viewer-04-contradiction-report.png`
  - captured after enabling `not inside(S1, forbidden_zone)` and executing the draft
  - supports the claim that contradictions are expressed as world-law violations rather than imperative repair logic
  - likely paper use: constraint-first admissibility discussion
  - paper-ready crop: `figures/viewer-04-contradiction-report-paper.png`
