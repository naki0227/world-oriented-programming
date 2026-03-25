# Research Log: 2026-03-25 Paper Appendix Setup

## Session Goal

Keep the main paper tighter while preserving non-essential interface figures for appendix or supplementary use.

## Actions Taken

- created `paper/appendix.tex`
- moved the omitted viewer overview and candidate-generation figures into an appendix-oriented LaTeX file
- updated `paper/README.md` so the paper directory now documents both main and appendix paths

## Expected Value

- the main paper can stay focused on the strongest argumentative figures
- supplementary material remains ready for reviewer questions, talks, or later expansion
- figure decisions are now reversible without redoing layout work

## Verification Note

- appendix setup was created first and then compiled successfully in a later Phase E pass
- the figure assets it references already exist in `figures/`

## Next Recommended Step

Continue polishing `paper/main.tex` for submission quality, while keeping `appendix.tex` available for supplementary reviewer-facing material.
