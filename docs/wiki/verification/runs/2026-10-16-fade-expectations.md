---
title: Preserve manually tuned header fade expectations
section: Verification
updated: 2026-10-16
result: Passed
---
# Preserve manually tuned header fade expectations

The user confirmed that the current fade values were deliberately hand-tuned and
requested changing the test and documentation. Production rendering is unchanged.

## Scope and build

Correct the stale expectations for [workspace views](../../features/workspace-views.md)
(`FEAT-workspace-views`) and [spaces](../../features/spaces.md) (`FEAT-spaces`):
16 px growth over 32 px of scrolling, with an opaque region of 30–60% of the header
height. Preserve geometry, clipping, paint order, header input and fitting-list checks.

Tested revision: `b34d0ba065ef7d3872297e8f2611e105cfdcd381` plus the
[test-only patch](../../evidence/2026-10-16-fade-expectations/test-expectation.patch.txt).
The [result manifest](../../evidence/2026-10-16-fade-expectations/workspace-result.json)
records its SHA-256 and exact command. macOS 27.0 arm64, pinned Rust 1.96.0;
production sources are identical to the earlier readiness assessment.

## Results

| Scenario | Expected | Observed | Result | Evidence |
| --- | --- | --- | --- | --- |
| Header fade | User tuning matches painted gradient; layout/input checks remain intact | Passed at two rem sizes and offsets 0/1/8/30/80/0 px | Passed | [Workspace log](../../evidence/2026-10-16-fade-expectations/workspace.log) |
| Workspace suite | All exercised tests pass | 428 passed, zero failed, seven ignored; exit 0 | Passed | [Log](../../evidence/2026-10-16-fade-expectations/workspace.log), [result](../../evidence/2026-10-16-fade-expectations/workspace-result.json) |

Formatting, wiki build/check and `git diff --check` also passed. Ignored tests
were not selected by this run; the three live backend tests already passed in the
[preceding readiness assessment](2026-10-16-release-readiness.md).

## Limits

This resolves the expectation mismatch. It does not replace the historical failed
assessment or establish native notification, cross-platform CI or package readiness.
The production fade has not been retuned or newly assessed visually in this run.
