---
title: Jstack migration implementation
section: Journal
updated: 2026-10-05
order: 3
status: Complete
---
# Jstack migration implementation

Implemented the [accepted scope](../discussions/jstack-migration.md) after the
user requested implementation. Development records now live in this wiki;
public guides remain outside it and Wayfinder retains its original product format.

## Result

- Moved 27 development/historical Markdown records and 16 research attachments.
  Short compatibility pointers remain at the old Markdown paths; internal callers
  now link directly to canonical content.
- Created 15 [feature records](../features/index.md) and a crosswalk for all 97
  legacy stories, plus shared setup and the advisory contributor workflow.
- Imported all seven ADRs, including the formerly unindexed ADR 0007, and retained
  the hosted-tier supersession. Archived the former workspace specification as
  history rather than keeping a second active behavior contract.
- Kept full dated research, screenshots, publication receipts and the inactive
  Companion protocol. Added a complete attachment index, including previously
  unlinked evidence files.
- Updated root/public navigation and developer-test pointers. Existing product
  CI, build and release scripts remain unchanged; no new process gate was added.
- Added simple inline local images to shared jstack and copied that asset here,
  with its licenses intact. Cross-boundary references use GitHub URLs.

## Validation

The [manifest](../evidence/2026-10-05-docs-migration/migration-manifest.json) records
original/destination paths, original hashes, chartr revision and dirty-file
fingerprints, and the exact shared reader hash. The
[validation summary](../evidence/2026-10-05-docs-migration/validation.json) records
check outcomes.

- All 32 jstack Node tests passed, including new image rendering, hash/freshness,
  invalid-path, remote-image, symlink and raw-HTML bypass cases.
- The installed plugin-creator validator accepted jstack's manifest; the installed
  skill-creator validator accepted all seven skill frontmatters.
- The wiki build and freshness/link check passed. A standalone copy of the wiki
  also built and checked without a sibling jstack checkout or installed packages.
- The first-party Markdown audit found no missing local targets/anchors or invalid
  current-repository source paths. All 97 legacy story numbers are mapped.
- All 16 moved attachment hashes match. All 16 archived historical Markdown bodies
  match their originals after allowing only rewritten link destinations and the
  explicit archival notices. Other moved developer pages retain their runbooks
  and rationale, with the recorded navigation/current-workflow updates.
- A [native browser observation](../evidence/2026-10-05-docs-migration/reader-browser-check.txt)
  confirmed a real historical screenshot renders inline with alt text, retained
  proportions and internal navigation when the reader is opened as a local file.
- The five pre-existing modified Rust files retain their exact original hashes.
  Wayfinder's tracker convention still matches its embedded manifest text.
  The copied reader matches the shared asset byte for byte.

## Limits

No chartr build, product test, live session, package/release action or app-code edit
was performed. Feature recipes remain **unvalidated**, and no synthetic product
run reports were created. Source observations do not establish release or platform
availability. Close All Sessions is an unverified dirty-tree addition; Cycle view
modes already exists in the recorded committed baseline. Earlier audit results
remain scoped to their original builds.

External source/research URLs were preserved or formed from repository paths and
were not checked for live availability. Public guide availability claims were not
recertified. Browser smoke coverage is one local PNG in Helium; broader cases are
covered structurally by tests, not claimed as cross-browser visual results.

Shared jstack changes are in its source checkout; no installed plugin cache was
modified, no commit was made and nothing was published. The reader works directly
from chartr's checkout, while skills resolve through the installed jstack collection
or its source checkout when explicitly used.
