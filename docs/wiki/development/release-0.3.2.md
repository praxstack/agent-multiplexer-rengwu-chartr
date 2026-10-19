---
title: Release 0.3.2 preparation
section: Development
updated: 2026-10-19
status: Candidate prepared; release acceptance scope pending
---
# Release 0.3.2 preparation

The user requested preparation of **0.3.2** on October 19. The workspace version
and both Cargo lockfiles now agree. The preparation branch is `release/0.3.2`;
no release tag or GitHub publication has been created.

## Scope since 0.3.1

- Move Tabs/Spaces switching exclusively to Settings → General → View mode.
  Remove the title-header picker, its hide/show setting, and direct/cycle view
  commands and shortcuts. Stored legacy shortcut entries remain preserved but
  inactive and no longer reserve keys.
- Refine pane tabs: use the window background behind trailing controls, retain
  single-pixel shared dividers between stacked panes, soften inactive backgrounds
  across all built-in themes, and strengthen VSCode Dark Modern's borders.
- Add owning space and agent names to notification titles and supported-log reply
  previews, with a generic fallback. Keep existing attention and unread semantics.

The [verification run](../verification/runs/2026-10-19-release-0.3.2.md) records
candidate-specific local checks and platform/package evidence.

Candidate `4afea26f5bd2fc168675e0b58aa731324c1bea1f` is pushed. Its
[CI](https://github.com/rengwu/chartr/actions/runs/39215273814) and explicitly
dispatched [package builds](https://github.com/rengwu/chartr/actions/runs/39215274962)
passed on all shipping platforms. All seven downloaded payload checksums pass;
Linux metadata/architecture and tar/Debian/Arch binary consistency were verified.
The candidate macOS DMG matches the exact source revision, production bundle ID,
version and architectures; its signature and checksum pass. Both inspected macOS
volumes were detached without launching Chartr.

## Deferrals and remaining gates

- **Notification testing is explicitly deferred until the next release** at the
  user's request after the [shared-profile local bundle incident](../discussions/agent-attention.md#local-bundle-attempt-and-user-deferral).
  Native banners, previews and click navigation are unverified; no further
  notification test app is launched during preparation.
- **AUR publication remains deferred** while account registration is unavailable.
  Keep automatic publication disabled. Package/recipe tests still apply.
- The broader [release acceptance matrix](../verification/acceptance.md) remains
  applicable. The 0.3.1 focused-verification exception was release-specific;
  it is not silently extended to 0.3.2. Physical input/accessibility, the complete
  native visual/plugin matrix and installed Linux desktop journeys are not newly
  verified by local unit tests or package inspection.
- Candidate macOS/Ubuntu/Arch CI and macOS/Linux package builds pass for the
  prepared source. Final tagging and publication remain a separate release step.

**Remaining readiness choice:** accept the completed focused checks for 0.3.2
and retain the broader desktop matrix as unverified follow-up, or complete that
matrix before publication. Focused acceptance is a proposal, not yet an accepted
0.3.2 exception. Existing notification and AUR deferrals are already settled.

## Draft release notes

### Changes

- Simplified the window header: switch between Tabs and Spaces in
  **Settings → General → View mode**. The header picker and view-switching
  commands/shortcuts have been removed.
- Refined pane tab styling with window-matching backgrounds behind controls,
  single-line dividers between stacked panes, subtler inactive tabs across themes,
  and clearer VSCode Dark Modern borders.
- Added context to agent notifications: titles identify the owning space and
  agent; supported agent logs supply a shortened first-line reply preview.
  Unavailable previews retain “Agent may need your attention.”

### Downloads and known limits

- macOS: Apple silicon. The app is ad-hoc signed and **not notarized**.
- Linux: x86_64 and ARM64 tarballs and Debian packages, built on Ubuntu 24.04.
  X11/XWayland, a working Vulkan driver, GTK 3 and WebKitGTK 4.1 are required.
  Arch binary packaging is x86_64 only; AUR publication remains deferred.
- Agent alerts remain best effort. Response previews require a recognized native
  session and a supported, readable log; they do not certify successful completion.
- Native notification appearance/click testing for these changes remains deferred.
- External native plugin payloads are distributed separately.
