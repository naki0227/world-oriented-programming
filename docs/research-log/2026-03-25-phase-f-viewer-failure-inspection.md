# 2026-03-25 Phase F Viewer Failure Inspection

## Summary

The viewer now uses failure envelopes that preserve partial simulation context.
When a world ends in contradiction but produced stable snapshots beforehand, the viewer shows the last stable state instead of switching to a blank error canvas.

## Changes

- updated the contradiction canvas to overlay the error on top of the last stable snapshot
- updated sidebar inspection so failed runs still expose stable sphere state when available
- kept `World Laws` and `World Activity` visible for contradiction reports
- aligned the viewer documentation with the new failure-inspection behavior

## Research Value

This improves the explanation quality of failed worlds.
The system now shows not only that a contradiction occurred, but also what the world looked like immediately before failure and which law fired or repaired.
That makes Phase F more persuasive as a world-law execution model rather than a black-box runtime failure.
