# Research Log: 2026-03-25 Paper Template Setup

## Session Goal

Create the first ACM/SIGPLAN-oriented LaTeX paper setup for the project.

## Actions Taken

- added `paper/main.tex` as an `acmart`-based manuscript entry point
- added `paper/references.bib` with BibTeX entries derived from the current manuscript references
- added `paper/README.md` with setup notes and expected next steps
- moved the core paper structure, figures, citations, and bibliography calls into LaTeX form

## Expected Value

- the paper can now move from markdown drafting toward actual venue template work
- figure placement and bibliography are now represented in the format expected by ACM/SIGPLAN workflows
- future polishing can happen inside the medium that will likely be submitted

## Verification Note

- `tectonic` was installed successfully through Homebrew
- `paper/main.tex` compiled successfully and produced `paper/main.pdf`
- the build still reports many layout warnings, especially overfull boxes, so the manuscript now needs a LaTeX-specific typography pass rather than structural bootstrapping

## Next Recommended Step

Review `paper/main.pdf`, reduce overfull boxes, and decide which wide figures should remain in the main paper versus appendix material.
