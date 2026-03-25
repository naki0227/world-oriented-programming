# 2026-03-25 Phase G5 Minimal Formalization

## Summary

Added a first minimal formalization of the semantic objects used in Phase G5.

## What Was Added

- a world tuple:
  `W_t = (E, G, L, tau, sigma, t)`
- an event object:
  `ev = (k, p, t_ev)`
- a law object:
  `ell = (k_ell, pol_ell, pred_ell, act_ell)`
- an observation operator:
  `Obs : W x T -> S union {X}`
- an explicit admissibility predicate:
  `Adm(W_t)`

## Why It Matters

This pass is the first one that names the core semantic objects in a definition-like style instead of only prose.
It is still intentionally lightweight, but it changes the status of the semantics from trajectory to minimal formal core.

## Paper Integration

A short summary of this minimal formalization was also added to `paper/main.tex` so the manuscript reflects the new semantic maturity.
