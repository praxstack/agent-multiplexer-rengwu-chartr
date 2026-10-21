---
title: Version 0.3.2 release publication
section: Verification
updated: 2026-10-21
result: Passed
---
# Version 0.3.2 release publication

The user requested publication on October 21 after accepting the
[focused verification scope](../../decisions/index.md#dec-release-032-001-focused-verification-is-sufficient-for-0-3-2).
[Chartr v0.3.2](https://github.com/rengwu/chartr/releases/tag/v0.3.2) was published
as the latest stable release at 10:47:41 UTC. Tag builds, package inspection and
public download checks passed. AUR publication remains explicitly deferred;
its release-triggered workflow and job were skipped.

## Scope and identity

Affected feature: [distribution](../../features/distribution.md)
(`FEAT-distribution`). This run covers tagging, release packaging, downloaded
artifact inspection, draft review and public publication. It does not repeat
the deferred desktop acceptance matrix or launch a notification test app.

Main was fast-forwarded to the clean, prepared revision
`204e9a7985ac2944c1478b21602e6ca541226ae3`. Annotated tag `v0.3.2` points to that
commit; its tag object is `02eed3744f5f70da1758a5962ade6d88cb914416`.
Workspace version and tag both identify 0.3.2. Product code is unchanged from
verified candidate `4afea26f`; the intervening commits contain wiki records only.
The [manifest](../../evidence/2026-10-21-release-publication/manifest.json) records
source identity and inspection host. Evidence and this report were added after
tagging; the release tag stays unchanged.

GitHub runners built Apple silicon macOS, Ubuntu 24.04 x86_64 and Ubuntu 24.04
ARM64 packages. Inspection ran on the macOS arm64 development host. The macOS
DMG came from the exact tag workflow; Linux packages came from its draft release.
Every downloaded file matched the draft asset's GitHub SHA-256 digest, and all
asset identities/digests remained unchanged after publication. No personal
Chartr profile or running instance was controlled. The duplicate main-branch
package-cache build was cancelled; the tag's shipping builds passed independently.

## Results

| Scenario | Expected | Observed | Result | Evidence |
| --- | --- | --- | --- | --- |
| Source CI | Tagged source has passing CI | All CI jobs passed on the exact commit before tagging and again on the tag | Passed | [Source CI](https://github.com/rengwu/chartr/actions/runs/39217418048), [source receipt](../../evidence/2026-10-21-release-publication/source-ci.json), [tag CI](https://github.com/rengwu/chartr/actions/runs/39584417263), [tag receipt](../../evidence/2026-10-21-release-publication/tag-ci.json) |
| Tag packaging | Tag matches workspace version; all three platform jobs create a draft | Version checks, release scripts, Linux packages, real Arch/AUR recipe checks, macOS DMG and draft assembly passed | Passed | [Tag workflow](https://github.com/rengwu/chartr/actions/runs/39584417229), [receipt](../../evidence/2026-10-21-release-publication/tag-workflow.json) |
| Asset completeness and integrity | Seven payloads and three checksum files match | All ten names/sizes/GitHub SHA-256 digests matched; all seven payloads matched their checksum manifests | Passed | [Checksums](../../evidence/2026-10-21-release-publication/checksums.json) |
| Linux package identity | Correct version, commit, architecture, layout and shared binaries | Both tar/DEB pairs identify 0.3.2 and the tagged commit, contain the expected ELF architectures and relative launch symlink, and have identical Chartr/Herdr binaries; Arch binaries and build metadata match the x86_64 tarball; AUR metadata matches its version, URL and checksum | Passed | [Inspection](../../evidence/2026-10-21-release-publication/linux-inspection.json) |
| macOS package identity | Valid DMG and signature with correct app metadata | DMG verification and strict deep signature validation passed; production bundle ID, version 0.3.2, revision `204e9a7985ac`, ARM64 binaries and exact pinned Herdr version matched; signature is ad hoc | Passed | [Inspection](../../evidence/2026-10-21-release-publication/macos-inspection.json), [command log](../../evidence/2026-10-21-release-publication/macos-inspection-log.json) |
| Stable publication | Reviewed notes and verified assets become latest stable release | Release ID 419438751 is public, non-draft, non-prerelease and latest; all ten asset identities and digests are unchanged | Passed | [Publication receipt](../../evidence/2026-10-21-release-publication/published.json), [published notes](../../evidence/2026-10-21-release-publication/release-notes.txt) |
| Public downloads | Versioned links work without authentication | All ten asset URLs returned HTTP 200; all three public checksum files were downloaded and matched their verified digests | Passed | [Public checks](../../evidence/2026-10-21-release-publication/public-downloads.json) |
| AUR deferral | Publishing GitHub release does not publish AUR | Release-triggered Publish AUR workflow and its job were skipped | Passed | [Workflow](https://github.com/rengwu/chartr/actions/runs/39587187461), [receipt](../../evidence/2026-10-21-release-publication/aur-skipped.json) |

## Inspection and cleanup

The macOS image was mounted read-only without opening Chartr and detached after
inspection. Only the packaged Herdr `--version` command was executed. No desktop
fixture or personal backend was started. Downloads remain under
`target/release-publish-0.3.2/`. A slow ARM64 workflow-artifact transfer was stopped
once the draft became available; its complete release assets were downloaded and
verified instead. Draft API lookup used the release ID because the tag endpoint
did not expose the unpublished draft.

Executed drivers: [asset checks](../../evidence/2026-10-21-release-publication/verify-downloads.py.txt),
[Linux inspection](../../evidence/2026-10-21-release-publication/inspect-linux.py.txt),
[macOS inspection](../../evidence/2026-10-21-release-publication/inspect-macos.py.txt)
and [publication/public downloads](../../evidence/2026-10-21-release-publication/verify-publication.py.txt).

## Limits

The app remains ad-hoc signed and unnotarized, as stated in the prepared release
notes. The [accepted deferred checks](../../development/release-0.3.2.md#accepted-readiness-scope)
remain unverified. Packaging/publication checks do not certify them. Earlier
Failed and Untested reports remain intact.
