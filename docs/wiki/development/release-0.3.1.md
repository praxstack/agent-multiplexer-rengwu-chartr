---
title: Release 0.3.1 preparation
section: Development
updated: 2026-10-16
status: Ready under accepted verification scope
---
# Release 0.3.1 preparation

The user selected **0.3.1** on October 16. Version preparation is committed at
`e915365a68c9770149ba36c05bd7525fd7b91f70`; no tag or published release has been
created.

**Readiness is settled.** In the [release-scope answer](../journal/2026-10-16-grill-37e45122-ab5c-4350-82ff-7a6c037454a4.md),
the user accepted the completed focused verification and deferred the remaining
matrix for 0.3.1. [DEC-RELEASE-031-002](../decisions/index.md#dec-release-031-002-focused-verification-is-sufficient-for-0-3-1)
records this release-specific decision. Deferred checks remain unverified.

The [initial readiness assessment](../verification/runs/2026-10-16-release-readiness.md)
records the original gaps. The [fade expectation correction](../verification/runs/2026-10-16-fade-expectations.md)
preserves the user's manual rendering values and passes all 428 exercised tests.

## Candidate gates

Latest source candidate: `78d651e6e12a1d2e3dd58fde1496c978747692fd`.
Its [CI](https://github.com/rengwu/chartr/actions/runs/38868786358) and
[all package builds](https://github.com/rengwu/chartr/actions/runs/38868786289)
passed. The [correction recheck](../verification/runs/2026-10-16-release-desktop-recheck.md)
passes packaged Linux permission disclosure, replacement, restart, terminal
input and Clock-setting persistence. Earlier candidate evidence remains below.

The subsequent records-only revision `999698930603346ddf8dbb07b5765a98a8b2ec6f`
also passed [CI](https://github.com/rengwu/chartr/actions/runs/38870507068) and
[all package builds](https://github.com/rengwu/chartr/actions/runs/38870507012).
The readiness-decision update changes documentation only.

- [CI](https://github.com/rengwu/chartr/actions/runs/38818070198): passed for the
  candidate, including macOS, Ubuntu and Arch/AUR checks.
- [Release packages](https://github.com/rengwu/chartr/actions/runs/38818070127):
  passed for Apple silicon macOS and Linux x86_64/ARM64.
- [Candidate verification](../verification/runs/2026-10-16-release-candidate.md)
  records passing package inspection and selected native checks. The
  [desktop follow-up](../verification/runs/2026-10-16-release-desktop.md) adds
  macOS Quit-policy and Linux terminal/Clock checks and preserves the discovered
  Linux prompt-clipping failure. Its
  [recheck](../verification/runs/2026-10-16-release-desktop-recheck.md) records the fix.
  Full desktop acceptance remains incomplete.
- The user deferred AUR publication on October 16 because account registration
  is unavailable. Automatic publication requires `AUR_PUBLISH_ENABLED=true`;
  leave it unset until account access and both SSH settings are ready. GitHub
  release packages and the tested AUR recipe remain available independently.
- The user also deferred the requested manual notification-click check. It
  remains unverified. The later release-scope answer separately defers the
  remaining desktop matrix for 0.3.1.

## Remaining release judgment

Resolved October 16: the user chose **Use focused verification; defer the
remaining matrix for 0.3.1**. The [acceptance matrix](../verification/acceptance.md)
remains the broader baseline. The following coverage is retained as follow-up,
not a blocker for this release and not a passing verification claim:

| Area | Evidence and remaining limit |
| --- | --- |
| macOS lifecycle | Reopen, persistence and both Cmd+Q exit policies exercised; actual Hide/background monitoring remains unverified |
| Linux desktop | Ubuntu ARM64 virtual desktop exercises a terminal and Clock; physical GPU, application-menu launch and Arch interactive terminal/web journeys remain incomplete |
| Visual and input matrix | Existing automated layout/input checks pass; the complete size/theme/drag/split matrix, physical trackpad/IME and screen-reader checks have not been completed |
| Plugins | Local Clock install/settings journey exercised; full Git/network error, native embedded and cross-plugin journeys remain incomplete |
| Notifications | Delivery and unread-state checks exist; native banner/click/stale-target interaction is not certified; the requested manual click is explicitly deferred |

This decision applies to 0.3.1; it does not waive these checks for future releases
or change historical Failed/Untested outcomes. No further readiness-scope question
remains open.

## Draft release notes

### Changes

- Added persistent agent attention dots and desktop notifications. Existing Herdr
  detection provides best-effort alerts when an agent may need attention; visiting
  the exact terminal clears its dot and allows its next notification. Agent
  identity and unread state survive backend handoff and app relaunch.
- Refined Tabs and Spaces navigation, space headings, subtle space cards, row
  spacing, drag previews and fading scrollbars. Space cards now scroll beneath
  the fixed Spaces header and its overlaid fade.
- Fixed reopening the running macOS app after its last window closes, and added
  application-wide macOS Hide and Quit menu actions and shortcuts.
- Fixed clipped Linux confirmation details, including plugin permission disclosures.
- Improved macOS frame delivery to avoid the main-thread drawable wait observed
  during Low Power Mode use.
- Added Close All Sessions, space labels beneath chat titles, and Wayfinder
  starfield improvements that respect reduced motion.
- Kept status-bar settings language independent of particular plugins.

### Downloads and known limits

- macOS: Apple silicon. The app is ad-hoc signed and **not notarized**.
- Linux: x86_64 and ARM64 tarballs and Debian packages, built on Ubuntu 24.04.
  X11/XWayland, a working Vulkan driver, GTK 3 and WebKitGTK 4.1 are required.
  Arch binary packaging is x86_64 only.
- Agent alerts are best effort: cancellation can produce an alert, and unfamiliar
  agent screens can cause missed or false alerts. An alert does not certify that
  an agent completed successfully.
- The release contains no external native plugin payloads. Plugins are installed
  from their own distributions.

The accepted readiness scope is satisfied. Creating the version tag, verifying
the resulting draft assets and publishing remain separate steps in the
[packaging runbook](releasing.md); none has been performed by this readiness check.
