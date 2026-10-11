---
title: Concrete fix design for Low Power Mode input lag
section: Journal
updated: 2026-10-10
status: Implementation-ready proposal; product code unchanged
---
# Concrete fix design for Low Power Mode input lag

The user rejected completion at a conditional “try a frame cap, then investigate
further” plan. This record supersedes that recommendation in the
[earlier investigation](2026-10-10-low-power-frame-investigation.md). The requested
deliverable is a concrete, supported way to fix the identified problem. Product
implementation is still explicitly out of scope.

## Recommended change

**On macOS 14 and later, feed the normal GPUI frame callback from
CAMetalDisplayLink, and pass its supplied drawable into the renderer. Do not call
`nextDrawable` from that callback.** Keep the existing CVDisplayLink implementation
for older macOS versions and preserve the separate AppKit transaction draw paths.

This removes the blocking acquisition operation from the exact call path seen
in the lag capture. It does not depend on a guessed refresh rate, a timeout, or a
frame cap leaving enough buffers available. A Low Power Mode preferred-rate cap
is an energy policy for the new callback, not the mechanism that prevents waiting.

## Why this is a concrete fix for the measured bottleneck

1. In the user-confirmed battery capture, **all 2732 samples in
   `CAMetalLayer nextDrawable` are under `gpui_macos::window::step`**: two branches
   contain 2319 and 413 samples. There is no sampled `display_layer` transaction
   path in that capture. These are counts in the retained
   [raw sample](../evidence/2026-10-10-running-build-lag/chartr-confirmed-reproduction.sample.txt),
   not a new input-latency measurement.
2. The pinned renderer's normal draw path does three things in sequence:
   acquire a drawable, encode into its texture, and present/commit. The
   acquisition is separable from the existing encoding code. Its transaction
   branch also waits until the command buffer is scheduled; that branch is
   preserved for AppKit and is not the measured normal-frame bottleneck.
3. In the standalone baseline, acquisition waited about **16.3 ms per frame**
   while the actual GPU clear averaged **0.026 ms**. With supplied drawables,
   that acquisition call is absent; the 60 Hz diagnostic encoded and committed
   in **0.062 ms on average**. These are API experiments, not a patched Chartr
   performance result. [Earlier measurements and exact source](2026-10-10-low-power-frame-investigation.md#evidence-and-reproducibility).
4. A follow-up supplied-drawable lifecycle experiment successfully dropped
   **122 unused updates**, resumed drawing, paused, resized and resumed, then
   invalidated. It submitted **379 frames** and received **zero callbacks after
   invalidation**. This supports the required demand/lifetime design rather
   than only demonstrating an always-drawing toy loop.

A final comparison sampled the **entire main thread**, including Apple's
framework code, to check that acquisition waiting was removed rather than hidden
outside the diagnostic callback. Each clear-only program ran for about five
seconds with a four-second stack sample; both reported effective Low Power Mode
and exited successfully:

| Main-thread observation | CV baseline | Supplied drawable, 60 Hz preference |
| --- | --- | --- |
| Total samples | 2650 | 2775 |
| In `CAMetalLayer nextDrawable` | 2568 (96.91%) | **0 (0%)** |
| Normal event-loop wait | 3 (0.11%) | **2610 (94.05%)** |

This directly demonstrates that the proposed API/pacing combination frees the
main thread in the diagnostic. It is stronger evidence than measuring only the
callback body, but remains a standalone comparison rather than a patched Chartr
or end-to-end input-latency result.

[Counts and conditions](../evidence/2026-10-10-low-power-fix-design/main-thread-comparison.json),
[baseline whole-thread sample](../evidence/2026-10-10-low-power-fix-design/baseline-main.sample.txt),
[supplied-drawable whole-thread sample](../evidence/2026-10-10-low-power-fix-design/supplied-main.sample.txt).

The last experiment also exposed an explicit teardown constraint: its first
version invalidated the same link once in a timed phase and again during final
cleanup. It crashed in `CA::Display::MetalLinkItem::invalidate` on this OS. Removing
the second invalidation produced exit 0 with all lifecycle phases intact. The
proposed wrapper therefore uses **pause/resume for temporary stops and exactly
one invalidation for destruction**, enforced by taking its owned link handle.

[Lifecycle data and phase counts](../evidence/2026-10-10-low-power-fix-design/summary.json),
[successful raw records](../evidence/2026-10-10-low-power-fix-design/metal-lifecycle-once.json),
[successful source](../evidence/2026-10-10-low-power-fix-design/metal_lifecycle_probe.m.txt),
[failed-run crash excerpt](../evidence/2026-10-10-low-power-fix-design/double-invalidate-crash-excerpt.json),
[evidence hashes](../evidence/2026-10-10-low-power-fix-design/manifest.json).

## Exact implementation boundary

| Owner | Required change | Existing behavior reused |
| --- | --- | --- |
| `vendor/zed-platform/gpui_macos/src/display_link.rs` and a small adjacent Metal-link module | Select a per-window Metal frame source when the runtime class is available; retain the CV source otherwise. Provide start/pause/drop operations. | Current CV registry, immortal-link and cancellation guarantees remain intact for the legacy branch. |
| `vendor/zed-platform/gpui_macos/src/window.rs` | Create the Metal link with `renderer.layer()`. Route its drawable through a temporary per-callback slot into `MacWindow::draw`. | The existing GPUI request-frame callback, dirty-state decisions, high-input-rate behavior, scene construction and rendering order. |
| A pinned local patch of `gpui_apple/src/metal_renderer.rs` | Extract `draw_with_drawable(&Scene, &MetalDrawableRef)` from the existing `draw` body. Legacy `draw` acquires a drawable and delegates; the Metal-link path calls the new entry directly. | `render_frame`, command encoding, sprite atlas, instance buffers, shaders and presentation branches. |
| Root `Cargo.toml`, lockfile and vendor provenance | Patch `gpui_apple` to a local copy of the existing pinned revision and expand its inherited manifest fields, following the existing platform-vendor convention. | The Zed revision remains `1ea16c1ab9dd6d36649e002dc60995634da04daf`; no framework upgrade or common GPUI fork. |

`gpui_apple` is currently a separate approximately 144 KiB source crate. Its
renderer already exposes the Metal layer and keeps `render_frame` private. A
narrow crate patch is therefore the clean integration seam; changing only the
timer cannot pass the supplied drawable through today's public renderer API.

No changes are needed to Chartr's sidebar implementation, workspace model,
terminal code, Herdr protocol, or the common `PlatformWindow` trait.

## Frame and ownership contract

The ordinary frame flow becomes:

```text
Metal update supplies drawable
  → upgrade weak window state; retain drawable for this callback only
  → reject zero/stale-size update before consuming GPUI dirty state
  → call the existing GPUI request-frame callback
      → if dirty: build the scene as today
      → if presentation is needed: draw_with_drawable(scene, drawable)
      → if no presentation is needed: submit nothing
  → release any unused supplied drawable before returning
```

Implement the slot as an explicit frame context: ordinary direct drawing versus
Metal-link drawing with at most one drawable. `MacWindow::draw` must **not silently
fall back to `nextDrawable` inside a Metal-link callback** if the slot is empty.
Take the drawable for the one presentation and clear the context on exit. This
prevents accidentally restoring the blocking path and prevents queued/stale
drawable ownership. The current GPUI frame callback calls `present` once at
most, after either drawing a dirty scene or deciding to present the retained one.

The callback must not hold `MacWindowState`'s mutex while invoking GPUI: follow
the existing `step` pattern of taking the callback, releasing the lock, invoking
it and restoring it. The Objective-C delegate retains a closure holding a weak
window-state reference; the source owns the delegate, layer and link. Upgrade
the weak state at callback entry. This avoids a window/delegate ownership cycle
and a dangling native-view context after close.

Use the supplied texture's dimensions for the renderer viewport. If a resize or
scale change means its dimensions no longer match the current intended drawable
size, discard that update before requesting GPUI's frame; the next update retries
without consuming dirty/forced work. Do not build a queue or copy the scene to a
worker thread.

## Lifecycle and compatibility choices

- Register the Metal link on the main run loop in common modes. Reuse the window's
  existing visibility/start/stop decisions. Stop means pause; start means update
  the preferred range and resume the same layer-bound link. An idle callback
  that GPUI does not need simply releases its drawable without presentation.
- Apply a preferred maximum of 60 Hz in effective Low Power Mode, bounded by the
  screen's maximum. Restore the normal preference outside Low Power Mode. Read
  the effective mode at start and at callbacks, and update the range only when
  it changes; no new process-wide policy framework is required. This avoids the
  diagnostic's approximately 120 submissions/s for approximately 60 actual
  presentations while leaving input processing independent of acquisition waits.
- `MacWindow::drop` already takes the frame source before closing the native
  window. The Metal source takes and invalidates its link **once**, clears its
  delegate and releases resources while the layer still exists. A later source
  drop must not invalidate it again. Temporary pause must never invalidate it.
- Keep `display_layer` and the subsequent-key-window activation path synchronous
  with `presents_with_transaction`, as today. They pause the frame source,
  perform their existing direct transaction draw, then resume. The new supplied
  path targets normal continuous rendering; it does not claim zero waits in
  AppKit transaction or screenshot/readback paths. Those paths have separate
  correctness requirements and are not in the measured blocking sample.
- Runtime-detect CAMetalDisplayLink on macOS 14+. Keep the current macOS 11
  minimum and unchanged CV fallback on macOS 11–13. **The demonstrated fix scope
  is modern macOS, including this user's macOS 27 machine.** Responsiveness on
  older systems is not certified by these experiments.

## Alternatives resolved

- **Do not lead with a 60 Hz CVDisplayLink cap.** It reduced waiting but varied
  between 0.2% and 25.6% of elapsed test time; synchronous acquisition remains.
  It is a mitigation, not the selected mechanism for removing the observed wait.
- **Do not add only a pending-frame guard.** It still waited for 75.8% of the
  diagnostic run; the current dispatch source already coalesces ticks.
- **Do not switch only to CADisplayLink.** That timer still called back at about
  120 Hz under the tested Low Power Mode conditions and supplies no drawable.
- **Do not claim an autorelease-pool fix.** An additional test compiled the
  render probe without ARC to approximate metal-rs' borrowed autoreleased
  objects. Mean waits were 16.40 ms without a per-frame pool and 16.30 ms with
  one; neither timed out. This does not explain every lifetime detail in Rust,
  but it provides no support for treating an added pool as the lag fix.
  [Source without the pool](../evidence/2026-10-10-low-power-fix-design/metal_no_pool_probe.m.txt)
  and [measured comparison](../evidence/2026-10-10-low-power-fix-design/summary.json).

## Verification after implementation is authorized

The diagnosis and implementation boundary are now concrete. Implementation and
app-level verification remain future work because the user requested no fixes.

1. Build an isolated release profile with the same terminal/backend workload.
   Repeat the user’s 30-second sidebar/view-switch interaction three times on
   battery with Low Power Mode. A main-thread stack trace must show **no
   `nextDrawable` call beneath normal Metal-link frame callbacks**. This is the
   direct acceptance criterion for eliminating the measured bottleneck.
2. Record native event timestamps, handler entry and actual drawable presentation
   timestamps. Existing GPUI profiler timing ends at `PlatformWindow::draw`
   return, not actual display presentation, so it cannot alone prove visible
   latency. Proposed interaction targets remain p95 ≤50 ms and no recurring
   rendering-related event-dispatch stalls >100 ms. Measure CPU and submitted
   versus presented frames as well.
3. Exercise idle→scroll, view switches, disposable terminal echo, resize/scale,
   activation/native tabs, popup windows, hide/show, close during activity and
   reopen. Verify direct AppKit transaction draws still synchronize correctly,
   unused drawables do not accumulate, callbacks stop after close, and no link
   is invalidated twice. Repeat across battery/AC and Low Power Mode transitions;
   test sleep/wake and external-display movement where hardware permits.
4. Run the relevant Rust/build checks and native scenarios. The earlier
   diagnostic success does not count as a passing Chartr build, and unsupported
   or unavailable compatibility scenarios remain explicitly untested.

No product source, app binary or power setting changed during this continuation.
All diagnostic programs have exited. The five existing Rust edits retain the
same patch fingerprint as before the investigation.
