---
title: Development workflow
section: Development
updated: 2026-10-05
---
# Development workflow

Use jstack as an advisory set of tools for the task at hand. Public guides remain
in `docs/` and plugin READMEs; development records live here. The root
[AGENTS.md](https://github.com/rengwu/chartr/blob/main/AGENTS.md) is the short entry
point for future agents. Existing CI and release gates continue unchanged.

## Resume and choose the right skill

Read the [home](../index.md), relevant [decisions](../decisions/index.md),
[open questions](../discussions/open-questions.md), latest
[journal](../journal/2026-10-05-jstack-migration-implementation.md), and affected
[feature records](../features/index.md). Consult the [code map](code-map.md) for
implementation owners. Preserve pre-existing work and identify the tested tree
when verification is needed.

| Work | jstack skill | Useful record |
| --- | --- | --- |
| Discuss scope and tradeoffs | `plan` | Accepted decisions, unresolved questions, dated journal |
| Interview to resolve consequential choices | `grill-me`, optionally `grill-form` | Answers and their interpretation; not a mandatory kickoff |
| Maintain user-facing behavior and recipes | `feature-map` | One coherent feature page with a stable ID |
| Verify a behavior change when needed | `verify-change` | Exact build, selected scenarios, observed outcomes and evidence |
| Repair stale recipes or maps | `maintain-verification` | Documentation/tooling corrections, separately reported product failures |
| Edit, link or render these records | `wiki` | Markdown, attachments and generated reader |

For a small editorial change, update the affected document and validate its
links/rendering. For a behavior change, update the relevant public explanation
and development feature record when useful; select tests and live scenarios
proportionate to the change. A design interview or new run report is optional,
not a prerequisite for every edit. Release work still follows
[acceptance](../verification/acceptance.md) and [packaging](releasing.md).

## Keep records honest

Markdown is authoritative. Distinguish proposals, accepted decisions, source
observations and executed results. Keep the recipe and expected behavior together
on the feature page; link from planning pages rather than duplicating a spec.
Retain stable IDs and preserve superseded decisions and failed run history.

The migration adopted the inspected current implementation as its baseline.
That is not permission to redefine a future regression as correct. When a later
change conflicts with agreed behavior, record the conflict and resolve intent.

When a run report is useful, use `verification/runs/<unique-run-id>.md` and
`evidence/<run-id>/`. Record revision plus dirty fingerprint, selected scope,
expected/observed outcomes and evidence. Outcomes are `Passed`, `Failed` or
`Untested`: any exercised failure makes the aggregate Failed; otherwise any
selected unknown/skipped path makes it Untested. A pass covers only its build
and selected scenarios. Do not convert imported historical counts into a new run.

## Tool availability and reproducibility

The reusable skills come from the [jstack plugin](https://github.com/rengwu/jstack).
Load the collection together so its relative references remain available. Resolve
its actual installed location when invoking a skill script; chartr does not
require a sibling `../jstack` checkout. Product records and project-specific
verification tools belong in chartr, never in the plugin installation.

The wiki's checked-in `_build.mjs` is a copy of the shared jstack asset with its
bundled licenses. Node.js 22+ is enough; no package installation, package file,
server or network is needed to build and read the wiki:

```sh
node docs/wiki/_build.mjs
node docs/wiki/_build.mjs --check
```

If new verification reports were written, run the plugin's metadata checker:

```sh
# Set this to the actual jstack plugin root in your environment.
JSTACK_DIR=/absolute/path/to/jstack
node "$JSTACK_DIR/skills/verify-change/scripts/check-reports.mjs" docs/wiki
```

An empty/missing run directory is intentional until a product run exists; do not
create a dummy Passed report to satisfy the checker. The checker validates outcome
metadata, not observation truth or full report completeness.

Commit source Markdown, attachments and the rebuilt `index.html` together when
committing is in scope. CI has no new mandatory jstack gate. To update the reader,
compare the installed shared asset, preserve any intentional local changes and
licenses, copy it, rebuild and check. This migration adds no chartr-specific fork.

## Reading and linking

Open `docs/wiki/index.html` locally. Inline PNG/JPEG/GIF/WebP images and linked
files live beside it under `attachments/` or `evidence/`; share those directories
with the HTML. Wiki pages use relative Markdown links. References to public guides
or source outside the wiki use GitHub URLs; pin historical source references when
their recorded revision is known. External links need network access and are not
validated for availability by the reader.

The [archive](../archive/index.md) contains historical proposals, removed behavior
and past audits. Current user-facing behavior is summarized in the feature map.
Wayfinder's product `.plan/maps/` format remains unchanged; it is not the storage
format for this development wiki.
