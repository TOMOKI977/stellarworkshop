# Feature: workshop-site

## Objective
Turn the "Build With Stellar + AI" talk (Cochabamba, 2026-11-13) into a static website that is BOTH the slide deck and the attendee guide, published on GitHub Pages at `https://tomoki977.github.io/stellarworkshop/`.

## Problem / Why
- The first version lived as a private Claude artifact: attendees could not open it.
- Attendees need to scan a QR, land on the same slide on their phone, and copy commands and prompts.
- Every step must show two paths: **without AI** (manual commands) and **with AI** (one prompt the agent executes).

## Scope
- Slides: cover, about me (QR to linktr.ee/tomoki977), stack, gentle-ai (new), Stellar CLI, Stellar Skills, MCP, demo, hackathon, thanks.
- Each tool slide: "Sin IA" commands vs "Con IA" prompt, plus a QR to its deep link.
- Repo files: `site/` (HTML/CSS/JS + QR SVGs), `prompts/`, `commands/`, `scripts/` (QR generator + structural check), `.github/workflows/pages.yml`, `README.md`.

## Constraints
- Site copy in neutral Spanish (tú), no voseo. Code, comments and identifiers in English.
- No build step; plain static files. No external runtime dependencies except Google Fonts.
- Commands must match official docs (developers.stellar.org, skills.stellar.org, gentle-ai README).
- Conventional commits, no AI attribution.
- Push, PR and merge are the user's decisions.

## Delivery strategy
`ask-on-risk`. Forecast: ~1,200 authored lines (mostly static content), over the 400-line heuristic because one coherent site ships as a unit. Single feature branch `feat/workshop-site`.

## Tasks
- [x] T1 Repo scaffold: git init, feature branch, this document, `.gitignore`. Route: inline (mechanical).
- [x] T2 Content source: `commands/*.md` and `prompts/*.md` per step (sin IA / con IA). Route: delegated writer (2+ non-trivial files). Commit `ae91a88`.
- [x] T3 Site: `site/index.html`, `site/styles.css`, `site/app.js` — keyboard slide mode on desktop, scroll + copy buttons on mobile, deep links `#<slide-id>`. Route: delegated writer. Commit `05a4f96`.
- [x] T4 QR: `scripts/gen_qr.py` generating `site/qr/*.svg` per slide deep link + linktree. Route: delegated writer. Commit `f709b5e`.
- [x] T5 Pages workflow + README with run/deploy instructions. Route: delegated writer. Commits `2ac38cf` (workflow + `scripts/check-site.mjs`), `3118d6d` (README).

## Acceptance criteria
- Opening `site/index.html` shows slides navigable with arrow keys; on a phone it scrolls with readable sections.
- Each tool slide shows "Sin IA" and "Con IA" with copy buttons.
- Every QR decodes to the matching deep link on the Pages URL.
- Structural check script passes.

## Checks
- Test-first exception: static site with no runnable behavioral test runner; proportional structural checks instead (`python scripts/gen_qr.py`, `node scripts/check-site.mjs`).

## Progress / Evidence
- T1: done in commit `643657d`.
- T2: `ae91a88`. Prompts and commands copied verbatim from the brief; voseo scan (`rg` for vos/tenés/podés…) found none.
- T3: `05a4f96`. `node --check site/app.js` OK; voseo scan clean.
- T4: `f709b5e`. `python -I scripts/gen_qr.py` wrote 8 SVGs (site, linktree, repo, gentle-ai, cli, skills, mcp, demo). Every SVG was rasterized and decoded with OpenCV's `QRCodeDetector`; all 8 decode to their expected URLs.
- T5: `2ac38cf`, `3118d6d`. `node scripts/check-site.mjs` → `OK: 46 checks`. Negative test: altering the demo prompt in `index.html` → `FAIL: 1 of 46 checks failed` (exit 1), then restored.

## Next step
All tasks done. Push `feat/workshop-site`/`main`, create the GitHub repo and enable Pages (Source: GitHub Actions): user's decision. Optional: visual check of the deck on a projector and a phone.
