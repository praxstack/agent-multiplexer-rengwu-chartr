---
title: Version 0.3.1 desktop correction recheck
section: Verification
updated: 2026-10-16
result: Passed
---
# Version 0.3.1 desktop correction recheck

The selected correction checks pass: Linux confirmation text is readable,
plugin replacement and restart preserve the terminal and Clock setting, and
all current CI and package jobs pass. This bounded result does not complete the
[full release acceptance matrix](../acceptance.md). The
[earlier failure](2026-10-16-release-desktop.md) is preserved.

## Scope and identity

Affected features: [plugin packages](../../features/plugin-packages.md)
(`FEAT-plugin-packages`), [session lifecycle](../../features/session-lifecycle.md)
(`FEAT-session-lifecycle`), and [distribution](../../features/distribution.md)
(`FEAT-distribution`). Verify the observed Linux prompt-clipping correction,
repeat replacement/restart with one isolated process, and check the user's
explicit AUR publication deferral.

Product fix: `2707f875`, followed by the AUR workflow change in candidate
`78d651e6e12a1d2e3dd58fde1496c978747692fd`. The local suite ran after the prompt
change with only documentation and the pending AUR workflow edit outside product
source. The downloaded ARM64 Debian package identifies the latter candidate's
[package workflow](https://github.com/rengwu/chartr/actions/runs/38868786289).

The [Linux environment manifest](../../evidence/2026-10-16-release-desktop/linux-environment.json)
records the isolated Ubuntu 24.04 ARM64 container, image identity, read-only
fixture/package mounts and loopback noVNC port. It uses Xvfb/Openbox, software
Vulkan, a user D-Bus and GTK portal. A single explicit process was launched after
upgrading the package. [Runtime identity](../../evidence/2026-10-16-release-desktop/linux-recheck.json)
records installed binary hashes, PIDs and managed Clock settings. No personal
profile or regular Chartr instance was used.

## Results

| Scenario | Expected | Observed | Result | Evidence |
| --- | --- | --- | --- | --- |
| Long prompt regression | Narrow windows wrap long paths/details, scroll the body and retain reachable actions with correct response indices | GPUI regression at 360×280 passed on macOS and Ubuntu; Cancel returned index 1 and removed the prompt; Escape also cancelled instead of confirming removal | Passed | [Local test](../../evidence/2026-10-16-release-desktop/prompt-tests.log), [final workspace log](../../evidence/2026-10-16-release-desktop/prompt-workspace-tests.log), [CI excerpts](../../evidence/2026-10-16-release-desktop/ci-results-excerpt.log) |
| Local workspace suite | Existing tests remain green | 429 passed, zero failed, seven ignored; formatting and diff checks passed | Passed | [Counts](../../evidence/2026-10-16-release-desktop/local-tests.json), [log](../../evidence/2026-10-16-release-desktop/prompt-workspace-tests.log) |
| Candidate CI | macOS, Ubuntu and Arch/AUR jobs pass | All jobs passed, including real-session tests, Linux X11/GTK checks, native plugin contract and JS checks | Passed | [CI](https://github.com/rengwu/chartr/actions/runs/38868786358), [receipt](../../evidence/2026-10-16-release-desktop/ci.json), [excerpts](../../evidence/2026-10-16-release-desktop/ci-results-excerpt.log) |
| Candidate packages | macOS arm64 and both Linux architectures build | All three jobs passed; draft creation correctly skipped on branch push | Passed | [Packages](https://github.com/rengwu/chartr/actions/runs/38868786289), [receipt](../../evidence/2026-10-16-release-desktop/packages-ci.json) |
| Downloaded ARM64 package | Published workflow checksums match and Debian upgrade succeeds | Both tarball and Debian checksum matched; Debian installed over the previous disposable 0.3.1 package | Passed | [Checksums](../../evidence/2026-10-16-release-desktop/linux-recheck-checksums.json), [runtime hashes](../../evidence/2026-10-16-release-desktop/linux-recheck.json) |
| Permission disclosure | Full source, project-file authority and replacement behavior are readable | Actual packaged UI showed all detail text and both actions without clipping | Passed | [Corrected prompt](../../evidence/2026-10-16-release-desktop/linux-permission-prompt-fixed.png) |
| Replacement/restart | Installation succeeds and one process restores the terminal | Install then Restart changed PID 17185 to one process, 17585. Original terminal output returned; new UI-typed command printed `CHARTR_031_RESTART_INPUT_OK` | Passed | [Restored terminal](../../evidence/2026-10-16-release-desktop/linux-restart-restored.png), [new input](../../evidence/2026-10-16-release-desktop/linux-restart-input.png), [runtime receipt](../../evidence/2026-10-16-release-desktop/linux-recheck.json) |
| Plugin data survives replacement | Clock retains its saved 12-hour format and renders after activation | Restored Clock displayed AM; saved format remained `12` | Passed | [Clock after replacement](../../evidence/2026-10-16-release-desktop/linux-clock-after-replacement.png), [runtime receipt](../../evidence/2026-10-16-release-desktop/linux-recheck.json) |
| AUR deferral configuration | Stable releases do not automatically publish until explicitly enabled | YAML parsed; source condition requires `AUR_PUBLISH_ENABLED=true`, which is absent. Manual dispatch and credential validation remain intact | Passed | [Configuration inspection](../../evidence/2026-10-16-release-desktop/aur-deferral.json), [runbook](../../development/releasing.md#aur-publishing) |

The AUR row is a source/configuration check, not an executed publication event.
No release was published and no AUR account or credentials were created.

## Limits and cleanup

The earlier duplicate-process fixture is not counted as a passing restart test;
the clean single-process recheck above resolves that uncertainty for this build.
No new failure was observed in the selected recheck.

Native notification click/stale-target interaction, actual macOS hidden-state
monitoring, physical input/GPU/accessibility coverage, application-menu launching,
Arch interactive desktop journeys and the complete visual/plugin matrix remain
outside this pass. The [release preparation page](../../development/release-0.3.1.md#remaining-release-judgment)
keeps those remaining gates explicit. AUR and the requested manual click are the
only accepted deferrals so far.

The disposable Linux container is stopped and removed after preserving evidence;
see [cleanup](../../evidence/2026-10-16-release-desktop/linux-cleanup.json).
The macOS fixture is also stopped, with its unique alert style restored to
Temporary. Downloads/build caches remain under `target/`; personal app instances
and unrelated browser windows remain untouched. No tag or release was created.
