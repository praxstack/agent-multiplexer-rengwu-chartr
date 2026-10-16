---
title: Version 0.3.1 release publication
section: Verification
updated: 2026-10-16
result: Passed
---
# Version 0.3.1 release publication

[Chartr v0.3.1](https://github.com/rengwu/chartr/releases/tag/v0.3.1) was published
as the latest stable release on October 16 at 12:42:55 UTC. The user explicitly
requested publication after accepting the
[focused verification scope](../../decisions/index.md#dec-release-031-002-focused-verification-is-sufficient-for-0-3-1).
AUR publication remains deferred and its release-triggered job was skipped.

## Scope and identity

Affected feature: [distribution](../../features/distribution.md)
(`FEAT-distribution`). This run covers tagging, release packaging, downloaded
artifact inspection, draft review and public publication. It does not repeat
the deferred desktop acceptance matrix.

The clean, pushed source revision was
`eebe2b22767d6565e86f3e12bb6185c71999533a`. Annotated tag `v0.3.1` points to that
commit; its tag object is `01f0726b199b070a7192de98eb4b6a255d0f12f0`.
Workspace version and tag both identify 0.3.1. Evidence and this report were
added after publication; the published tag remains unchanged.

GitHub runners built Apple silicon macOS, Ubuntu 24.04 x86_64 and Ubuntu 24.04
ARM64 packages. Inspection ran on the macOS arm64 development host. Linux
assets were downloaded from the exact tag workflow; the DMG was downloaded
from its draft release. Every downloaded file was compared with GitHub's draft
asset digest, and all published asset identities/digests remained unchanged.

## Results

| Scenario | Expected | Observed | Result | Evidence |
| --- | --- | --- | --- | --- |
| Source CI | Tagged source has passing CI | All CI jobs passed on the exact commit | Passed | [CI](https://github.com/rengwu/chartr/actions/runs/38880883098), [receipt](../../evidence/2026-10-16-release-publication/source-ci.json) |
| Tag packaging | Tag matches workspace version; all three platform jobs create a draft | Version checks, release scripts, Linux packages, real Arch/AUR recipe checks, macOS DMG and draft assembly passed | Passed | [Tag workflow](https://github.com/rengwu/chartr/actions/runs/38900807140), [receipt](../../evidence/2026-10-16-release-publication/tag-workflow.json) |
| Asset completeness and integrity | Seven payloads and three checksum files match | All ten names/sizes/GitHub SHA-256 digests matched; all seven payloads matched the supplied checksum manifests | Passed | [Checksums](../../evidence/2026-10-16-release-publication/checksums.json) |
| Linux package identity | Correct version, commit, architecture, layout and shared binaries | Both tar/DEB pairs identify 0.3.1 and the tagged commit, contain the expected ELF architectures and relative launch symlink, and have identical Chartr/Herdr binaries; Arch matches the x86_64 tarball; AUR metadata matches its version, URL and checksum | Passed | [Inspection](../../evidence/2026-10-16-release-publication/linux-inspection.json) |
| macOS package identity | Valid DMG and signature with correct app metadata | DMG verification and strict deep signature validation passed; production bundle ID, version 0.3.1, revision `eebe2b22767d`, arm64 binaries and exact pinned Herdr version matched; signature is ad hoc | Passed | [Inspection](../../evidence/2026-10-16-release-publication/macos-inspection.json), [command log](../../evidence/2026-10-16-release-publication/macos-inspection-log.json) |
| Stable publication | Reviewed notes and verified assets become latest stable release | Release ID 414949646 is public, non-draft, non-prerelease and latest; all ten asset identities and digests are unchanged | Passed | [Publication receipt](../../evidence/2026-10-16-release-publication/published.json), [published notes](../../evidence/2026-10-16-release-publication/release-notes.txt) |
| Public downloads | Versioned links work without authentication | All ten asset URLs returned HTTP 200; all three public checksum files were downloaded and matched their verified digests | Passed | [Public checks](../../evidence/2026-10-16-release-publication/public-downloads.json) |
| AUR deferral | Publishing GitHub release does not publish AUR | Release-triggered Publish AUR workflow and its job were skipped | Passed | [Workflow](https://github.com/rengwu/chartr/actions/runs/38902477238), [receipt](../../evidence/2026-10-16-release-publication/aur-skipped.json) |

## Inspection corrections and limits

Temporary inspection assertions initially assumed a full macOS revision instead
of the packaging script's 12-character revision, a literal PKGBUILD URL instead
of its shell variables, and stable draft asset URLs across a notes edit. Those
inspection assumptions were corrected against the package/source format; asset
IDs, contents and digests were unchanged. Python's local certificate store could
not validate the public-download connection; system curl completed the checks
with normal TLS verification. These were verification-tool issues, not product
failures or changes to the release payloads.

The macOS image was mounted read-only without opening the app and detached after
inspection. Only the packaged Herdr `--version` command was executed. No personal
profile or running Chartr instance was used. Downloads remain under
`target/release-publish-0.3.1/`; no desktop fixture was started.

The app remains ad-hoc signed and unnotarized, as stated in the release notes.
The [accepted deferred checks](../../development/release-0.3.1.md#remaining-release-judgment)
remain unverified; this packaging/publication pass does not certify them. Earlier
Failed and Untested reports remain intact.
