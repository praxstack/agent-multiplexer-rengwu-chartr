---
title: Chartr development wiki
updated: 2026-10-19
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

[0.3.2 preparation](development/release-0.3.2.md) covers the Settings-only view
switcher, pane/tab styling and notification context changes. Its
[candidate checks](verification/runs/2026-10-19-release-0.3.2.md) record passing
local/platform gates and package inspection, explicit notification/AUR deferrals,
and the remaining judgment about broader native acceptance coverage.

- [Shared setup](verification/setup.md): exact build identity, instance isolation,
  fixtures, available tests and cleanup.
- [Release acceptance](verification/acceptance.md) and
  [packaging runbook](development/releasing.md).

The [0.3.1 candidate follow-up](verification/runs/2026-10-16-release-candidate.md)
records passing package inspection and selected native attention checks.
The [desktop follow-up](verification/runs/2026-10-16-release-desktop.md) verifies
macOS Quit policies and a Linux terminal/Clock journey, and preserves the Linux
confirmation-clipping failure. The [recheck](verification/runs/2026-10-16-release-desktop-recheck.md)
tracks its correction and the latest automated gates.
[Release preparation](development/release-0.3.1.md) records the accepted readiness
scope. The [submitted answer](journal/2026-10-16-grill-37e45122-ab5c-4350-82ff-7a6c037454a4.md)
accepts focused verification and defers the remaining matrix for 0.3.1;
the [checkpoint](journal/2026-10-16-release-preparation.md) records that outcome.
AUR publication and the manual notification-click check remain deferred.
[Chartr v0.3.1](https://github.com/rengwu/chartr/releases/tag/v0.3.1) is published
as the latest stable release. The [publication run](verification/runs/2026-10-16-release-publication.md)
records passing tag builds, asset/checksum inspection, public downloads and the
skipped AUR publication.

The [initial October 16 assessment](verification/runs/2026-10-16-release-readiness.md)
remains historical **Failed** evidence. The
[fade expectation correction](verification/runs/2026-10-16-fade-expectations.md)
preserves the user's manual rendering values and passes all 428 local workspace tests.

The migration did not build or exercise Chartr. Later execution is recorded in
dated runs, including the [macOS frame-delivery recheck](verification/runs/2026-10-11-metal-frame-delivery-recheck.md).
Unexecuted recipes remain unvalidated, and historical passes apply only to their
recorded snapshots.

## History and migration

- [Agent attention and notifications](discussions/agent-attention.md): the October 15
  [accepted simplification](discussions/agent-attention.md#accepted-simplification-october-15)
  is implemented using existing Herdr detection and one notification until the
  exact terminal is visited. The [Codex recheck](verification/runs/2026-10-15-lean-attention.md)
  records passing automated tests, real completion/approval/cancellation states,
  dots/focus, persistence, live handoff and minimized/closed-window monitoring.
  Native click navigation, direct banner inspection and broader platform/layout
  coverage remain unverified. [Setup attempts](verification/runs/2026-10-15-lean-attention-attempts.md)
  and the [October 14 strict implementation](verification/runs/2026-10-14-agent-attention.md)
  remain historical evidence.
  The October 19 [notification-context decision](decisions/index.md#dec-attention-006-space-context-and-supported-log-response-previews)
  adds space/agent titles and supported-log reply previews with generic fallback;
  native appearance for this change remains unverified.
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
