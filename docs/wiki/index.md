---
title: Chartr development wiki
updated: 2026-10-05
---
# Chartr development wiki

Development knowledge for the current chartr working tree. Use the
[public documentation](https://github.com/rengwu/chartr/blob/main/docs/README.md)
for user and plugin-author guides. Markdown is authoritative; `index.html` is the
locally generated reader.

## Start here

- [Development workflow](development/workflow.md): choose the jstack skills that
  fit the task; records and interviews are advisory.
- [Feature map](features/index.md): 15 coherent behavior records, with the complete
  [97-story crosswalk](features/story-crosswalk.md).
- [Code map](development/code-map.md): implementation owners and boundaries.
- [Decisions](decisions/index.md), [architecture rationale](decisions/adr/index.md),
  and [open questions](discussions/open-questions.md).

## Verification and releases

- [Shared setup](verification/setup.md): exact build identity, instance isolation,
  fixtures, available tests and cleanup.
- [Release acceptance](verification/acceptance.md) and
  [packaging runbook](development/releasing.md).

**The app recipes are unvalidated.** This migration did not build or exercise
chartr. Historical passes apply only to their recorded snapshots. No synthetic
product run reports were created.

## History and migration

- [Historical archive](archive/index.md): complete research, original rewrite
  specification, inactive Companion protocol, screenshots and receipts.
- [Agreed migration scope](discussions/jstack-migration.md).
- [Implementation and validation journal](journal/2026-10-05-jstack-migration-implementation.md).
- [Round 1 interpretation](journal/2026-10-05-jstack-migration-round-1.md) and
  [round 2 interpretation](journal/2026-10-05-jstack-migration-round-2.md).
- [Round 1 answers](journal/2026-10-05-grill-ab31b76a-3123-4152-9f0b-1c822da69430.md) and
  [round 2 answers](journal/2026-10-05-grill-fa77900d-80f2-4c30-95c5-cf40e9aa1ee9.md).
