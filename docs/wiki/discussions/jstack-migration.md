---
title: Jstack migration
updated: 2026-10-05
status: Accepted
---
# Jstack migration

## Accepted scope

The [decision register](../decisions/index.md) records the accepted choices.
Migrate chartr's development documentation and workflow to jstack while keeping
public guides outside the wiki and Wayfinder's product behavior intact. Use the
current implementation as the migration baseline, preserve a browsable historical
archive, and keep the workflow advisory. Shared reader changes should be lean.

## Source findings

The initial repository review found 34 Markdown pages under `docs/` before this
wiki was created, plus 16 tracked Markdown files elsewhere outside vendor code.
The main development sources include the 422-line workspace specification with
97 numbered stories, seven ADRs, the 317-line acceptance checklist, the code map,
release instructions, and dated research/audits. These counts describe the review
snapshot, not a continuing invariant.

Chartr was at `c853533c` with pre-existing modifications in actions, app state,
command palette, app view, and keymap files. Jstack was at `29816b8`. The initial
review ran all 30 jstack tests successfully and did not build or exercise chartr.

The original reader dry run used a temporary copy of `docs/` and failed at a link
to a plugin README outside the wiki. An initial scan found 61 relative link
occurrences outside `docs/`, seven inline images, and 14 local non-Markdown
attachment links. These figures cover the old whole-docs scope; keeping public
guides separate reduces what the wiki must handle.

## Accepted reader work

Support standard Markdown inline images for local PNG/JPEG/GIF/WebP attachments.
Reuse attachment containment checks and hashing, add image rendering and sizing,
and adjust sanitizer and content policy deliberately. Keep the reader offline
and dependency-free; verify actual image display in a browser as well as tests.

Use GitHub URLs for links to public guides and source files outside the wiki.
Pin historical references to their recorded revisions where available. These
links already work and require no reader extension. The user selected this over
adding local repository-link handling or rendering outside Markdown in the wiki.

Existing support already covers flat metadata, tables, code blocks, search,
backlinks, and JSON/image evidence files in attachment directories. No new
framework, package installation, database, or metadata schema is proposed.

## Accepted migration validation

Validate the migrated content, internal links, history/evidence preservation,
reader build/check, and changed jstack behavior. Include automated reader tests
and a browser rendering check for inline images. Imported app verification
recipes remain unexecuted until actually run. Fresh chartr builds, live checks,
and acceptance runs are not migration prerequisites. Q10 explicitly resolves
the earlier Q6 selection and its accompanying challenge to app testing.

## Completion criteria

- Current development records have one authoritative home in the wiki; public
  guides remain outside its subtree, with navigation updated for moved content.
- Feature records describe the inspected current implementation, link to relevant
  decisions and recipes, and distinguish source review from execution evidence.
- Historical records and attachments remain browsable with dates, claims, and
  provenance preserved. Old results are not promoted to current verification.
- Shared jstack supports simple inline local images, with its required tests and
  browser rendering validation completed; chartr uses that shared reader asset.
- Cross-boundary references use GitHub URLs and internal links resolve.
- The advisory workflow is documented without mandatory per-change interviews,
  reports, or new process-enforcement CI gates.
- The wiki builds and passes its freshness/structure check; no fresh application
  execution is needed to declare this documentation migration complete.

## Implementation status

The migration is implemented. The [question register](open-questions.md) has no
remaining consequential scope questions. See the
[implementation journal](../journal/2026-10-05-jstack-migration-implementation.md)
for validation evidence, preserved history and explicit limits.
