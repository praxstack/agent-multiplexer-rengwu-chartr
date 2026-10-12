---
title: Chartr development wiki
updated: 2026-10-12
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

The migration did not build or exercise Chartr. Later execution is recorded in
dated runs, including the [macOS frame-delivery recheck](verification/runs/2026-10-11-metal-frame-delivery-recheck.md).
Unexecuted recipes remain unvalidated, and historical passes apply only to their
recorded snapshots.

## History and migration

- [Last-window reopen implementation](verification/runs/2026-10-12-window-reopen.md):
  window restoration and shutdown ownership corrected; automated checks and
  native reopen/session/quit checks pass. The unchanged-build failure is preserved.

- [Sidebar CI fixture repair](verification/runs/2026-10-12-sidebar-ci.md):
  stale empty-space expectation corrected while preserving populated-space
  coverage; local checks and the full macOS, Ubuntu and Arch/AUR CI matrix passed.
- [Supplied-drawable implementation](journal/2026-10-11-low-power-implementation.md):
  implemented Low Power Mode fix, with three active captures and selected native
  and app rendering checks passed; aggregate lifecycle failure, latency percentile
  and compatibility limits recorded.
- [Concrete Low Power Mode fix design](journal/2026-10-10-low-power-fix-design.md):
  supplied-drawable frame delivery removes acquisition from the observed blocking
  path; exact code boundaries, lifetime rules and compatibility scope are defined.
- [Low Power Mode evidence and resolution plan](journal/2026-10-10-low-power-frame-investigation.md):
  standalone Metal timing reproduces drawable backpressure; records a bounded
  pacing trial, fallback and acceptance criteria. No product fix applied.
- [Running build lag investigation](journal/2026-10-10-running-build-lag.md):
  battery/AC reproduction localizes the visible stall to main-thread Metal
  drawable waits; frame scheduling needs a fix. Backend overhead is recorded
  separately.
- [Historical archive](archive/index.md): complete research, original rewrite
  specification, inactive Companion protocol, screenshots and receipts.
- [Agreed migration scope](discussions/jstack-migration.md).
- [Implementation and validation journal](journal/2026-10-05-jstack-migration-implementation.md).
- [Round 1 interpretation](journal/2026-10-05-jstack-migration-round-1.md) and
  [round 2 interpretation](journal/2026-10-05-jstack-migration-round-2.md).
- [Round 1 answers](journal/2026-10-05-grill-ab31b76a-3123-4152-9f0b-1c822da69430.md) and
  [round 2 answers](journal/2026-10-05-grill-fa77900d-80f2-4c30-95c5-cf40e9aa1ee9.md).
