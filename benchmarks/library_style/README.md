# Library-Style Baselines

These baselines represent a stronger comparison than both compact imperative and event-driven Python.

They assume that the host language has a small world-building library with source-level calls for:

- entities
- regions and planes
- laws
- candidate actions
- visibility-conditioned preferences
- observations

This family is intentionally favorable to the baseline side: it gives Python a domain-specific API that can hide event-detection and repair mechanics.
That makes it useful for asking what `sekai` still contributes when a conventional language has a good library.

The intended distinction is:

- the library-style baseline expresses world/law content through API calls
- `sekai` expresses the same content as the source language itself
- future work should compare reporting, source spans, contradiction identity, and viewer integration rather than only line counts

