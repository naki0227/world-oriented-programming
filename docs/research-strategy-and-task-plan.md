# Research Strategy And Task Plan

Last updated: 2026-04-18

## Purpose

This document turns the current critique of `sekai` into an executable research plan.

The project should not present itself as a completed revolutionary language yet.
It should present itself as a serious, still-developing attempt to make executable world description into a programming model.

The next phase therefore has one central goal:

> Make World-Oriented Programming defensible as a distinct language model, not merely as a small DSL with nice examples.

That requires progress on four fronts at the same time:

- a sharper novelty claim
- a more formal semantic core
- a fairer and more convincing evaluation
- a larger representative vertical slice that existing systems would express awkwardly

## Strategic Thesis

The strongest version of the project is not:

> Programs do not need update loops.

Many existing systems can already hide update loops.

The stronger thesis is:

> A program can define an observable spatial-temporal world whose entities, laws, events, contradictions, local synchronization, and underdetermined continuations share one executable semantic model.

This thesis gives the project a more precise identity than "declarative simulation" alone.
It also gives related-work comparisons a stronger target.

## Non-Negotiable Research Standards

From this point onward, every major claim should satisfy at least one of these standards.

1. **Semantic standard**
   The claim is tied to a named semantic object such as world frontier, event, law, admissibility, synchronization scope, observation, contradiction, or convergence.

2. **Implementation standard**
   The claim is demonstrated by a runnable `.sk` example, a structured JSON report, and at least one regression test or reproducible command.

3. **Evaluation standard**
   The claim is compared against at least one fair baseline that is not intentionally naive.

4. **Interface standard**
   If the claim involves diagrams or inspection, it is visible in the viewer without depending on informal explanation.

## Immediate Cleanup

These tasks reduce avoidable credibility loss.

### Task C1. Remove stale conflict markers

- File: `docs/model.md`
- Action: remove Git conflict markers and preserve both Phase G2 and Phase G3 semantic refinements.
- Done when: a search for Git conflict-marker lines in `docs/` returns no matches.

### Task C2. Stop saying "completed" where the research is only prototype-complete

- Files: `README.md`, `docs/roadmap.md`, paper framing
- Action: use "prototype-complete", "first executable slice", or "seed implementation" for narrow features.
- Done when: public-facing text no longer implies a finished general-purpose language.

### Task C3. Separate paper-ready claims from research-log momentum

- Files: `paper/main-body.tex`, Phase I/J/K docs
- Action: keep the seed paper focused on the minimum coherent model; move sprawling future-facing material into docs or appendix.
- Done when: every subsection in the paper supports the same central thesis.

## Pillar A. Novelty And Positioning

Goal: make the project clearly different from nearby traditions without overstating originality.

### Task A1. Write a formal "nearest neighbors" matrix

- New file: `docs/positioning-matrix.md`
- Compare against: Modelica, Simulink, FRP, CLP, game engines/ECS, physics engines, actor systems, GeoGebra, LabVIEW, constraint-based animation.
- Axes:
  - first-class space
  - continuous or event-frontier time
  - constraints as admissibility laws
  - contradictions as world-level results
  - local synchronization scope
  - underdetermined continuation/convergence
  - diagram-to-logic continuity
- Done when: each system has both a respectful overlap statement and a precise difference statement.

### Task A2. Replace broad novelty language with a narrow claim

- Files: `paper/main-body.tex`, `docs/public-facing-summary.md`
- Action: change the paper's claim from "new paradigm" to "integrated executable world model".
- Done when: the introduction can survive the question, "How is this different from Modelica plus a viewer?"

### Task A3. Define what World-Oriented Programming is not

- New section in `docs/philosophy.md`
- Non-goals:
  - not a general physics engine
  - not a full constraint logic language
  - not just visual programming
  - not an agent behavior tree replacement yet
  - not a finished proof system
- Done when: limitations sound intentional rather than defensive.

## Pillar B. Semantic Core

Goal: make the language model precise enough that "world-oriented" is not only a metaphor.

### Task B1. Consolidate the operational semantics

- Files: `docs/model.md`, new `docs/semantics-core.md`
- Define:
  - `W_t`
  - entity state
  - geometry set
  - hard law set
  - soft preference set
  - event frontier
  - `Next(W_t)`
  - `Sync(ev)`
  - `Fire`
  - `Enforce`
  - `Contradict`
  - `Observe`
  - `Conv(W_t^?)`
- Done when: paper semantics can cite one stable notation source instead of several phase notes.

### Task B2. Turn proof obligations into small lemmas

- File: `docs/semantics-proof-obligations.md`
- Start with three limited claims:
  - deterministic event order for the supported event set
  - snapshot determinism for deterministic laws and fixed observation times
  - termination for the current finite candidate model
- Done when: each claim has assumptions, statement, proof sketch, and known counterexamples outside scope.

### Task B3. Specify simultaneous-event behavior

- Files: `orbis/src/world.rs`, `docs/semantics-core.md`
- Action: document and test what happens when collision, speed repair, and region law become relevant at the same frontier.
- Done when: simultaneous or competing laws are not described as "underspecified" in the seed-paper limitation section.

### Task B4. Make contradiction a semantic result, not an error label

- Files: runtime report types, JSON output docs, viewer docs
- Action: represent contradiction as `W_t^X` with law id, participants, frontier, and failed admissibility predicate.
- Done when: all contradiction reports expose the same structured fields.

## Pillar C. Fair Evaluation

Goal: prevent the evaluation from looking designed to make `sekai` win.

### Task C-E1. Replace one-sided declarative density

- File: `scripts/spec_metrics.py`
- Action: classify both `sekai` and baselines into common categories:
  - world declarations
  - law declarations
  - observation requests
  - update mechanics
  - event detection
  - repair logic
  - branching/selection logic
  - reporting/boilerplate
- Done when: imperative code can receive credit for declarative setup lines instead of always getting density `0.000`.

### Task C-E2. Add stronger baselines

- Directory: `benchmarks/`
- Add at least three baseline families:
  - compact imperative Python: present
  - event-driven Python: present
  - library-style simulation skeleton or pseudocode baseline: present
- Optional later baselines:
  - Modelica-like model
  - FRP-like model
  - ECS/game-engine-style model
- Done when: the paper can say "we compare against several reasonable expression styles," not only "a compact imperative baseline."

### Task C-E3. Add qualitative coding guidelines

- New file: `docs/evaluation-coding-manual.md`
- Action: define how LOC, tokens, update mechanics, world content, and law content are counted.
- Done when: another person could reproduce the categories without asking the author.

### Task C-E4. Choose one flagship scenario for deep comparison

- Candidate: visibility-conditioned corridor with multiple agents, law conflict, deferred convergence, and contradiction report.
- Done when: one scenario demonstrates why the integrated model matters more than any single toy example.

### Task C-E5. Remove unsupported cognitive-load claims

- Files: paper and Phase K docs
- Action: unless a user study is run, say "representational structure" rather than "cognitive load".
- Done when: every human-factor claim is either measured or explicitly labeled as a hypothesis.

## Pillar D. Flagship Vertical Slice

Goal: build one example that makes the language feel necessary.

The current corpus has many small examples.
The next stage needs one larger scenario that combines the project's distinctive features.

### Target Scenario: Visibility-Guided Local Coordination

World:

- several moving entities
- corridor or room geometry
- one or more occluders
- forbidden regions
- speed or safety laws
- visibility-conditioned candidate actions
- local synchronization when entities interact
- contradiction when a hard world law fails
- observation checkpoints showing determinate, representative, and unresolved states

Why this scenario matters:

- Modelica-like systems can express dynamics but not naturally diagram-to-law branching.
- FRP-like systems can express time but not naturally admissible world laws.
- game engines can implement it but usually bury laws in update code.
- CLP-like systems can express constraints but not naturally event-frontier world evolution.

### Task D1. Design the flagship `.sk`

- New example: `examples/visibility_coordination_flagship.sk`
- Status: present.
- Done when: the example uses at least four distinctive language features together.

### Task D2. Implement missing runtime support only when the flagship demands it

- Avoid feature creep.
- Add geometry or event features only if the flagship scenario needs them.
- Done when: every new runtime feature has a corresponding flagship use.

### Task D3. Add a structured report fixture

- New sample: `viewer/samples/visibility_coordination_flagship.json`
- Status: present.
- Done when: the viewer can inspect the flagship without rerunning the CLI.

### Task D4. Add a paper figure from the flagship

- File: `docs/paper-figures.md`
- Done when: the figure shows the integrated world model, not just a pretty scene.

## Pillar E. Viewer As Research Instrument

Goal: make the viewer prove that one world model connects diagrams, laws, execution, and contradiction.

### Task E1. Event-frontier timeline

- Files: `viewer/index.html`, `viewer/app.js`, `viewer/styles.css`
- Add:
  - time scrubber
  - event markers
  - law labels
  - contradiction markers
- Done when: a reader can see where laws fire without reading JSON.

### Task E2. Candidate branch inspection

- Show top candidate continuations at decision frontiers.
- Label hard-law rejection, repair, soft-preference choice, tie-break, and deferral.
- Done when: underdetermined worlds are visually inspectable rather than only logged.

### Task E3. Geometry relation overlay

- Show line of sight, blockers, forbidden regions, and collision/contact frontiers.
- Done when: visibility-conditioned branching is visually obvious.

## Pillar F. Language And Runtime Hardening

Goal: stop the prototype from looking like a hand-built demo runner.

### Task F1. Add regression tests for all paper examples

- Use `cargo test` where possible.
- Cover:
  - bounce
  - forbidden region
  - two-body collision
  - candidate fallback
  - candidate repair
  - deferred resolution
  - visibility occlusion
  - flagship scenario
- Done when: paper examples cannot silently drift.

### Task F2. Validate DSL errors

- Add tests for malformed declarations, missing properties, invalid normals, unknown constraints, and invalid observation times.
- Done when: the language fails as a language, not as a demo parser.

### Task F3. Stabilize JSON schema

- File: `docs/output-format.md`
- Define versioned report fields.
- Done when: viewer and paper scripts do not depend on accidental runtime structure.

### Task F4. Add law identifiers

- Runtime laws should carry stable ids and source spans where possible.
- Done when: reports can connect a contradiction or repair to a specific source law.

## Pillar G. Paper Strategy

Goal: turn the paper from a broad manifesto into a defensible seed paper.

### Task G1. Cut or move sprawling convergence material

- File: `paper/main-body.tex`
- Action: keep Phase I material short unless it is central to the flagship.
- Done when: the paper has one main story.

### Task G2. Reframe evaluation modestly

- Replace:
  - "shows the model is better"
- With:
  - "shows a measurable shift in representational burden for selected spatial-temporal scenarios"
- Done when: the paper does not overclaim.

### Task G3. Add a limitations table

- Include:
  - geometry is narrow
  - semantics are partial
  - baselines are still limited
  - no user study yet
  - solver is not general
- Done when: limitations increase trust rather than weakening the paper.

### Task G4. Add a "why not existing systems?" section

- Use the positioning matrix.
- Done when: the related-work section can answer hostile comparison questions directly.

## Suggested Execution Order

### Week 1: Credibility And Framing

- C1: remove conflict markers
- A1: positioning matrix
- B1: semantics core skeleton
- C-E1: fairer metric categories
- G2: modest evaluation wording

### Week 2: Formal And Evaluation Backbone

- B2: proof-obligation sketches
- C-E2: stronger baselines
- C-E3: coding manual
- F1: regression tests for existing paper examples

### Week 3: Flagship Design

- D1: design flagship scenario
- D2: implement only missing runtime support
- F3/F4: schema and law identifiers

### Week 4: Viewer Evidence

- E1: event-frontier timeline
- E2: candidate branch inspection
- E3: geometry overlays
- D3/D4: flagship report and figure

### Week 5: Paper Consolidation

- G1: cut scope creep
- G3: limitations table
- G4: stronger related work
- final paper pass against the strategic thesis

## Definition Of Real Progress

The language becomes genuinely stronger when all of the following are true:

- a skeptical reader can state what `sekai` is that existing systems are not
- the core semantics can be read without relying on metaphor
- the evaluation does not depend on an unfair baseline
- one flagship scenario makes the integrated model feel necessary
- the viewer exposes semantic structure rather than merely rendering positions
- every paper claim has either a runnable artifact, a formal statement, or a clearly labeled limitation
