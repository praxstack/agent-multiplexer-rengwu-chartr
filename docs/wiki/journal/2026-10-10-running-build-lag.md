---
title: Running build lag investigation
section: Journal
updated: 2026-10-10
status: UI stall localized; fix pending
---
# Running build lag investigation

The user reported lag in the currently running Chartr. This was a read-only
investigation of that process, its private terminal backend and the pinned backend
source. No session was stopped, restarted, detached, or sent terminal input.
Related features: [terminals](../features/terminals.md) and
[workspace views](../features/workspace-views.md).

The later coordinated reproduction localized the visible lag to the macOS
rendering path: 70.59% of GUI main-thread samples were inside Metal drawable
acquisition on battery with Low Power Mode enabled, versus 16.24% on AC power,
where the user reported normal responsiveness. Earlier idle captures and the
separate backend overhead findings remain below as historical evidence.

The later [Low Power Mode follow-up](2026-10-10-low-power-frame-investigation.md)
adds controlled Metal API experiments and a concrete resolution plan. It narrows
the earlier scheduling hypothesis: a pending-frame guard alone did not remove
the wait, and prolonged input starvation from callback coalescing is not proved.

## Build identity

The running GUI was PID 1121, `/Applications/chartr.app/Contents/MacOS/chartr`,
version 0.3.0, build 755, with bundle revision label `c09f327dbb9f` and a
September 20 executable timestamp. It had been running since October 7 at
15:10 local time. It was not the current checkout's debug executable or the
October 8 sidebar/view commit.

The private backend was PID 1123, the installed bundle's `herdr`, version
`0.8.2-chartr.5a2dee700eee`, protocol 22. The current checkout pins this same
backend revision. Matching Mach-O UUIDs connected the sampled processes to the
on-disk binaries. [Identity and hashes](../evidence/2026-10-10-running-build-lag/identity.json)
record the executable identity separately from the development working tree;
the installed build's original dirty source state is unknown.

## Executed observations

- Repeated process measurements showed Herdr at roughly 47–57% CPU, about half
  of one CPU core. Chartr itself was around 0–0.7% during the observation.
- In the eight-second [GUI sample](../evidence/2026-10-10-running-build-lag/chartr.sample.txt),
  661 of 664 main-thread samples were waiting for events. This did not capture a
  GUI-thread stall or a busy render loop.
- The [backend sample](../evidence/2026-10-10-running-build-lag/herdr.sample.txt)
  placed 340 of 498 main-thread samples under `render_and_stream`, with substantial
  time in terminal cell extraction, Ratatui rendering, frame construction and
  comparison. These are stack-sample proportions, not input-latency measurements.
- A read-only [backend query](../evidence/2026-10-10-running-build-lag/backend-summary.json)
  returned 21 panes across 11 workspaces. The GUI had 21 PTY reader threads and
  corresponding live terminal-attach children.
- Across ten [activity snapshots](../evidence/2026-10-10-running-build-lag/activity-samples.json),
  only pane `wQ:p2` changed its reported revision. Its metadata identified the
  Codex conversation for this task in the Chartr project, with an Action Required
  title. This observation occurred during the investigation; it is not proof
  that this was the only source of work when the user first noticed lag.
- Twelve read-only [ping requests](../evidence/2026-10-10-running-build-lag/ping-latencies.json)
  took 0.23–0.32 ms. One `pane.list` request took 8.21 ms. The control socket was
  responsive in this interval; that does not establish terminal input latency.
- The 10-core, 16 GiB Mac had about 4.3 GiB of swap in use, but the short system
  samples showed 50–60% CPU idle and no new swap-outs. Cumulative swap use alone
  does not establish the cause of this lag.

## Sidebar and view-switching follow-up

The user clarified that sidebar scrolling was very slow and delayed, switching
views responded late, and terminal typing was affected less severely. This
narrows the next investigation to input dispatch and UI frame production;
backend CPU alone does not explain those observations.

Three additional GUI samples lasted 15, 30 and 60 seconds. The main thread was
waiting in its normal event loop for 2594/2601, 5185/5206 and 5356/5368 samples,
respectively. None captured a sustained render, input handler or blocking UI
operation. The user was asked to reproduce the lag during the capture, but had
not confirmed doing so. These are inconclusive captures, not a successful
reproduction or evidence that the reported lag is absent. The
[follow-up manifest](../evidence/2026-10-10-running-build-lag/ui-followup.json)
links and hashes the raw samples. Native UI inspection could not initialize
because its computer-use pipe failed to start.

Source review found a shared redraw path worth measuring during reproduction:

- GPUI's pinned `Div::paint_scroll_listener` notifies the containing view after
  a wheel offset changes. The sidebar is built directly in `WorkspaceWindow`.
  Its scrollbar thumb also notifies that parent via `notify_content()`.
- `WorkspaceWindow::render` rebuilds sidebar entries for every space, visits
  every terminal view to set its resize-pause flag, and measures title-bar
  controls. This is source-observed work, not a measured bottleneck. The
  resize-pause setter only notifies when its value changes.
- The render path also checks each registered folder with `is_dir`. Read-only
  checks of the 11 persisted folder paths were all successful and took 0.036 ms
  in total on the first pass; five checks per folder remained fast. The
  [timings](../evidence/2026-10-10-running-build-lag/folder-stat-latencies.json)
  do not support slow folder lookup as the cause in this interval.
- View changes update local state and notify; ordinary workspace persistence
  is coalesced and saved off the UI thread. No synchronous backend request was
  found in the direct Tabs/Spaces mode setter.

No product code, settings or running sessions were changed in this follow-up.

## Coordinated battery and AC reproduction

At 02:55:44 +0800 the user reproduced sidebar scrolling and Tabs/Spaces switching
for a 30-second GUI/backend capture, then confirmed the lag was present. The user
subsequently identified battery with Low Power Mode enabled as the triggering
condition and confirmed that the Mac was on battery during this capture, then
plugged in afterward.

At 02:57:44 a second 30-second GUI capture exercised the same kinds of interaction
on AC power. The user reported that it felt fast. `pmset` showed AC power and
Low Power Mode configured as enabled for battery and disabled for AC. Battery
state during the first capture is user-confirmed; the power query was made only
after that capture. The application process and binaries were unchanged.

| GUI main-thread observation | Battery + Low Power Mode | AC, Low Power Mode off |
| --- | --- | --- |
| Total samples | 3870 | 4057 |
| Inside `CAMetalLayer nextDrawable` | 2732 (70.59%) | 659 (16.24%) |
| Drawable semaphore wait | 2719 (70.26%) | 643 (15.85%) |
| Normal event-loop wait | 4 (0.10%) | 1562 (38.50%) |
| Inside the display-link frame callback | 3768 (97.36%) | 2297 (56.62%) |
| GUI CPU, mean of subsequent `top` intervals | 26.51% | 45.43% |
| Backend CPU, mean of subsequent `top` intervals | 59.42% | 50.75% |
| User outcome | Lag present | Fast |

These categories overlap: the drawable wait is inside the frame callback. The
percentages describe stack samples, not CPU utilization, frame rate, or measured
input latency. System CPU remained mostly idle in both runs. The comparison
changes both power source and effective Low Power Mode policy; battery with Low
Power Mode off was not separately tested. Manual interaction was not an identical
input replay.

Evidence:

- [Comparison manifest, counts and hashes](../evidence/2026-10-10-running-build-lag/power-comparison.json)
- [Battery GUI sample](../evidence/2026-10-10-running-build-lag/chartr-confirmed-reproduction.sample.txt)
- [AC GUI sample](../evidence/2026-10-10-running-build-lag/chartr-ac-reproduction.sample.txt)
- [Battery backend sample](../evidence/2026-10-10-running-build-lag/herdr-confirmed-reproduction.sample.txt)
- [Battery CPU intervals](../evidence/2026-10-10-running-build-lag/reproduction-top.txt),
  [AC CPU intervals](../evidence/2026-10-10-running-build-lag/ac-reproduction-top.txt),
  [observed AC power and configured policies](../evidence/2026-10-10-running-build-lag/ac-power-state.txt)

### Rendering mechanism and limits of the diagnosis

The sampled blocking stack is `gpui_macos::window::step` → platform draw →
`MetalRenderer::draw` → `CAMetalLayer nextDrawable` → semaphore wait. This blocks
the GUI thread that must also dispatch input. The paired captures strongly
associate the reported delay with drawable backpressure and lack of time back
in the normal event loop, rather than excessive CPU use by the sidebar itself.

The installed revision uses the same pinned GPUI renderer as this checkout:

- [Frame scheduling](https://github.com/rengwu/chartr/blob/c09f327dbb9f/vendor/zed-platform/gpui_macos/src/display_link.rs#L119)
  merges each CVDisplayLink tick into a dispatch source on the main queue. Ticks
  can arrive while its frame callback is still executing. The source coalesces
  ticks; this is not an unbounded queue of individual frames. Nevertheless,
  another callback can already be pending when a blocked draw returns, a
  plausible mechanism for starving input dispatch under sustained backpressure.
- [Drawable configuration](https://github.com/zed-industries/zed/blob/1ea16c1ab9dd6d36649e002dc60995634da04daf/crates/gpui_apple/src/metal_renderer.rs#L149)
  uses three drawables and disables the next-drawable timeout.
- [Drawing](https://github.com/zed-industries/zed/blob/1ea16c1ab9dd6d36649e002dc60995634da04daf/crates/gpui_apple/src/metal_renderer.rs#L447)
  acquires the next drawable synchronously before encoding and presenting.
- Apple documents that [drawable acquisition waits when the pool is exhausted](https://developer.apple.com/documentation/quartzcore/cametallayer/1478172-nextdrawable)
  and that [Low Power Mode can affect the available frame rates](https://developer.apple.com/documentation/quartzcore/cametaldisplaylink/preferredframeraterange).

The exact callback cadence, individual wait duration, and downstream reason for
slower drawable recycling were not measured. In particular, no specific refresh
rate or GPU saturation is established. The backend remained busy rendering in
the battery capture (1491/1882 backend main-thread samples under
`render_and_stream`), but its earlier all-attachments overhead is a separate
optimization candidate; the new evidence prioritizes UI frame scheduling.

### Accepted requirement and proposed fix direction

The user explicitly rejected this input delay as acceptable behavior in Low
Power Mode. Keep scrolling, view switching and typing responsive under reduced
power. Lower rendering throughput must not allow frame production to monopolize
the event loop.

Prioritize pacing rendering to drawable availability and discarding stale frame
requests while a frame is in flight, with input dispatch able to proceed. A
system-paced Metal display link is another implementation option. These are
proposals, not implemented fixes. Simply increasing drawable buffers, disabling
Low Power Mode, or enabling a one-second timeout would not establish that the
input starvation is fixed. Preserve the existing display-link lifetime and
teardown guarantees when changing this code.

## Source observations and inference

The exact pinned Herdr source explains the sampled rendering overhead:

1. [Render target selection](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/server/clients.rs#L482)
   includes every connected terminal-attach client, regardless of which Chartr
   tab is visible.
2. [The incremental rendering path](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/server/headless/retained_surface.rs#L213)
   falls back when any target is not a ClientShell. Chartr uses TerminalAttach.
3. [The complete renderer](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/server/headless/render.rs#L370)
   loops through all targets and constructs terminal frames before asking whether
   a frame matches the previously sent one.
4. [Terminal rendering](https://github.com/herdrdev/herdr/blob/5a2dee700eeeea68267a4d16777307632f77172f/src/server/render_stream.rs#L390)
   creates a new Ratatui backend and buffer, draws the terminal, then clones its
   resulting buffer on each call.

Consequently, one terminal's updates can cause full-frame work across all 21
attachments, including unchanged ones. This is a concrete performance issue
and a plausible contributor to lag as session count grows. The samples and
source agree on the expensive path. In the initial investigation, before the
coordinated power comparison above, no capture of the visible stall had been
performed. No backend-fix comparison has been performed. Merely rebuilding the current Chartr
checkout would still use this backend implementation.

## Remaining verification

The visible stall is now localized; an implementation and controlled fix
comparison remain outstanding. Measure frame-request cadence, drawable waits
and input latency, then validate responsiveness on battery with Low Power Mode
both on and off, on AC, and across live power-source changes. Use an isolated
build/profile for changes before replacing the user's running app. The current
checkout still uses the implicated frame scheduler and renderer, so a rebuild
alone is not a fix.

A separate backend fix candidate is to skip unchanged direct
terminal targets before constructing frames, retaining invalidation for resize,
reconnect, explicit repaint, cursor, scroll, and graphics changes. This is a
proposal, not an implemented or verified fix. Any controlled comparison should
use disposable sessions rather than closing the user's running terminals.
