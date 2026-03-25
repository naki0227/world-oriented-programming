# 2026-03-25 Phase I Paper Equivalence Note

Added a short note to the paper's Phase I subsection covering observationally equivalent top-score ties.

The paper already mentioned fallback after reject, repaired selection, and deterministic tie-breaking. This pass adds the final nuance now present in the prototype: some tied top-score candidates differ as labels but collapse to the same observed continuation.

This keeps the paper aligned with the current Phase I report structure, where observational equivalence is exposed explicitly rather than being implied by implementation behavior.
