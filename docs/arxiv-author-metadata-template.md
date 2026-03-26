# arXiv Author Metadata Template

Fill this in before switching from the anonymous review manuscript to a public arXiv version.

## Required

- paper title
- author names
- affiliations
- email addresses

## Optional But Recommended

- ORCID identifiers
- project or lab homepage
- acknowledgements / funding text

## LaTeX Sketch

```tex
\author{Author Name}
\affiliation{
  \institution{Institution Name}
  \city{City}
  \country{Country}
}
\email{author@example.com}
```

## Reminder

The current `paper/main.tex` intentionally stays in anonymous review mode.
For an arXiv release, create a named-public variant only after the metadata above is fixed.
