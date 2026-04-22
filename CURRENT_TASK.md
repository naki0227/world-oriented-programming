# Current Task

**Session Date**: 2026-04-20  
**Focus Area**: flagship vertical slice, paper evidence, and fair comparison

Strategic task plan:

- `docs/research-strategy-and-task-plan.md`

The project is now past the old "viewer v2 mockup" task.
The current work is to turn one strong executable scenario into the center of the research argument.

---

## Current Flagship

- source: `examples/visibility_coordination_flagship.sk`
- contradiction source: `examples/visibility_coordination_flagship_contradiction.sk`
- viewer sample: `viewer/samples/visibility_coordination_flagship.json`
- contradiction sample: `viewer/samples/visibility_coordination_flagship_contradiction.json`
- static figure: `figures/visibility_coordination_flagship-xy.png`
- vector fallback: `figures/visibility_coordination_flagship-xy.svg`
- caption: `figures/visibility_coordination_flagship-caption.md`
- explanation: `docs/flagship-scenario.md`

The scenario demonstrates:

- visibility-conditioned preferences
- hard speed laws
- deferred ambiguity
- two candidate-bearing entities
- staggered convergence
- observation timeline: `unresolved(2) -> unresolved(1) -> determinate(0)`

Important distinction:

- semantic observations occur only at explicit `observe:` frontiers
- viewer playback interpolates between snapshots only for visual readability
- candidate ambiguity and convergence remain tied to semantic frontiers

---

## What I'm Doing Now

### Primary Task

**Make the flagship scenario paper-credible.**

Done in the current pass:

- [x] Added smooth viewer playback between discrete semantic frontiers
- [x] Added the flagship sample to the viewer sample list
- [x] Generated `figures/visibility_coordination_flagship-xy.png`
- [x] Added SVG fallback support to `scripts/render_figure.py`
- [x] Added a flagship caption
- [x] Inserted the flagship figure into the LaTeX paper draft
- [x] Documented the semantics/display distinction
- [x] Added first manual flagship comparison notes
- [x] Turned the manual flagship comparison into a compact LaTeX table
- [x] Added a separate contradiction-bearing flagship variant
- [x] Added the contradiction variant to the viewer samples
- [x] Added a regression test for the blocked visibility observer case

Next tasks:

- [ ] Add an event-driven flagship baseline only if it clarifies the story
- [ ] Decide whether the contradiction variant belongs in the main paper, appendix, or viewer-only evidence
- [ ] Check whether the paper should replace the older visibility triptych with the flagship figure in the main body

---

## Immediate Verification Commands

```text
cargo test
python3 scripts/spec_metrics.py --markdown
PYTHONPYCACHEPREFIX=/tmp/sekai-pycache python3 -m py_compile scripts/render_figure.py scripts/spec_metrics.py
PYTHONPATH=/tmp/sekai-py python3 scripts/render_figure.py examples/visibility_coordination_flagship.sk --output figures/visibility_coordination_flagship-xy.png
cargo run -p sekai-cli -- simulate-report examples/visibility_coordination_flagship_contradiction.sk
```

Viewer:

```text
python3 scripts/viewer_server.py --port 8000
```

Open:

```text
http://127.0.0.1:8000/viewer/
```

Choose `visibility_coordination_flagship`, then use Play.
Choose `visibility_coordination_flagship_contradiction` to inspect the blocked observer report.

---

## Next Research Question

The strongest next critique target is no longer "can this run?"
It is:

> Does `sekai` make the semantic identity of the world clearer than a favorable host-language library baseline?

Line counts are secondary.
The next evaluation pass should compare source identity, report integration, observation-frontier structure, and how much of the world is recoverable from the program text without reading update mechanics.

---

**Last Updated**: 2026-04-20
