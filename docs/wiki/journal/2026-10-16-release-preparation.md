---
title: Version 0.3.1 release preparation checkpoint
section: Journal
updated: 2026-10-16
status: Published after accepted verification scope
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

Before the release-scope answer, the [draft notes and remaining scope](../development/release-0.3.1.md)
were ready for review. Physical input/GPU/accessibility, full visual/plugin
coverage, actual hidden-state monitoring and additional desktop journeys remain
incomplete under the existing acceptance matrix. Whether to defer those checks
for this patch release was the outstanding user choice. No tag,
draft release or publication has been created.

Disposable desktop fixtures are stopped; the unique macOS notification setting
is restored. Personal Chartr instances and unrelated browser windows remain
untouched.

## Accepted release-scope answer

The user selected **Use focused verification; defer the remaining matrix for
0.3.1** in the [submitted form](2026-10-16-grill-37e45122-ab5c-4350-82ff-7a6c037454a4.md).
[DEC-RELEASE-031-002](../decisions/index.md#dec-release-031-002-focused-verification-is-sufficient-for-0-3-1)
records the interpretation: the readiness scope is satisfied, while deferred
coverage remains explicitly unverified follow-up. Earlier report outcomes are
unchanged, and future releases retain the baseline matrix.

The records-only checkpoint `99969893` also passed both CI and package workflows.
This answer requires documentation updates and wiki validation, not another
product-test cycle. Tagging and publication were not part of the readiness
question and have not been performed.

## Publication after explicit request

The user subsequently requested, “lets do a release.” Annotated tag `v0.3.1`
was pushed at `eebe2b22767d6565e86f3e12bb6185c71999533a`, whose CI and branch
package builds had passed. The tag workflow passed on all three platforms and
created the draft. All seven payloads and three checksum files were inspected,
the prepared notes were applied, and
[v0.3.1](https://github.com/rengwu/chartr/releases/tag/v0.3.1) was published as
latest stable at 12:42:55 UTC on October 16.

The [publication run](../verification/runs/2026-10-16-release-publication.md)
records exact asset digests, package metadata, public download checks and the
correctly skipped AUR workflow. Existing verification deferrals and historical
run outcomes are unchanged. The macOS image was detached after inspection;
personal app instances were not used.
