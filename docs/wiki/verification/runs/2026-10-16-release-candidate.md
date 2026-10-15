---
title: Version 0.3.1 candidate verification
section: Verification
updated: 2026-10-16
result: Untested
---
# Version 0.3.1 candidate verification

The automated CI and package-build gates pass for **0.3.1**. Full release
acceptance remains incomplete: native notification clicks and several desktop
interaction scenarios are unverified. No tag or release has been published.

## Scope and identity

Follow up the [initial readiness assessment](2026-10-16-release-readiness.md) after
the user confirmed the manual fade tuning and selected version 0.3.1. The
[expectation correction](2026-10-16-fade-expectations.md) changes tests and current
documentation, preserving production rendering.

Candidate: `e915365a68c9770149ba36c05bd7525fd7b91f70`. The normal local build,
GitHub CI and downloaded packages all identify this revision. The
[manifest](../../evidence/2026-10-16-release-candidate/manifest.json) records
source hashes, container environments and the bounded backend-test adaptation.
Only documentation/evidence and the AUR setup-error documentation link are
changed after the candidate; application and packaging behavior are unchanged.

Affected features: [distribution](../../features/distribution.md)
(`FEAT-distribution`), [workspace views](../../features/workspace-views.md)
(`FEAT-workspace-views`), [agent attention](../../features/agents-inbox.md)
(`FEAT-agents-inbox`), [session lifecycle](../../features/session-lifecycle.md)
(`FEAT-session-lifecycle`) and [terminals](../../features/terminals.md)
(`FEAT-terminals`).

## Automated and distribution results

| Scenario | Expected | Observed | Result | Evidence |
| --- | --- | --- | --- | --- |
| Candidate CI | macOS, Ubuntu and Arch/AUR jobs pass | All three jobs passed for the candidate. macOS workspace: 428 passed, seven ignored. Ubuntu workspace: 430 passed, nine ignored. Each platform subsequently passed all three real-session checks; Ubuntu also passed forwarded X11 input and GTK scaling/lifecycle checks | Passed | [CI run](https://github.com/rengwu/chartr/actions/runs/38818070198), [job receipt](../../evidence/2026-10-16-release-candidate/ci.json), [result excerpts](../../evidence/2026-10-16-release-candidate/ci-results-excerpt.log), [workspace counts](../../evidence/2026-10-16-release-candidate/ci-workspace-summary.json) |
| Candidate packages | All three shipping architectures build | macOS arm64 and Linux x86_64/aarch64 passed. Draft-release job correctly skipped on the branch push | Passed | [Package run](https://github.com/rengwu/chartr/actions/runs/38818070127), [job receipt](../../evidence/2026-10-16-release-candidate/packages-ci.json) |
| Normal local build | Build succeeds after test compilation | Locked normal build passed on macOS 27 arm64 | Passed | [Result and hash](../../evidence/2026-10-16-release-candidate/build-result.json), [log](../../evidence/2026-10-16-release-candidate/normal-build.log) |
| macOS DMG | Checksum, image, signature, metadata and sidecar agree | Verified downloaded image and deep strict ad-hoc signature; version 0.3.1, revision e915365a68c9, exact pinned Herdr | Passed | [Inspection](../../evidence/2026-10-16-release-candidate/macos-package-inspection.json) |
| Linux x86_64 artifacts | All checksums match | Tarball, Debian, Arch and AUR recipe checksums matched | Passed | [Checksums](../../evidence/2026-10-16-release-candidate/linux-x86_64-checksums.json) |
| Ubuntu x86_64 installation | Debian install and extracted launcher can start the shipped app and private backend | Both launch paths remained alive under Xvfb and handshook the pinned private Herdr | Passed | [Startup results](../../evidence/2026-10-16-release-candidate/ubuntu-x86_64-startup.json), [log](../../evidence/2026-10-16-release-candidate/linux-x86_64-package-check.log) |
| Arch x86_64 installation | Arch package install and extracted launcher can start the shipped app and private backend | Both launch paths remained alive under Xvfb and handshook the pinned private Herdr | Passed | [Startup results](../../evidence/2026-10-16-release-candidate/arch-startup.json), [log](../../evidence/2026-10-16-release-candidate/arch-package-check.log) |
| Ubuntu ARM64 installation and real backend | Debian install, extracted launcher and the shipped Herdr pass runtime checks | Both launch paths started under Xvfb and handshook the exact sidecar. All three existing real-session tests passed against the installed release sidecar: attach identity, crash recovery and mouse/TUI behavior | Passed | [Checksums](../../evidence/2026-10-16-release-candidate/linux-aarch64-checksums.json), [startup results](../../evidence/2026-10-16-release-candidate/ubuntu-arm64-startup.json), [log](../../evidence/2026-10-16-release-candidate/linux-arm64-package-check.log) |
| Cross-format binary identity | Each architecture ships identical executable bytes across formats | Chartr and Herdr hashes match between tarball and Debian on both Linux architectures, and between those formats and Arch on x86_64 | Passed | [Binary hashes](../../evidence/2026-10-16-release-candidate/linux-binary-identity.json) |

The x86_64 container checks run through OrbStack emulation on the arm64 host;
they are startup/backend checks, not interactive desktop or rendering assertions.
The first Arch fixture setup hit a pacman downloader seccomp error before
installing Chartr. Its [setup log](../../evidence/2026-10-16-release-candidate/arch-emulation-setup-attempt.log)
is retained. Disabling that downloader sandbox only inside the disposable
container allowed setup and the subsequent checks to pass; host/container
isolation was retained. Executed [Ubuntu script](../../evidence/2026-10-16-release-candidate/ubuntu-x86_64-check.sh.txt),
[Arch script](../../evidence/2026-10-16-release-candidate/arch-check.sh.txt) and
[startup probe](../../evidence/2026-10-16-release-candidate/linux-startup.py.txt)
make the selected checks reproducible.

The ARM64 backend harness avoids rebuilding GPUI. It copies the existing test
and backend crates, changing only the test's sidecar path to the installed
release executable. An initial harness compile omitted the test's direct
`serde_json` dependency; that [setup attempt](../../evidence/2026-10-16-release-candidate/arm64-harness-setup-attempt.log)
is retained and is not a Chartr test failure. After correction, all three tests
and both startup paths passed. The executed [runner](../../evidence/2026-10-16-release-candidate/ubuntu-arm64-check.sh.txt),
[manifest](../../evidence/2026-10-16-release-candidate/backend-harness-Cargo.toml.txt),
[lockfile](../../evidence/2026-10-16-release-candidate/backend-harness-Cargo.lock.txt)
and [test adaptation](../../evidence/2026-10-16-release-candidate/backend-harness-live_session.rs.txt)
identify the harness precisely.

## Native attention observations

The uniquely registered **Chartr Release Verify** app uses a private
`/tmp/cr031-mchot24z` profile and the signed normal-build copy recorded in its
[instance manifest](../../evidence/2026-10-16-release-candidate/native-instance.json).
The user's personal Chartr app and earlier fade preview were not driven.
Codex 0.162.1 executed harmless sleep-and-reply prompts in the disposable project.

| Scenario | Expected | Observed | Result | Evidence |
| --- | --- | --- | --- | --- |
| Adjacent split focus | Completing agent remains unread while its neighbor is focused | Private state and AX labels showed unread attention on Codex and its outer two-tab group; screenshot showed both dots | Passed | [State](../../evidence/2026-10-16-release-candidate/adjacent-split-unread.json), [observations](../../evidence/2026-10-16-release-candidate/native-observations.json) |
| Exact terminal focus and rearming | Visiting the agent clears its dot and permits a later notice | Selecting Codex cleared persisted unread at revision 1; later completion notified at revision 2 | Passed | [Observations](../../evidence/2026-10-16-release-candidate/native-observations.json), [notification logs](../../evidence/2026-10-16-release-candidate/notifications.log) |
| Minimized window | Monitoring continues without reopening the window | Completion produced unread revision 2 while only Finder was inspected | Passed | [State](../../evidence/2026-10-16-release-candidate/minimized-unread.json) |
| Closed window and same-process reopen | Monitoring continues and reopening restores the split and unread state | Completion produced unread revision 3; opening the exact bundle through Finder restored its split and unread state | Passed | [State](../../evidence/2026-10-16-release-candidate/closed-window-unread.json), [observations](../../evidence/2026-10-16-release-candidate/native-observations.json) |
| Visible alert and native click navigation | Inspect the actual alert; clicking reopens and focuses Codex | Notification permission was allowed and macOS logged delivery/presentation. Automation timed out selecting the system alert, including after making this disposable app's alerts persistent. A manual click observation was requested | Untested | [Logs](../../evidence/2026-10-16-release-candidate/notifications.log), [pending alert state](../../evidence/2026-10-16-release-candidate/persistent-alert-unread.json) |
| Hide, graceful Quit, stale native click | Hidden-app monitoring, graceful persistence and rejection of stale notice navigation work | Cmd+H did not hide this app; the native app menu was empty. Graceful Quit and stale-click scenarios were not established | Untested | [Observations](../../evidence/2026-10-16-release-candidate/native-observations.json) |

Some early native screenshots lagged the AX tree/backend state until a window
zoom. Later screenshots showed the correct split and attention dots. This
capture discrepancy does not establish a new product rendering regression.
Delivery logs prove a system delivery attempt, not an observed banner or click.

## Release limits

The [full acceptance matrix](../acceptance.md) is not complete. Outstanding
coverage includes notification clicks/stale targets, Hide/graceful Quit,
interactive Linux desktop terminal/web-pane journeys, and the full visual,
input/accessibility and plugin-installation matrix. Container startup does not
replace those desktop checks. The production-ID macOS DMG was inspected rather
than launched alongside the user's running personal app.

AUR publishing still requires repository secret `AUR_SSH_PRIVATE_KEY` and
variable `AUR_SSH_KNOWN_HOSTS`; a fresh
[names-only check](../../evidence/2026-10-16-release-candidate/aur-configuration.json)
confirms both remain absent.
This does not prevent GitHub artifacts, but stable-publication automation for
AUR will fail until configured. Do not bypass that check or claim publication.

The [0.3.1 preparation page](../../development/release-0.3.1.md) contains draft
release notes and links to the candidate gates. No version tag or release was
created during this verification.

## Cleanup and manual handoff

All three disposable containers have exited and been removed; the DMG is
detached. Downloads and build caches remain under `target/release-candidate-0.3.1`.
The [cleanup record](../../evidence/2026-10-16-release-candidate/cleanup.json)
identifies the test resources and retained native fixture.

The isolated Chartr Release Verify process and its two disposable sessions remain
available for the requested manual notification click. Its window was closed
after completion and unread revision 4 remains pending. Only this newly created
bundle's alert style was changed to Persistent for inspection; restore Temporary
and clean up the private profile/backend after that observation. No global
notification settings, personal Chartr sessions or earlier preview were changed.
