---
title: Last-window reopen baseline reproduction
section: Verification
updated: 2026-10-12
result: Failed
---
# Last-window reopen baseline reproduction

## Scope

Reproduce the [FEAT-session-lifecycle](../../features/session-lifecycle.md)
last-window reopen failure before testing the October 12 fix. This result belongs
to the unchanged main build; the [fixed-build run](2026-10-12-window-reopen.md)
is separate. The earlier [October 11 failure](2026-10-11-metal-frame-delivery-recheck.md)
remains unchanged.

## Build and environment

Commit `4cfe1581cca6ec53cef79691e5add0fdf1829e37`, locked development build on
arm64 macOS with Rust 1.96.0 and debug information disabled. Signed app executable
SHA-256 `1387b22f6f8dd9fd84d517b6b7efeff4b83dadc5fac0833a5918165729144f3c`.
[Manifest](../../evidence/2026-10-12-window-reopen/native-build-manifest.json),
[build output](../../evidence/2026-10-12-window-reopen/baseline-build.log).

Isolated bundle `dev.chartr.reopen-verification.baseline`, profile
`/tmp/chartr-reopen.xf24t27z/baseline/profile`, GUI PID 23941, backend PID 23909.
Disposable terminal `term_65da386d8f7681`. The user explicitly
[approved the native AppKit driver](../../journal/2026-10-12-grill-e4ec3c4a-3dec-4cfc-a0f5-5c9f91924f20.md)
after the regular computer tool failed to start. The driver calls native window
operations inside this process; LaunchServices reopening uses `open -a` on the
exact isolated bundle. It does not directly invoke the GPUI reopen callback.

## Results

| Scenario | Expected outcome | Observed outcome | Result | Evidence |
| --- | --- | --- | --- | --- |
| Startup and terminal readiness | One visible workspace and usable disposable shell | PID 23941 opened window 11362; typing a shell command through native window events wrote `before-close-ok` into the fixture project | Passed | [Observations](../../evidence/2026-10-12-window-reopen/baseline-observations.json) |
| Close last window and reopen running bundle | Recreate a visible workspace in the same process | Native close removed the GPUI window. LaunchServices open left PID 23941 alive with zero GPUI windows | Failed | Observations above; [summary](../../evidence/2026-10-12-window-reopen/native-summary.json) |
| Default quit policy | Shell survives normal app quit | The same private terminal remained after native app termination | Passed | Observations above |

Overall **Failed** because the exercised reopen expectation failed.

## Limits and follow-up

The first harness attempt launched PID 23552/backend 23520, but both were gone
before a subsequent shell command could use the driver; that follow-up timed out.
The completed attempt kept startup, interaction and cleanup in one runner
lifetime. The earlier attempt is retained in the observations and is not used to
infer a product failure. The completed failure is the native close/reopen result
above, matching the independent October 11 observation.

The app entry point lacks a reopen callback. The selected correction and its
fresh native checks are documented in the fixed-build run. This baseline run
makes no assertion about renderer performance or general release readiness.

## Cleanup

The baseline GUI was quit and only its private backend was stopped. The final
[cleanup evidence](../../evidence/2026-10-12-window-reopen/native-summary.json)
found no remaining owned process; installed app PID 1121/backend PID 1123 and the
installed executable hash remained unchanged.
