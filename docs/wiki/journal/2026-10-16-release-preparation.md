---
title: Version 0.3.1 release preparation checkpoint
section: Journal
updated: 2026-10-16
status: Release-scope decision pending
---
# Version 0.3.1 release preparation checkpoint

The user selected **0.3.1**, asked to preserve the manual fade tuning, delegated
remaining release work through `work-until-grill`, deferred the manual
notification-click check, and deferred AUR while account registration is
unavailable. [DEC-RELEASE-031-001](../decisions/index.md#dec-release-031-001-release-version-and-explicit-deferrals)
records these accepted choices. No broader acceptance waiver has been inferred.

The follow-up corrected the missing native macOS Hide/Quit menu in `b807e4c3`,
then fixed an observed Linux permission-dialog clipping bug in `2707f875`.
`78d651e6` makes automatic AUR publication opt-in without weakening explicit
manual publication checks. All changes are committed and pushed.

The [desktop run](../verification/runs/2026-10-16-release-desktop.md) preserves
the clipping failure and verifies both macOS Quit policies and a Linux Clock
journey. The [recheck](../verification/runs/2026-10-16-release-desktop-recheck.md)
passes the correction, package replacement/restart, terminal input and persisted
Clock data. Latest macOS/Ubuntu/Arch CI and all three package builds passed.
Local workspace tests: 429 passed, zero failed, seven ignored. Wiki structure
checks are recorded separately from product behavior.

The [draft release notes and remaining scope](../development/release-0.3.1.md)
are ready for review. Physical input/GPU/accessibility, full visual/plugin
coverage, actual hidden-state monitoring and additional desktop journeys remain
incomplete under the existing acceptance matrix. Deciding whether to defer those
checks for this patch release is the next consequential user choice. No tag,
draft release or publication has been created.

Disposable desktop fixtures are stopped; the unique macOS notification setting
is restored. Personal Chartr instances and unrelated browser windows remain
untouched. Continue from the release-scope answer rather than repeating completed
checks or asking again about AUR/the requested manual click.
