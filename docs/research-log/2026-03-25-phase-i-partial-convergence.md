# 2026-03-25 - Phase I Partial Convergence

Phase I now includes a mixed underdetermined-world slice where one entity remains deferred while another converges after repair in the same run.

This matters because it turns Phase I from a collection of isolated single-entity examples into a world-level claim:
convergence status can differ across entities while observation remains globally meaningful.

The new sample is `examples/candidate_velocity_partial_deferred.sk`.
In that scene:

- entity `A` has a tied top score and explicitly defers selection
- entity `B` chooses a higher-scoring candidate that is repaired by the hard-law layer

The resulting report therefore shows:

- one deferred entity
- one repaired entity
- an unresolved world-level observation summary

This is the first executable partial-convergence case in the project.

The follow-up slice extends this one across two observation times, so deferred ambiguity is not only represented at the initial frontier but can also persist while other entities continue to move.

The next follow-up adds a tiny controlled re-convergence case: a deferred entity may remain unresolved at the first frontier and then resolve deterministically at a later observation time declared in the action layer.
