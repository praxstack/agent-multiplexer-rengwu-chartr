---
title: Decisions
updated: 2026-10-05
---
# Decisions

These migration decisions were accepted in the
[first interview](../journal/2026-10-05-grill-ab31b76a-3123-4152-9f0b-1c822da69430.md)
and clarified in the
[second interview](../journal/2026-10-05-grill-fa77900d-80f2-4c30-95c5-cf40e9aa1ee9.md).
The original product ADRs are linked below. Each migration decision below is dated
2026-10-05 and remains accepted unless explicitly superseded.

| ID | Decision | Context and basis |
| --- | --- | --- |
| DEC-JSTACK-001 | Migrate development documentation and process; retain Wayfinder product behavior. | Q1 explicitly limits the scope. Wayfinder's map/ticket format is a separate product contract. |
| DEC-JSTACK-002 | Keep public guides outside `docs/wiki/`; use that wiki for development records. | Q2 explicitly preserves public guides under `docs/`, outside the wiki subtree. This maintains separate user and contributor audiences. |
| DEC-JSTACK-003 | Use current implementation as the migration baseline when older documentation disagrees. | Q3 selects implementation over previous specifications for this migration. Record the inspected revision and dirty state; source inspection does not establish successful execution. This is a migration baseline, not an ongoing rule to redefine regressions as correct. |
| DEC-JSTACK-004 | Preserve a full browsable historical archive in the wiki. | Q4 selects the full archive. Preserve dates, original claims, evidence, and supersession context while adapting links and presentation. |
| DEC-JSTACK-005 | Make needed reader improvements in shared jstack, with lean and simple implementations. | Q5 selects shared extensions and asks what changes are needed. Q8 and Q9 settle the extension scope in DEC-JSTACK-007 and DEC-JSTACK-008. Avoid a chartr-specific reader fork. |
| DEC-JSTACK-006 | Use an advisory development workflow with optional records. | Q7 explicitly selects advisory use. No mandatory interview/report sequence or new enforcement gate is established by this choice. |
| DEC-JSTACK-007 | Add simple inline local images to the shared jstack reader. | Q8 accepts standard Markdown image rendering for local PNG/JPEG/GIF/WebP attachments, reusing validation and hashing with responsive sizing and necessary sanitizer/content-policy changes. Keep files alongside the HTML and verify rendering. |
| DEC-JSTACK-008 | Use GitHub URLs for references outside the wiki. | Q9 rejects the need for a local repository-link extension. Pin historical references to recorded revisions where available. This preserves the lean reader while keeping public guides separate. |
| DEC-JSTACK-009 | Validate documentation and changed-reader behavior for migration completion; fresh chartr execution is not required. | Q10 resolves the qualification in Q6. Check content fidelity, links, preserved history/evidence, wiki build/freshness, and tests plus browser rendering for reader changes. Imported verification recipes are unexecuted until actually run. |

The [question register](../discussions/open-questions.md) preserves the resolved
questions. No consequential migration-scope question remains open after round 2.

## Product architecture decisions

Original files and numeric identities are preserved in the [ADR register](adr/index.md).
Their original prose supplies context and rationale; no missing original decision
dates have been invented during import.

| Identity | Decision | Current connection |
| --- | --- | --- |
| ADR-0001 | [Private Herdr](adr/0001-a-private-herdr.md) | [Session lifecycle](../features/session-lifecycle.md) |
| ADR-0002 | [Zed layer](adr/0002-the-zed-layer.md) | [Terminal](../features/terminals.md), [Settings](../features/settings.md) |
| ADR-0003 | [Plugin runtimes](adr/0003-two-plugin-tiers.md) | Hosted tier superseded by ADR-0007; retain rejection rationale |
| ADR-0004 | [Complete terminal stack](adr/0004-the-zed-terminal-stack.md) | [Terminal interaction](../features/terminals.md) |
| ADR-0005 | [Multi-workspace ownership](adr/0005-spaces-follow-zed-multi-workspace.md) | [Spaces](../features/spaces.md), [Panes](../features/panes.md) |
| ADR-0006 | [Native services and web Wayfinder](adr/0006-native-plugin-services-and-wayfinder.md) | [Plugin services](../features/plugin-runtime.md), [Wayfinder](../features/wayfinder.md) |
| ADR-0007 | [Prebuilt native surfaces](adr/0007-prebuilt-native-surfaces.md) | [Plugin packages](../features/plugin-packages.md), [Runtime](../features/plugin-runtime.md) |
