# What Existing Systems Scatter / What `sekai` Makes First-Class

Last updated: 2026-04-22

## Purpose

This note is not a claim that no prior system addresses safety, planning, uncertainty, or execution.
Many systems do.

The point of this document is narrower:

> What semantic concerns are usually split across modules, APIs, planners, monitors, and logs, and which of those does `sekai` try to make first-class language/runtime objects?

This is the stronger framing for the project.
Without it, `sekai` risks sounding like a thin integration layer.

## Short Answer

`sekai` should not be defended as "we connected AI, planning, and safety."
It should be defended as:

> a language/runtime where incomplete world state, explicit laws, observation frontiers, contradiction, deferred choice, and convergence history remain visible as one executable semantic account

## Comparison Table

| Concern | Typical existing treatment | What `sekai` aims to make first-class |
| --- | --- | --- |
| World description | Split across geometry config, simulator setup, planner inputs, and runtime glue | One source-level world description with entities, geometry, laws, actions, and observations |
| Laws and safety constraints | Often encoded in planner rules, post-hoc filters, monitors, or imperative logic | Named law-like source constructs with stable runtime/report identity |
| Observation timing | Often dense logging, frame-based polling, or implicit sampling | Explicit observation frontiers with semantic status |
| Contradiction | Often thrown as error, rejected command, or safety abort with weak semantic identity | Contradiction as a world-level semantic result tied to law, frontier, and participants |
| Uncertainty | Often only confidence scores on detections or planner probabilities | Structurally unresolved world states and observation statuses |
| Deferred action | Often encoded informally as wait states, retries, or controller-specific fallback | Deferred continuation as a legitimate runtime state, not merely an ad hoc workaround |
| Candidate branching | Often hidden in planner search internals or policy logits | Source-visible candidate actions and convergence outcomes |
| Explanation | Often reconstructed from logs after execution | Runtime report directly tied to source-level objects and semantic events |
| Perception handoff | Often custom glue from model output into planner state | Intended world-fact grounding boundary with stable execution semantics after grounding |

## Where Existing Systems Are Already Strong

This project should be honest about prior strength.
Existing stacks already do many important things well:

- planners can search effectively
- controllers can execute reliably
- runtime monitors can enforce hard safety conditions
- probabilistic systems can reason under uncertainty
- perception systems can produce rich environment signals
- robotics middleware can connect many subsystems robustly

`sekai` does not win by denying that.
It wins only if it makes the semantic relationship between those concerns more explicit and more executable.

## The Stronger Novelty Claim

The weak claim is:

> Existing systems are fragmented; `sekai` integrates them.

That is not enough.

The stronger claim is:

> Existing systems often distribute key semantic concerns across different layers, while `sekai` tries to keep world description, admissibility, contradiction, observation frontier, deferred continuation, and convergence legible as one executable model.

That is closer to a language contribution than a systems-integration contribution.

## The Most Important First-Class Objects

If the project has a durable core, it is probably here:

1. world frontier
2. law outcome
3. contradiction
4. deferred continuation
5. observation status
6. convergence history

If these remain shallow, the project risks collapsing into "DSL plus tooling."
If these become precise and demonstrable, the project has a stronger chance of reading as a real semantic contribution.

## What This Means For Evaluation

Evaluation should not ask only:

- Is `sekai` shorter?
- Is `sekai` more readable?
- Can `sekai` run a robot?

It should also ask:

- Which semantic objects are explicit in `sekai` but implicit elsewhere?
- Which outcomes remain inspectable at runtime rather than being collapsed into generic failure?
- Can the same source-level object be followed through execution, report, viewer, and paper evidence?
- Can underdetermined worlds remain underdetermined without being forced too early into a single action?

## Threats To This Position

The project becomes weaker if:

- all interesting behavior is really hidden inside runtime code instead of source-level structures
- contradiction is just an error string
- defer is only a convenient implementation patch
- observation frontier is only a viewer notion
- host-language libraries can reproduce everything without losing semantic identity

These are not reasons to give up.
They are the right pressure points for development and paper discipline.

## Connection To Existing Project Documents

- [Positioning Matrix](/Users/nagaseibuki/Documents/github-repos/naki0227/world-oriented-programming/docs/positioning-matrix.md) compares `sekai` to nearby traditions
- [Research Strategy And Task Plan](/Users/nagaseibuki/Documents/github-repos/naki0227/world-oriented-programming/docs/research-strategy-and-task-plan.md) turns the critique into executable research tasks
- [Semantics Core](/Users/nagaseibuki/Documents/github-repos/naki0227/world-oriented-programming/docs/semantics-core.md) defines the current semantic account
- [Roadmap](/Users/nagaseibuki/Documents/github-repos/naki0227/world-oriented-programming/docs/roadmap.md) explains where this comparison fits into the broader development plan
