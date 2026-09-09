---
title: "GitHub Pages Documentation Proof"
description: "Deployment and public smoke-test evidence for P40"
category: qa
created: 2026-09-08
updated: 2026-09-08
tags: [qa, documentation, github-pages, deployment]
---

# GitHub Pages Documentation Proof

P40 passes. GitHub Pages deployment run
[`34296165296`](https://github.com/tdimino/open-rebellion/actions/runs/34296165296)
built, uploaded, and deployed documentation from `main` at commit
`52e646ae53a69cd6695ad4233db36f4dfd9bdb53`.

Public HTTP smoke tests returned 200 for:

- [Project landing page](https://tdimino.github.io/open-rebellion/)
- [Full-functionality audit index](https://tdimino.github.io/open-rebellion/docs/qa/2026-09-08-full-functionality-audit/)
- [Fleet miniature browser proof](https://tdimino.github.io/open-rebellion/docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-fleet-miniatures.html)

The first run for the documentation commit built successfully but GitHub
canceled its artifact upload when the amended README commit superseded it. The
newer run completed all build, artifact, status-reporting, and deploy jobs.
The only annotation was GitHub's platform notice that the bundled
`actions/upload-artifact@v4` Node 20 action was forced onto Node 24; it did not
affect the deployment.
