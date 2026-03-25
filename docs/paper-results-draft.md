# Paper Results Draft

## In-Text Figure References

Figure 1 demonstrates that a simple spatial-temporal world can be made executable without requiring the user to author an update loop.
The user specifies the existence of a sphere and a floor, along with initial motion and a collision rule, and the runtime derives the observed evolution from those declarations alone.

Figure 2 illustrates local synchronization.
Two spheres are allowed to progress independently until a collision occurs.
At that point, the runtime resolves the explicit elastic-collision constraint and produces a new consistent continuation of the world.

## Short Discussion Paragraph

Taken together, the current prototype suggests that world-oriented execution is already meaningful even in a narrow core language.
The value does not yet come from scale, but from representational alignment: the user describes entities, motion, interaction, and admissibility conditions directly, while the runtime assumes responsibility for temporal progression and localized synchronization.
