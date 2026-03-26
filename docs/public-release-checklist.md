# Public Release Checklist

This checklist is for releasing `sekai` publicly before arXiv endorsement is available.

## GitHub

- push a clean `main`
- make sure `README.md` reflects the current seed-paper status
- verify that the repository `About` text matches the one-sentence description
- confirm that `paper/main-public.tex` builds locally
- confirm that `paper/main.pdf` and `paper/main-public.pdf` exist

## Public Assets

- keep the public project summary in `docs/public-facing-summary.md`
- keep paper-ready figures under `figures/`
- keep visibility screenshots in the paper-assets list
- keep `scripts/prepare_arxiv_package.sh` working even if arXiv submission is delayed

## Citation And Archive Metadata

- keep `CITATION.cff` up to date
- keep `.zenodo.json` up to date
- make sure the named-public manuscript uses the current author metadata

## Outreach Package

- prepare one PDF: `paper/main-public.pdf`
- prepare one repository link: `https://github.com/naki0227/world-oriented-programming`
- prepare one short summary from `docs/public-facing-summary.md`
- prepare one endorsement-request message for researchers in nearby fields

## Suggested Order

1. publish the latest `main` to GitHub
2. check that the repository landing page and summary are coherent
3. connect the repository to Zenodo if desired
4. share the PDF and repository with potential endorsers
5. return to arXiv once endorsement is available
