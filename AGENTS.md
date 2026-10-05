# Chartr development

jstack is the advisory workflow for development planning, feature maps, and
verification records. Use the skills that fit the task; interviews and run reports
are not mandatory for every edit. Existing build, test, and release requirements
continue to apply.

Development records live in `docs/wiki/`; public guides remain outside that tree.
Start at [the wiki](docs/wiki/index.md) and follow its
[workflow](docs/wiki/development/workflow.md), [feature map](docs/wiki/features/index.md),
and [verification setup](docs/wiki/verification/setup.md).
When resuming relevant planning work, read the wiki home, decision register, open
questions, latest journal, and affected topic pages. Update the records after
substantive discussions, distinguishing accepted decisions from proposals and
source observations from executed verification.

Markdown is authoritative. After changing wiki content or attachments, run:

```sh
node docs/wiki/_build.mjs
node docs/wiki/_build.mjs --check
```

These commands validate document structure and snapshot freshness, not product
behavior. Keep historical evidence and earlier outcomes intact.

The original rewrite specification and dated audits are archived, not current
verification. Wayfinder's `.plan/maps/` product contract stays separate from the
jstack development records. Shared reader improvements belong in jstack; sync its
asset here without a chartr-specific fork. Resolve installed skill locations at
use time instead of assuming a sibling checkout. Refer to the wiki workflow for
upstream provenance, report format and reader update instructions.
