---
title: Low Power Mode rendering investigation and plan
section: Journal
updated: 2026-10-10
status: Findings retained; preliminary plan superseded
---
# Low Power Mode rendering investigation and plan

The user subsequently required a concrete fix rather than this page's conditional
trial plan. The [implementation-ready design](2026-10-10-low-power-fix-design.md)
supersedes the recommendation below, selecting supplied-drawable frame delivery
and resolving lifecycle, dependency and compatibility details. Earlier evidence
and proposals are retained here as history.

The user requested evidence identifying the running build's lag, an explicit
goal, and a lean resolution plan, with **no fixes yet**. This follow-up ran on
battery with Low Power Mode enabled. It changes development records only.

## Finding

The confirmed failure is **synchronous Metal drawable acquisition blocking the
UI thread during rendering**. The same thread dispatches scrolling, view changes
and keyboard events. The earlier user-confirmed battery capture found 2732/3870
main-thread samples (70.59%) in `CAMetalLayer nextDrawable`; the AC capture found
659/4057 (16.24%), with the same app process and binary. The user reported lag in
the first capture and normal responsiveness in the second. See the
[original capture, identity and limitations](2026-10-10-running-build-lag.md#coordinated-battery-and-ac-reproduction).

This follow-up reproduced drawable backpressure in a tiny standalone Metal
program on the same Mac under the current power conditions. It requested frames
at about 120 Hz through the same CVDisplayLink/main-queue dispatch pattern, while
nonzero drawable presentation timestamps were about 60 Hz. The UI-thread draw
callback waited about 16.3 ms per drawable despite an average GPU clear time of
only 0.026 ms. Expensive application content is therefore **not necessary to
produce this API-level stall**. This does not establish Chartr's GPU execution
time or exact frame rate during its earlier capture.

The supported diagnosis is rendering backpressure on the input thread, exposed
by the battery/Low Power Mode presentation behavior. The exact contribution to
each user's input delay still needs end-to-end event/presentation timing in
Chartr. Neither a particular driver defect nor unbounded queued frames is proved.

## Evidence and reproducibility

The running app remained PID 1121, build 755, revision label `c09f327dbb9f`, with
the same binary SHA-256 as the earlier capture. The five pre-existing modified
Rust files have an unchanged combined patch fingerprint. The installed bundle
declares macOS 11.0 as its minimum. The Mac is running macOS 27.0, build 26A5368g.
[Manifest and hashes](../evidence/2026-10-10-low-power-frame-investigation/manifest.json).

`pmset` recorded battery power and the Low Power Mode policy; each Metal test
also read effective `NSProcessInfo.lowPowerModeEnabled == true` and nominal
thermal state. No power settings were changed.
[Power state](../evidence/2026-10-10-low-power-frame-investigation/battery-power-state.txt).

Each standalone test drew a solid background in a 320×180 point window for about
five seconds, using three drawables and vsync. These were diagnostic processes,
not modified Chartr builds. A per-frame autorelease pool and a bounded drawable
timeout were used; Chartr disables the timeout. No timeout occurred. Tests ran
sequentially. GPU timestamps and drawable presented handlers were added for the
later runs; the manifest distinguishes the two early runs whose exact original
source was not retained.

| Diagnostic variant | Submitted frames / elapsed time | Time inside `nextDrawable` / elapsed time | Interpretation |
| --- | --- | --- | --- |
| CVDisplayLink baseline, early run | 302 / 5.03 s | 97.60% | Reproduced sustained UI-thread waiting with trivial drawing. |
| CVDisplayLink baseline, with GPU timestamps | 302 / 5.02 s | 98.21% | 600 display ticks; mean drawable wait 16.32 ms; mean GPU work 0.026 ms. |
| CVDisplayLink, every second tick, early run | 307 / 5.12 s | 25.57% | A 60 Hz request cap reduced waiting but did not eliminate it. |
| CVDisplayLink, every second tick, repeat | 307 / 5.14 s | 0.20% | Strong improvement, with material variation between runs. |
| CVDisplayLink, discard ticks while a callback is pending or executing | 301 / 5.00 s | 75.81% | A pending-frame guard alone is insufficient in this probe. |
| CAMetalDisplayLink, system defaults | 624 / 5.25 s | No `nextDrawable` call | Callback supplies a drawable; roughly 60 Hz nonzero presentation timestamps despite about 120 callbacks/s. |
| CAMetalDisplayLink, preferred range 30–60 Hz | 306 / 5.11 s | No `nextDrawable` call | About 60 callbacks/s; mean CPU encoding/commit time 0.062 ms. |

These percentages are measured elapsed waiting in the diagnostic, **not the
stack-sample proportions in Chartr**. Submitted frames and display callbacks
are not necessarily displayed frames. Presented handlers can report zero time,
and some final callbacks are lost when the test exits. A supplied drawable's raw
`wait_ms` field measures only local assignment/timer overhead and is deliberately
excluded from acquisition-wait statistics.

[Derived statistics and method](../evidence/2026-10-10-low-power-frame-investigation/probe-summary.json),
[baseline raw records](../evidence/2026-10-10-low-power-frame-investigation/metal-baseline-gpu.json),
[60 Hz raw records](../evidence/2026-10-10-low-power-frame-investigation/metal-60hz-gpu.json),
[pending-guard raw records](../evidence/2026-10-10-low-power-frame-investigation/metal-gated-gpu.json),
[Metal-link default records](../evidence/2026-10-10-low-power-frame-investigation/metal-displaylink-default-gpu.json),
[Metal-link 60 Hz records](../evidence/2026-10-10-low-power-frame-investigation/metal-displaylink-60-gpu.json).

Source is retained as text attachments:
[CVDisplayLink probe](../evidence/2026-10-10-low-power-frame-investigation/metal_probe.m.txt),
[Metal-link default probe](../evidence/2026-10-10-low-power-frame-investigation/metal_displaylink_probe.m.txt),
[Metal-link 60 Hz probe](../evidence/2026-10-10-low-power-frame-investigation/metal_displaylink_60_probe.m.txt).
Copy a source to a temporary `.m` file and compile with `xcrun clang -fobjc-arc
-fblocks -Wno-deprecated-declarations`, linking AppKit, QuartzCore, Metal,
CoreVideo and CoreGraphics. The CV executable takes a tick divisor and pending
guard: `1 0` baseline, `2 0` half rate, `1 1` guard. Other variants take no
arguments. They open a small temporary window and exit automatically.

## Findings that limit the proposed fix

- Source review confirms the display dispatch source coalesces ticks. There is
  no evidence of an unbounded frame queue. A separate synthetic callback test
  with 10–35 ms of work still serviced a 20 ms CFRunLoop timer, with maximum
  heartbeat gaps under 48 ms. Slow callbacks alone did not demonstrate prolonged
  run-loop starvation. [Probe source](../evidence/2026-10-10-low-power-frame-investigation/frame_pacing_probe.m.txt)
  and [20 ms result](../evidence/2026-10-10-low-power-frame-investigation/probe-20ms.json).
- A plain `CADisplayLink` also delivered approximately 120 callbacks/s under Low
  Power Mode in a separate no-render test. Merely substituting that timer is not
  an evidenced fix. [Result](../evidence/2026-10-10-low-power-frame-investigation/cadisplay-probe.json).
- The pinned GPUI high-input-rate path continues presenting for one second,
  even when the scene is unchanged. This can sustain demand while scrolling.
  Its existing frame throttles address inactivity/thermal pressure, not an
  effective Low Power Mode limit. These are source observations, not measured
  attribution of individual delayed events.
- New Instruments/native-automation attempts did not capture a user-confirmed
  lag episode. One parallel sample spent 3450/3473 main-thread samples in normal
  event-loop waiting. A separate potential 322 ms microhang remains unattributed.
  These captures neither reproduce nor disprove the earlier failure.
  [Attempt record](../evidence/2026-10-10-low-power-frame-investigation/trace-attempts.json),
  [raw parallel sample](../evidence/2026-10-10-low-power-frame-investigation/battery-visible.sample.txt).
- The battery/AC comparison changed both power source and effective Low Power
  Mode. Battery with Low Power Mode off has not been measured. No end-to-end
  input latency or Chartr GPU timing has yet been recorded.

Apple's [nextDrawable contract](https://developer.apple.com/documentation/quartzcore/cametallayer/nextdrawable())
confirms that acquisition can wait on the pool. Its
[CAMetalDisplayLink API](https://developer.apple.com/documentation/quartzcore/cametaldisplaylink)
provides a drawable with each update. The installed SDK marks that API as
available from macOS 14.0; it cannot replace the existing path unconditionally
while the application still supports macOS 11.0.

## Concrete lean resolution plan — proposed, not implemented

1. **Measure the actual interaction in an isolated release build.** Add temporary
   timing/signposts for native event arrival and dispatch, frame-callback entry,
   drawable acquisition, CPU encoding, GPU completion and actual presentation.
   Repeat the same 30-second scroll/view-switch sequence three times on battery
   with Low Power Mode. Include terminal echo in a disposable shell. Establish
   input-to-visible-change latency; sample percentages alone cannot measure it.
2. **Try the smallest change first: power-aware request pacing.** Keep the current
   macOS CVDisplayLink lifetime and coalescing design. At the per-window frame
   delivery boundary, limit frame requests to at most 60/s while effective Low
   Power Mode is on, and never above the display's nominal rate. Observe live
   power-policy changes. Skip an early callback before entering GPUI so dirty
   state, forced renders and pending frame callbacks are retained. Cover the
   high-input-rate presentation path too; do not add a second frame queue or
   change sidebar, terminal or backend behavior. The standalone cap showed
   enough benefit to justify this small trial, but its variation prevents
   declaring it a fix in advance.
3. **If synchronous waiting still breaks the latency criteria, use a supplied
   drawable on macOS 14+.** Replace per-window frame delivery there with
   CAMetalDisplayLink and add a small `draw_with_drawable` entry to the pinned
   GPUI Apple renderer, reusing its current encoding/presentation implementation.
   Consume or release the update's drawable in that callback; do not accumulate
   stale drawables. Use an appropriate preferred range in Low Power Mode to
   avoid the observed extra submissions. Preserve the legacy CV path below
   macOS 14, demand/forced-redraw behavior, and safe window teardown. This option
   needs a narrow dependency patch; do not upgrade or fork unrelated GPUI code.
4. **Accept only on measured responsiveness and lifecycle checks.** Proposed
   targets for the controlled workload are p95 input-to-visible-change ≤50 ms,
   no recurring >100 ms event-dispatch stalls attributable to rendering, and
   no recurring multi-frame buffer waits. Run three trials each on battery with
   Low Power Mode on/off and on AC. Record exact power state, window/display,
   workload and build identity; compare CPU/energy and presented-frame counts.
   Exercise live power transitions, idle→active input, resize, occlusion,
   close/reopen, sleep/wake and external-display moves where available. Missing
   hardware scenarios remain untested, not passed. Preserve the existing
   CVDisplayLink lifetime/teardown regression coverage and run relevant macOS
   checks before broader release checks.

The first implementation target is the existing macOS frame-delivery boundary,
not a sidebar rewrite. The independently observed Herdr all-attachment rendering
cost remains a separate optimization; it is not needed to explain the isolated
Metal wait and is not bundled into this fix plan.

No production implementation, dependency update, app rebuild/replacement or
power-setting change was performed. Diagnostic processes have exited, their
windows were removed, and the temporary Finder window was closed. Chartr remains
running with Spaces selected. Documentation validation is separate from the
still-unexecuted product-fix verification.
