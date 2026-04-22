# Event-Driven Baselines

These baselines are a stronger comparison target than the compact imperative examples.

They still use ordinary Python and explicit data structures, but they reduce reliance on a fixed frame loop by naming event-like operations such as:

- time-to-contact
- event firing
- visibility events
- candidate-resolution events
- law/preference enforcement

They are not meant to be the best possible competing system.
They are a middle comparison point between:

- compact hand-written imperative code
- future library-style or engine-style baselines
- `sekai` source-level world laws

The Phase K metrics include this family when files with matching scenario names exist in this directory.

