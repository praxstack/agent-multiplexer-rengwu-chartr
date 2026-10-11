---
title: Metal frame delivery with AppKit transaction handoff
section: Verification
updated: 2026-10-11
result: Failed
---
# Metal frame delivery with AppKit transaction handoff

## Scope

Recheck the [workspace view](../../features/workspace-views.md) rendering change
after the [first implementation failed activation](2026-10-11-metal-frame-delivery.md).
The correction invalidates/removes the layer-bound Metal source before direct
AppKit transaction draws, then recreates it afterward. Visibility stops continue
to pause. Normal frames still receive supplied drawables without acquisition.

## Build and environment

- HEAD `45829e5f5e093bfa98f9a500ba887b6ff5c837ee`, dirty. Product patch SHA-256
  `6c616935f55e014434c4a3fa1d2ee3f761154fbf2d3a3fb309e0c645488264ea`.
- Corrected release binary SHA-256
  `c90d4967d8ddaed555c8dd2f3c060593012ee85722d887e0e0046e49bffa9a9b`.
  [Manifest and individual source hashes](../../evidence/2026-10-11-metal-frame-delivery-recheck/build-manifest.json),
  [tracked product patch](../../evidence/2026-10-11-metal-frame-delivery-recheck/build-product.patch.txt),
  [release build log](../../evidence/2026-10-11-metal-frame-delivery-recheck/cargo-release-build.log).
- Same Rust 1.96.0, arm64 macOS 27.0 and pinned Zed/Herdr dependencies as the
  first run. Battery power and effective Low Power Mode are recorded in the
  manifest. GUI PID 52063, private backend PID 47998; isolated bundle
  `dev.chartr.metal-verification`, version label `45829e5-metal-2`.
- Profile `/tmp/chartr-mtl.4b5cch8e` retains the 32 fixture spaces, one disposable
  shell and an empty surface from the first interaction pass. The user's normal
  app/backend and data remain separate. Backtraces are enabled for this test
  bundle only. After the user unlocked the desktop, a bounded
  `caffeinate -d -u -t 1800` assertion kept the verification session awake;
  no persistent power setting was changed.

## Results

| Scenario | Expected outcome | Observed outcome | Result | Evidence |
| --- | --- | --- | --- | --- |
| Corrected release build | New transaction handoff compiles with pinned renderer | Locked optimized build passed; formatting and whitespace checks passed | Passed | Release build log above |
| Native API boundary | Direct acquisition becomes legal after link invalidation | Objective-C reproduction catches CAMetalLayerInvalidOperation while paused; nextDrawable succeeds after invalidation, before and after releasing the link | Passed | [API output](../../evidence/2026-10-11-metal-frame-delivery/transaction-api.log), [source](../../evidence/2026-10-11-metal-frame-delivery/transaction-probe.m.txt) |
| Native lifecycle regression | Eight transaction/recreation cycles and existing lifecycle assertions pass | Effective Low Power Mode: 60 unused callbacks, 60 submissions after idle, 60 resize/resume callbacks, all eight transactions passed; zero callbacks while paused/after drop; exactly one callback when destroying its own source. Policy unit test passed. | Passed | [Native test output](../../evidence/2026-10-11-metal-frame-delivery-recheck/native-battery.log) |
| Three active Chartr captures | No nextDrawable beneath ordinary Metal callbacks while real input/output progresses | Zero acquisition samples in all three complete captures: 0/4119, 0/4207, 0/4213. Each had positive Metal callback activity. Sidebar scrolling, repeated Tabs/Spaces switches and a 50 Hz disposable terminal counter progressed. PID 52063 remained unchanged. | Passed | [Counts and power state](../../evidence/2026-10-11-metal-frame-delivery-recheck/active-samples.json), [preflight](../../evidence/2026-10-11-metal-frame-delivery-recheck/capture-preflight.json) |
| Activation, resizing and popup lifecycle | Reactivation and direct AppKit draws continue without the earlier crash | Zoom/restore rendered at both sizes; space popup rendered and dismissed on focus loss; terminal printed CHARTR_AFTER_ACTIVATION_OK after reactivation. Same process remained alive. | Passed | [UI observations](../../evidence/2026-10-11-metal-frame-delivery-recheck/ui-observations.json), [runtime stderr](../../evidence/2026-10-11-metal-frame-delivery-recheck/runtime-stderr.log) |
| Visibility, window close and fresh launch | Rendering resumes after minimize/restore, stops after close, and a new process restores the disposable session | Minimized and reopened the bundle through Finder; terminal printed CHARTR_AFTER_RESTORE_OK. After closing the window, all 533 main-thread samples were ordinary event-loop wait. A fresh launch of the same binary, PID 72781, restored the shell and printed CHARTR_AFTER_RELAUNCH_OK. | Passed | UI observations above; [closed-window sample](../../evidence/2026-10-11-metal-frame-delivery-recheck/after-close.sample.txt) |
| Last-window reopen in the same process | Opening the running app after closing its last window recreates a window | Opening the bundle did not recreate the window; a fresh process was required. The unchanged app entry point has no reopen handler. | Failed | UI observations above; source inspection of crates/chartr/src/main.rs |
| App Hide/show | Hiding and showing the app resumes rendering | Cmd+H had no visible effect and the app menu was empty; this path was not established. Minimize/restore passed separately. | Untested | UI observations above |

Overall **Failed** because the exercised last-window reopen scenario failed.
The rendering scenarios passed, and the direct acceptance criterion from the
design is satisfied: the normal frame path
no longer acquires drawables synchronously, including while actual Chartr input
and output progress on battery with Low Power Mode enabled. This result does not
certify an input-latency percentile or the full release matrix.

During commit preparation, the overall status was corrected from Passed to
Failed to follow the [repository's aggregation rule](../../development/workflow.md#keep-records-honest).
The initial report limited its pass to rendering and described the reopen failure
only as a separate limitation. That excluded an exercised failure from the
aggregate. The observations, raw evidence and individual rendering passes are
unchanged; the lifecycle issue is not attributed to the rendering patch.

## Active capture evidence

Each capture used `sample 52063 30 5` while the isolated release build displayed a
bounded Python counter, with repeated sidebar scrolling and view switches through
the native UI tool. Battery was 55% and effective Low Power Mode was true before
and after the captures. Native tests reported Low Power Mode as well.

| Capture | Main-thread samples | nextDrawable | Metal callback | Ordinary event-loop wait |
| --- | --- | --- | --- | --- |
| [1](../../evidence/2026-10-11-metal-frame-delivery-recheck/active-1.sample.txt) | 4119 | 0 | 818 | 3223 |
| [2](../../evidence/2026-10-11-metal-frame-delivery-recheck/active-2.sample.txt) | 4207 | 0 | 877 | 3249 |
| [3](../../evidence/2026-10-11-metal-frame-delivery-recheck/active-3.sample.txt) | 4213 | 0 | 909 | 3228 |

The original user-confirmed lag capture had 2732/3870 samples in acquisition.
The new workload is a controlled disposable fixture, not the identical personal
session, so these figures establish removal of the observed blocking call rather
than a numeric end-to-end speedup. Screenshots and AX states were inspected during
the run; the observation record describes them without claiming saved image files
or inferring latency from UI tool duration.

## Limits and follow-up

The first native recheck was interrupted when the desktop auto-locked. Its
[failed setup attempt](../../evidence/2026-10-11-metal-frame-delivery/native-recheck-locked.log)
and the subsequent [locked preflight](../../evidence/2026-10-11-metal-frame-delivery-recheck/locked-preflight.json)
remain preserved. The completed native run above supersedes neither that history
nor the first implementation's activation failure.

App-level Hide was not verified: Cmd+H had no visible effect and this test app's
application menu was empty. Minimize/restore exercised the visibility boundary.
Closing the last window and reopening the already-running app did not create a
new window. Source inspection found no `on_reopen` registration in unchanged
`crates/chartr/src/main.rs`; the platform callback is optional. The closed process
was idle, not blocked. This existing lifecycle gap is recorded separately from
the successful fresh-process relaunch and is not silently counted as a pass.

Native event-to-presentation percentiles (including the proposed p95 ≤50 ms
target), submitted-versus-presented frame counts in Chartr, live power-policy
transitions, sleep/wake, external monitors, Intel and older macOS runtime behavior
remain unverified. No independent review or subjective user confirmation of the
patched build is claimed. These are limits of this bounded result, not evidence
that every responsiveness requirement has been certified.

## Cleanup

The disposable shell was exited, both owned GUI processes were stopped, and
only the private Herdr namespace was stopped. The temporary display-awake
assertion was removed. The isolated bundle and profile are retained as local
artifacts; no diagnostic processes remain. The installed app/backend were not
restarted or replaced, and the five pre-existing Rust edits retain their original
fingerprint. [Cleanup command](../../evidence/2026-10-11-metal-frame-delivery-recheck/cleanup-command.json),
[final identity and isolation checks](../../evidence/2026-10-11-metal-frame-delivery-recheck/final-checks.json).
