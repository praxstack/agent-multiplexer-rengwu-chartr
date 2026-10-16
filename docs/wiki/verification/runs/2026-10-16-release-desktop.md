---
title: Version 0.3.1 desktop follow-up
section: Verification
updated: 2026-10-16
result: Failed
---
# Version 0.3.1 desktop follow-up

This run found clipped permission text in the Linux installation confirmation.
The failure remains historical evidence; the [recheck](2026-10-16-release-desktop-recheck.md)
records the correction. Selected macOS Quit and Linux terminal/plugin checks
passed. This is not a full release-acceptance pass.

## Scope and identity

Continue the [candidate checks](2026-10-16-release-candidate.md) for
[session lifecycle](../../features/session-lifecycle.md) (`FEAT-session-lifecycle`),
[plugin packages](../../features/plugin-packages.md) (`FEAT-plugin-packages`),
and [distribution](../../features/distribution.md) (`FEAT-distribution`).

The macOS fixture is **Chartr Release Verify**, bundle
`dev.chartr.releaseverify.oct16`, private profile `/tmp/cr031-mchot24z`, on macOS
27 arm64. Its normal debug build used `08e6bd7953d02115a063a8d86ead370e9ce650ba`
plus the application-menu change subsequently committed as
`b807e4c3ef83bdf8725d5505444e5dd9fedde8bc`. The
[instance manifest](../../evidence/2026-10-16-release-desktop/instance.json)
records executable hashes, source hashes and the dirty patch fingerprint.
The [baseline](../../evidence/2026-10-16-release-desktop/baseline.json) distinguishes
stopping the old fixture to replace its binary from the later actual Quit checks.

Linux used the downloaded `e915365a68c9770149ba36c05bd7525fd7b91f70` ARM64 Debian
package in an isolated Ubuntu 24.04 container with Openbox, Xvfb, software Vulkan,
GTK portal, and the repository Clock fixture. UI actions went through noVNC in a
dedicated Safari window. See [identity and actions](../../evidence/2026-10-16-release-desktop/linux-baseline.json).
This is a virtual desktop check, not physical GPU, Wayland or Arch desktop coverage.

## Results

| Scenario | Expected | Observed | Result | Evidence |
| --- | --- | --- | --- | --- |
| macOS application menu | Hide and Quit actions are registered | The former empty menu now exposes Hide chartr, Hide Others, Show All and Quit chartr, with application-wide shortcuts | Passed | [Source at tested change](https://github.com/rengwu/chartr/blob/b807e4c3ef83bdf8725d5505444e5dd9fedde8bc/crates/chartr/src/application_menu.rs), [instance](../../evidence/2026-10-16-release-desktop/instance.json) |
| Normal Cmd+Q | Exit while preserving detached sessions and unread state | PID 41119 exited; both backend terminal identities and unread revision 4 survived. Relaunch restored both tabs and unread labels | Passed | [After Quit](../../evidence/2026-10-16-release-desktop/after-cmd-q.json), [restored AX tree](../../evidence/2026-10-16-release-desktop/macos-restored-ax.txt) |
| Cmd+Q from Settings with terminate policy | Exit policy applies to the current workspace | Enabling the switch persisted the setting; Cmd+Q exited PID 42641 and the private backend reported no panes | Passed | [Termination result](../../evidence/2026-10-16-release-desktop/terminate-on-quit.json) |
| Hidden-app monitoring | Hide the app and observe background attention | Shortcut was exercised but actual native hidden state was not established by the available capture; not inferred from menu registration | Untested | [Baseline and limits](../../evidence/2026-10-16-release-desktop/baseline.json) |
| Linux terminal input | UI typing reaches the real shell | Creating a terminal and typing a printf command displayed `CHARTR_031_LINUX_INPUT_OK` | Passed | [Terminal screenshot](../../evidence/2026-10-16-release-desktop/ubuntu-terminal.png) |
| Linux installation disclosure | Entire source and declared authority are readable before confirmation | GPUI's default 288px fallback prompt clipped the source/host-access detail at the right edge | Failed | [Clipped prompt](../../evidence/2026-10-16-release-desktop/linux-permission-prompt.png) |
| Clock install, Later and activation | Managed installation is staged, restart-required state persists and Clock becomes available | Folder installation succeeded; Later retained the restart banner; after restart Clock was available and rendered | Passed | [Restart banner](../../evidence/2026-10-16-release-desktop/linux-restart-banner.png), [install receipt](../../evidence/2026-10-16-release-desktop/linux-clock-install.json), [Clock](../../evidence/2026-10-16-release-desktop/linux-clock.png) |
| Clock setting update | Saved format changes reach the live web view | Switching to 12-hour saved the setting and the live clock displayed AM | Passed | [Settings](../../evidence/2026-10-16-release-desktop/linux-clock-settings.png), [saved data](../../evidence/2026-10-16-release-desktop/linux-clock-settings.json), [updated view](../../evidence/2026-10-16-release-desktop/linux-clock-updated.png) |
| Linux terminal restoration after restart | Original terminal remains usable after plugin restart | Two processes were observed: one launched by GPUI's restart helper and one by the original interactive shell. The selected window had a blank terminal. The fixture was stopped before a clean single-process recheck; no restoration pass is claimed | Untested | [Process evidence](../../evidence/2026-10-16-release-desktop/linux-baseline-processes.txt), [desktop log](../../evidence/2026-10-16-release-desktop/linux-desktop.log) |

A clean, explicitly single-process relaunch restored the original terminal and
its output; see [restored fixture](../../evidence/2026-10-16-release-desktop/linux-single-process-restored.png).
The visible shell history also showed a second queued launch command. The
original restart attempt remains inconclusive; the recheck repeats it without
that competing process.

The first Linux folder-picker attempt had no usable session D-Bus/portal in the
minimal container. Installing and starting those services, updating their display
environment and relaunching Chartr resolved that setup issue. A subsequent wrong
folder selection correctly reported the missing manifest. Neither is classified
as a product regression.

## Correction and limits

The application menu uses GPUI's ordinary quit path, preserving persistence and
exit-policy handling. Its local workspace suite passed 428 tests, with seven
ignored, before the Linux prompt change; see the [log](../../evidence/2026-10-16-release-desktop/workspace-tests.log).
Both [CI](https://github.com/rengwu/chartr/actions/runs/38867158380) and all
[release packages](https://github.com/rengwu/chartr/actions/runs/38867158387) passed
for `b807e4c3`.

The Linux prompt correction uses Chartr's existing dialog surface, wraps message
and detail text within the available width, scrolls long content, and keeps the
actions outside that scrolling body. macOS keeps its native prompt.

The user explicitly deferred AUR publication and the requested manual notification
click. Neither is represented as verified. Full physical-input, GPU, visual,
accessibility, plugin-variant and Arch desktop acceptance remain incomplete.
The [release preparation page](../../development/release-0.3.1.md) owns current
release status; no tag or publication occurred during this run.

## Cleanup

The unique macOS fixture's two sessions were terminated by the verified exit
policy. Its alert style was restored to Temporary through System Settings;
see [AX confirmation](../../evidence/2026-10-16-release-desktop/notification-cleanup.txt)
and [cleanup record](../../evidence/2026-10-16-release-desktop/macos-cleanup.json).
The user's normal Chartr and prior fade-preview instances were not driven.
Linux fixture cleanup is completed and recorded by the recheck.
