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
[`34306480934`](https://github.com/tdimino/open-rebellion/actions/runs/34306480934)
built, uploaded, and deployed documentation from `main` at commit
`aaf428d286e482471662881124fbad78062fce6f`.

Public HTTP smoke tests returned 200 for:

- [Project landing page](https://tdimino.github.io/open-rebellion/)
- [Full-functionality audit index](https://tdimino.github.io/open-rebellion/docs/qa/2026-09-08-full-functionality-audit/)
- [Fleet miniature browser proof](https://tdimino.github.io/open-rebellion/docs/qa/2026-09-08-full-functionality-audit/evidence/2026-09-08-fleet-miniatures.html)

The audit entry point is deliberately named lowercase `index.md`; this makes
the directory URL above resolve on GitHub Pages instead of requiring the
case-sensitive `INDEX.html` path. The recorded run completed all build,
artifact, status-reporting, and deploy jobs. The only annotation was GitHub's
platform notice that the bundled
`actions/upload-artifact@v4` Node 20 action was forced onto Node 24; it did not
affect the deployment.
