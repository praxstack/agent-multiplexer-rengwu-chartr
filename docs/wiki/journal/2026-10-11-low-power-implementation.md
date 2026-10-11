---
title: Supplied-drawable rendering implementation
section: Journal
updated: 2026-10-11
status: Implemented; rendering checks passed, aggregate lifecycle failure recorded
---
# Supplied-drawable rendering implementation

The user authorized implementation with `work-until-grill`, superseding the
investigation-only scope of the [October 10 design](2026-10-10-low-power-fix-design.md).
The selected boundary remains the macOS frame source and pinned renderer.
Installing over the user's running app and restarting personal sessions are
outside this verification run.

## Implementation

On macOS 14+, a per-window `CAMetalDisplayLink` delivers its supplied drawable to
the existing GPUI request-frame callback. `MacWindow::draw` consumes that drawable
at most once. An empty slot inside that callback cannot fall back to synchronous
acquisition. Zero or stale-size updates return before consuming GPUI's frame
request. Unused drawables are released on callback return.

The source owns its delegate, layer and link; its closure holds weak window state.
Visibility stops pause the link. Destruction clears the delegate and invalidates
exactly once. A callback clones its closure before invoking it so closing a
window from that callback cannot leave a reference into a destroyed delegate.

Effective Low Power Mode requests at most 60 Hz, bounded by the screen's maximum;
normal mode restores the system preference. The mode is checked at resume and
callback entry. Apple's [rate-range documentation](https://developer.apple.com/documentation/quartzcore/cametaldisplaylink/preferredframeraterange)
describes this as a preference, not a guaranteed rate. Removing acquisition from
the callback is the fix mechanism; the cap is a separate energy policy.

`gpui_apple` is copied from the existing Zed pin, with only its standalone manifest
and the `draw_with_drawable` extraction differing from upstream. No dependency
version changes or common GPUI trait changes are needed. The CoreVideo fallback
for macOS 11–13 and direct AppKit transaction draws are preserved.

## Verification checkpoint

`cargo check -p chartr --locked`, workspace formatting and whitespace checks pass.
The native check runs the actual production frame-source module in a disposable
AppKit window. On AC and then battery/Low Power Mode it passed unused-update,
submission-after-idle, pause, resize/resume, post-destruction callback and
destruction-inside-callback assertions. The small policy unit test also passed.
The release build and Chartr-level interaction/performance verification are
still in progress; these native-module results alone do not establish a passing
Chartr performance result.

The five existing Rust edits retain patch SHA-256
`6e346819aca02946a579074fcfd0ce66e7e68260116fc35db96084ed71da00bf`.
Earlier investigation evidence and outcomes remain intact.

The release build subsequently passed. The [verification run](../verification/runs/2026-10-11-metal-frame-delivery.md)
records exact build identity, native results and the current checkpoint. The
isolated app/backend are running with a fresh short-path profile; the desktop
was locked initially, preventing interaction checks.

## Activation boundary found during real-app verification

After the user unlocked the desktop, two complete active 30-second captures
contained zero `nextDrawable` samples, with positive samples in the Metal
callback. A third capture was cut short by an activation crash. The earlier
design's assumption that pausing allowed direct AppKit acquisition was wrong:
Apple raises `CAMetalLayerInvalidOperation` when a link is still attached.
The run preserves the crash and a small native API reproduction showing that
invalidation, rather than pause, makes direct acquisition legal again.

The correction adds a transaction-specific stop: take and destroy the Metal
source before the existing synchronous transaction draw, then let the existing
start method recreate it. Ordinary occlusion stops still pause, and normal frames
still use supplied drawables. This keeps AppKit synchronization and the original
fix boundary without a queue, worker renderer or permanent timer change. The
native regression example now alternates eight direct transactions with recreated
Metal callbacks. The corrected release builds; the
[recheck](../verification/runs/2026-10-11-metal-frame-delivery-recheck.md) records its
new identity and the remaining unlocked-desktop prerequisite.

## Completed recheck

After the user unlocked the desktop again, the corrected native test passed on
battery with effective Low Power Mode: all eight direct-transaction/recreation
cycles, idle/submission/resize phases, and pause/drop assertions succeeded.
Three complete 30-second captures of the actual corrected release build contained
**zero acquisition samples out of 12,539 main-thread samples**, with positive
Metal callback activity in every capture. Repeated sidebar scrolling and view
switches ran while a disposable terminal produced 50 Hz counter updates.

Activation, native popup focus loss, zoom/restore, minimize/restore, native window
close and a fresh process launch were exercised. Terminal markers confirmed
input/output after activation, restoration and relaunch. The selected rendering
scenarios pass; native input-to-presentation percentiles and broader hardware,
power-transition and sleep/wake coverage remain unverified. This is proof that
the measured blocking path was removed, not a certified latency percentile.

Two app lifecycle limitations were recorded without expanding the rendering
patch: Cmd+H did not visibly hide the app, and reopening the already-running
process after its last window closed did not create a window. The unchanged
entry point has no reopen handler. That windowless process sampled entirely in
ordinary event-loop wait; a new process restored the test session successfully.

The disposable shell, GUI and private backend are stopped and the bounded awake
assertion is removed. The installed app and its personal sessions remain on the
original binary. Existing source edits and installed binary hashes are unchanged.
The [recheck report](../verification/runs/2026-10-11-metal-frame-delivery-recheck.md)
retains raw samples, build identity, observations, limits and cleanup evidence;
the earlier failed run remains Failed.

## Commit-preparation reporting correction

The recheck was initially labeled Passed for a narrowed rendering scope, with
the exercised same-process reopen failure relegated to its limitations. The
repository workflow requires any exercised failure to make the aggregate Failed.
The report now lists that failed scenario and the unverified App Hide path
explicitly and has aggregate result **Failed**. All rendering measurements and
their individual passes remain unchanged. No new product changes or verification
runs were made as part of this reporting correction.

The implementation is committed as `20679d7d`; the pre-existing Close All
Sessions work is separate in `47a152e8`. All 32 recorded source hashes still
match the tested release. Staging exposed seven indentation warnings in the
newly vendored `shaders.metal`; that file is byte-for-byte identical to the pinned
upstream source and was preserved. The remaining staged implementation passed
the whitespace check. Formatting, wiki snapshot validation and report metadata
checks passed during commit preparation; native runtime checks were not rerun.
