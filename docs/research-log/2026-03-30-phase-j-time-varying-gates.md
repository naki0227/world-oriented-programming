# 2026-03-30 - Phase J time-varying gates

This pass extended the new gate geometry family from static apertures to time-varying room transitions.

What changed:

- added `through_gate_after(...)` as a boundary law that keeps a wall closed until a given opening time
- added deferred gate examples showing both later passage and late clamping
- aligned the tests with the actual semantics of deferred candidate selection plus later gate enforcement
- added a compact imperative baseline for the closed-gate case
- propagated the slice into viewer samples, Phase J docs, Phase K evaluation notes, and the paper

Why it matters:

- geometry now affects not only static admissibility but also whether a room transition becomes available at a later frontier
- this connects the gate family directly to Phase I deferred convergence
- it gives Phase J a second temporally structured geometry pillar alongside time-varying visibility

Current reading:

The runtime now supports a small but meaningful connected-space story:
one wall can be declared with a named aperture, that aperture can remain closed until a later frontier,
and a deferred continuation can later either pass through or be repaired back to the original side.
