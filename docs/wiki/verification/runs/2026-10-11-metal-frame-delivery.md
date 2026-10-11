---
title: macOS supplied-drawable frame delivery
section: Verification
updated: 2026-10-11
result: Failed
---
# macOS supplied-drawable frame delivery

## Scope

This run covers [workspace view responsiveness](../../features/workspace-views.md)
and the native frame source selected by the
[authorized implementation](../../journal/2026-10-11-low-power-implementation.md).
The direct acceptance check is removal of main-thread `nextDrawable` waiting
from normal frame callbacks during real sidebar/view/terminal activity.
Native lifetime checks cover the new source's ownership and pause/drop behavior.

## Build and environment

- HEAD `45829e5f5e093bfa98f9a500ba887b6ff5c837ee`, with the rendering change and
  the five pre-existing, unchanged Rust edits. Product patch SHA-256
  `037675327c99b5d90a8985617c5dc5574c7d2338c15ba64c25ddbf78ef631e2f`.
  Untracked vendor sources are included by individual content hash in the
  [build manifest](../../evidence/2026-10-11-metal-frame-delivery/build-manifest.json).
- Rust 1.96.0, arm64 macOS 27.0 (26A5368g), MacBook Pro built-in display.
  Zed pin remains `1ea16c1ab9dd6d36649e002dc60995634da04daf` and Herdr remains
  the existing pin `5a2dee700eeeea68267a4d16777307632f77172f`.
- `cargo build --release -p chartr --locked -j 3` passed. The isolated signed
  executable SHA-256 is
  `4af62ded2f79f5692f25892345aa5f5671abf8fc76b9176961f4c45722eea1e3`.
  [Build log](../../evidence/2026-10-11-metal-frame-delivery/cargo-release-build.log),
  [tracked product patch](../../evidence/2026-10-11-metal-frame-delivery/build-product.patch.txt).
- Isolated bundle ID `dev.chartr.metal-verification`; GUI PID 47972, private
  backend PID 47998. Profile `/tmp/chartr-mtl.4b5cch8e` has fresh XDG config,
  state and data roots, 32 folder spaces and the normal ad hoc space. The full
  executable path and launch identity are in the manifest. No user terminal
  sessions are used. The bounded
  [terminal fixture](../../evidence/2026-10-11-metal-frame-delivery/frame-output.py.txt)
  produces a visible counter update every 20 ms when launched.
- The native module check ran on AC with effective Low Power Mode false, then
  on battery with it true. At release launch, `pmset` reported battery and the
  read-only [NSProcessInfo probe](../../evidence/2026-10-11-metal-frame-delivery/power-state.m.txt)
  returned true. No power policy was changed. A bounded display-idle assertion
  was added for desktop verification; it does not unlock the screen.

## Results

| Scenario | Expected outcome | Observed outcome | Result | Evidence |
| --- | --- | --- | --- | --- |
| Build and source checks | Patched pinned crates compile; no accidental source changes | Locked check and release build passed; workspace/new-source formatting and whitespace checks passed; source hashes match the release snapshot | Passed | [Check log](../../evidence/2026-10-11-metal-frame-delivery/cargo-check.log), build manifest and release log above |
| Rate policy | Low Power Mode caps at 60, respects slower displays, and normal mode restores default | Policy unit test passed for 120 Hz, 24 Hz, unknown maximum and normal mode | Passed | [Unit output](../../evidence/2026-10-11-metal-frame-delivery/policy-tests.log) |
| Native source on AC | Unused updates do not exhaust drawables; stop/resume and teardown work | 117 unused callbacks, 121 submissions after idle, 118 resize/resume callbacks; zero paused/post-drop callbacks; exactly one callback when it destroys its own source | Passed | [AC native run](../../evidence/2026-10-11-metal-frame-delivery/native-ac.log) |
| Native source on battery/Low Power Mode | Same lifecycle guarantees under the reproducing policy | Effective mode true; 60 unused callbacks, 61 submissions after idle, 60 resize/resume callbacks; zero paused/post-drop callbacks; callback-owned destruction passed | Passed | [Battery native run](../../evidence/2026-10-11-metal-frame-delivery/native-battery.log) |
| Real Chartr sidebar, views and terminal input | Real input reaches a visibly responsive release window | After unlocking: terminal printed CHARTR_INPUT_OK and a changing counter; sidebar scrolled and repeated Tabs/Spaces shortcuts changed the selected view. A later activation crashed (separate row below). | Passed | [Initial sample counts](../../evidence/2026-10-11-metal-frame-delivery/initial-samples.json) |
| Active Chartr main-thread capture | No acquisition beneath normal Metal callbacks in three active 30-second captures | First two complete captures: 0/4189 and 0/4141 nextDrawable samples, with 937 and 909 samples in the Metal callback. The third capture ended after only 225 main samples when the process crashed; it is excluded from performance acceptance. | Untested | [Capture 1](../../evidence/2026-10-11-metal-frame-delivery/active-1.sample.txt), [capture 2](../../evidence/2026-10-11-metal-frame-delivery/active-2.sample.txt), [interrupted capture](../../evidence/2026-10-11-metal-frame-delivery/active-3.sample.txt) |
| Real window lifecycle | Activation, resize, popup, hide/show and close/reopen remain functional | Activation reached the preserved synchronous transaction path and aborted. Pausing the Metal link does not permit nextDrawable; Apple throws CAMetalLayerInvalidOperation. | Failed | [Crash stack](../../evidence/2026-10-11-metal-frame-delivery/first-crash-excerpt.json), [stderr](../../evidence/2026-10-11-metal-frame-delivery/crash-stderr.log), [API reproduction](../../evidence/2026-10-11-metal-frame-delivery/transaction-api.log) |

The initial checkpoint was Untested while the desktop was locked. After the
user unlocked it, the first implementation failed an activation scenario. The
overall result is now **Failed**. The UI tool subsequently relaunched the same
isolated bundle as PID 51665, which was detected from the process identity and
excluded from the original PID's sampling claim.

A small [Objective-C reproduction](../../evidence/2026-10-11-metal-frame-delivery/transaction-probe.m.txt)
confirmed that `nextDrawable` throws even with the link paused, and succeeds after
invalidation. The correction invalidates/removes the source specifically for
AppKit transaction draws and recreates it afterward; ordinary visibility stops
still pause. This run remains the failed history, not a pass for that correction.

## Limits and follow-up

The first launch used macOS's long default temporary root and exceeded the Unix
socket path limit. A short `/tmp` root fixed this setup error. A copied snapshot
then duplicated seeded spaces; it was discarded before verification and a fresh
33-space profile was created. The abandoned GUI processes and the successfully
started abandoned backend were stopped. These were fixture issues, not observed
rendering regressions.

The corrected release build passed. Repeat the native check, now including eight
transaction/recreation cycles, and all real-app scenarios on its new binary
identity. A first native recheck lost drawable progress while the desktop had
auto-locked again ([output](../../evidence/2026-10-11-metal-frame-delivery/native-recheck-locked.log));
its result cannot qualify the corrected build. An unlocked, awake desktop is
required. The scratch pointer is `/tmp/chartr-metal-implementation-current.txt`.

End-to-end native-event-to-presentation p95 latency, sleep/wake, external-display
movement, Intel and macOS 11–13 runtime compatibility require separate coverage;
this run does not certify them. The fallback remains source-preserved, not
runtime-verified on an older OS. No independent reviewer was used.

The installed app (PID 1121) and private backend (PID 1123) remain untouched.
Its executable still has SHA-256
`699cbb789ad845e00e94bc2be69aa3ae9b6a2959a93ff554dedb24cd062cfcbe`.
The first GUI crashed; its relaunched replacement was stopped for the corrected
build. The isolated profile/backend are retained for the recheck; stop only
those run-owned processes after saving the final evidence. Use the exact private
namespace with `herdr server stop`; this server ignores ordinary SIGTERM.
