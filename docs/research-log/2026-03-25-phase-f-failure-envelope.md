# Research Log: 2026-03-25 Phase F Failure Envelope

## Session Goal

Preserve constraint and activity context even when execution ends in contradiction.

## Actions Taken

- added `simulate_program_envelope(...)` as a runtime entry point that returns structured success or failure results
- updated CLI `simulate-report` to use the envelope path directly
- preserved partial snapshots, active constraints, and recorded activities on failure
- extended tests and output-format documentation accordingly

## Outcome

- contradiction reports are now more useful for debugging and research demos
- failed worlds no longer discard the law metadata that explains why the failure happened
- the viewer can, in principle, display law context even for error cases

## Verification Note

- `cargo test` passed after the failure-envelope update

## Next Recommended Step

Use the richer failure envelope inside the viewer so contradiction screens can show the exact law and activity trace that led to failure.
