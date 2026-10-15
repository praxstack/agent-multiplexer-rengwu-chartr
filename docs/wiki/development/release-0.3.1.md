---
title: Release 0.3.1 preparation
section: Development
updated: 2026-10-16
status: Automated gates passed; desktop acceptance incomplete
---
# Release 0.3.1 preparation

The user selected **0.3.1** on October 16. Version preparation is committed at
`e915365a68c9770149ba36c05bd7525fd7b91f70`; no tag or published release has been
created.

The [initial readiness assessment](../verification/runs/2026-10-16-release-readiness.md)
records the original gaps. The [fade expectation correction](../verification/runs/2026-10-16-fade-expectations.md)
preserves the user's manual rendering values and passes all 428 exercised tests.

## Candidate gates

- [CI](https://github.com/rengwu/chartr/actions/runs/38818070198): passed for the
  candidate, including macOS, Ubuntu and Arch/AUR checks.
- [Release packages](https://github.com/rengwu/chartr/actions/runs/38818070127):
  passed for Apple silicon macOS and Linux x86_64/ARM64.
- [Candidate verification](../verification/runs/2026-10-16-release-candidate.md)
  records passing package inspection and selected native checks. Native alert
  clicks and full desktop acceptance remain incomplete.
- AUR publication is unavailable until `AUR_SSH_PRIVATE_KEY` and
  `AUR_SSH_KNOWN_HOSTS` are configured. This is independent of building the
  GitHub packages and tested AUR recipe. The stable-publication AUR job will fail
  its explicit configuration check until setup is completed.

## Draft release notes

### Changes

- Added persistent agent attention dots and desktop notifications. Existing Herdr
  detection provides best-effort alerts when an agent may need attention; visiting
  the exact terminal clears its dot and allows its next notification. Agent
  identity and unread state survive backend handoff and app relaunch.
- Refined Tabs and Spaces navigation, space headings, subtle space cards, row
  spacing, drag previews and fading scrollbars. Space cards now scroll beneath
  the fixed Spaces header and its overlaid fade.
- Fixed reopening the running macOS app after its last window closes.
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

Publish only after the remaining [acceptance](../verification/acceptance.md)
gates are resolved and the [packaging runbook](releasing.md) is followed.
